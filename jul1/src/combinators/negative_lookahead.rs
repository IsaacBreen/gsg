use crate::tokenizer::finite_automata::{Expr, Regex, RegexState};
use crate::*;
use crate::BaseCombinatorTrait;
// src/combinators/negative_lookahead.rs
use std::collections::HashMap;

#[derive(Debug)]
pub struct ExcludeBytestrings<T: CombinatorTrait> {
    pub(crate) inner: T,
    pub(crate) regex: Regex,
}

#[derive(Debug)]
pub struct ExcludeBytestringsParser<'a, T: CombinatorTrait + 'a> {
    pub(crate) inner: T::Parser<'a>,
    pub(crate) regex_state: RegexState<'a>,
    pub(crate) position: usize,
}

impl<T: CombinatorTrait> DynCombinatorTrait for ExcludeBytestrings<T> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<T: CombinatorTrait> CombinatorTrait for ExcludeBytestrings<T> {
    type Parser<'a> = ExcludeBytestringsParser<'a, T> where T: 'a;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        let start_position = right_data.right_data_inner.fields1.position;
        match self.inner.one_shot_parse(right_data, bytes) {
            Ok(right_data) => {
                let end_position = right_data.right_data_inner.fields1.position;
                let mut regex_state = self.regex.init();
                regex_state.execute(&bytes[..(end_position - start_position)]);
                if regex_state.fully_matches_here() {
                    return Err(UnambiguousParseError::Fail);
                } else {
                    Ok(right_data)
                }
            },
            Err(err) => Err(err),
        }
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let (inner, mut parse_results) = self.inner.parse(right_data.clone(), bytes);
        let mut regex_state = self.regex.init();
        let start_position = right_data.right_data_inner.fields1.position;

        // Optimized logic
        let mut end_offsets_to_match = HashMap::new();
        let mut end_offsets = Vec::new();
        for right_data in &parse_results.right_data_vec {
            let end_offset = right_data.right_data_inner.fields1.position - start_position;
            end_offsets_to_match.insert(end_offset, false);
            end_offsets.push(end_offset);
        }
        end_offsets.sort();

        let mut offset = 0;
        for end_offset in end_offsets {
            let slice = &bytes[offset..end_offset];
            regex_state.execute(slice);
            if regex_state.definitely_fully_matches() {
                end_offsets_to_match.insert(end_offset, true);
            }
            offset = end_offset;
        }

        // Run the regex to the end of the input
        regex_state.execute(&bytes[offset..]);

        parse_results.right_data_vec.retain(|right_data| {
            let position = right_data.right_data_inner.fields1.position - start_position;
            !end_offsets_to_match[&position]
        });

        (ExcludeBytestringsParser {
            inner,
            regex_state,
            position: start_position + bytes.len(),
        }, parse_results)
    }
}

impl<T: CombinatorTrait> BaseCombinatorTrait for ExcludeBytestrings<T> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }

    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        f(&self.inner);
    }
}

impl<T: CombinatorTrait> ParserTrait for ExcludeBytestringsParser<'_, T> {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut parse_results = self.inner.parse(bytes);

        // Optimized logic
        let mut end_offsets_to_match = HashMap::new();
        let mut end_offsets = Vec::new();
        for right_data in &parse_results.right_data_vec {
            let end_offset = right_data.right_data_inner.fields1.position - self.position;
            end_offsets_to_match.insert(end_offset, false);
            end_offsets.push(end_offset);
        }
        end_offsets.sort();

        let mut current_offset = 0;
        for end_offset in end_offsets {
            let slice = &bytes[current_offset..end_offset];
            self.regex_state.execute(slice);
            if self.regex_state.fully_matches_here() {
                end_offsets_to_match.insert(end_offset, true);
            }
            current_offset = end_offset;
        }

        // Run the regex to the end of the input
        self.regex_state.execute(&bytes[current_offset..]);

        parse_results.right_data_vec.retain(|right_data| {
            let position = right_data.right_data_inner.fields1.position - self.position;
            !end_offsets_to_match[&position]
        });

        self.position += bytes.len();
        parse_results
    }
}

pub fn exclude_strings<T: IntoCombinator + 'static>(inner: T, bytestrings_to_exclude: Vec<&str>)-> impl CombinatorTrait {
    let bytestrings_to_exclude: Vec<Vec<u8>> = bytestrings_to_exclude.iter().map(|s| s.as_bytes().to_vec()).collect();
    let expr = Expr::Choice(bytestrings_to_exclude.into_iter().map(|bytes| Expr::Seq(bytes.into_iter().map(|b| Expr::U8(b)).collect())).collect());
    let regex = expr.build();

    ExcludeBytestrings {
        inner: inner.into_combinator(),
        regex,
    }
}