// src/combinators/negative_lookahead.rs
use std::collections::HashMap;
use crate::*;
use crate::{BaseCombinatorTrait, VecX};
use crate::tokenizer::finite_automata::{Expr, Regex, RegexState};

#[derive(Debug)]
pub struct ExcludeBytestrings<T: CombinatorTrait> {
    pub(crate) inner: Box<T>,
    pub(crate) regex: Regex,
}

#[derive(Debug)]
pub struct ExcludeBytestringsParser<'a> {
    pub(crate) inner: Box<dyn ParserTrait + 'a>,
    pub(crate) regex_state: RegexState<'a>,
    pub(crate) position: usize,
}

impl<T: CombinatorTrait + 'static> DynCombinatorTrait for ExcludeBytestrings<T> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<T: CombinatorTrait + 'static> CombinatorTrait for ExcludeBytestrings<T> {
    type Parser<'a> = ExcludeBytestringsParser<'a>;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        let start_position = right_data.right_data_inner.fields1.position;
        match self.inner.one_shot_parse(right_data, bytes) {
            Ok(right_data) => {
                let end_position = right_data.right_data_inner.fields1.position;
                let offset = end_position - start_position;
                if self.regex.definitely_fully_matches(&bytes[..offset]) {
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
        let mut current_position = right_data.right_data_inner.fields1.position;

        // 1. Collect end positions from the inner parse results
        let mut end_positions: HashMap<usize, bool> = HashMap::new();
        for right_data in &parse_results.right_data_vec {
            end_positions.insert(right_data.right_data_inner.fields1.position, false);
        }

        // 2. Run the regex incrementally and populate the map
        let mut regex_state = self.regex.init();
        let mut current_offset = 0;
        let mut sorted_positions: Vec<usize> = end_positions.keys().cloned().collect();
        sorted_positions.sort();
        for position in sorted_positions {
            let new_offset = position - current_position;
            let slice = &bytes[current_offset..new_offset];
            regex_state.execute(slice);
            if regex_state.definitely_fully_matches() {
                end_positions.insert(position, true);
            }
            current_offset = new_offset;
        }
        // Run the regex to the end of the input
        regex_state.execute(&bytes[current_offset..]);

        // 3. Retain results based on the populated map
        parse_results.right_data_vec.retain(|right_data| {
            let end_position = right_data.right_data_inner.fields1.position;
            !end_positions.get(&end_position).cloned().unwrap_or(false)
        });
        
        current_position += bytes.len();

        (ExcludeBytestringsParser {
            inner: Box::new(inner),
            regex_state: self.regex.init(),
            position: current_position,
        }, parse_results)
    }
}

impl<T: CombinatorTrait + 'static> BaseCombinatorTrait for ExcludeBytestrings<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        f(&self.inner);
    }
}

impl ParserTrait for ExcludeBytestringsParser<'_> {
    fn get_u8set(&self) -> U8Set {
        self.inner.as_ref().get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut parse_results = self.inner.as_mut().parse(bytes);

        // 1. Collect end positions from the inner parse results
        let mut end_positions: HashMap<usize, bool> = HashMap::new();
        for right_data in &parse_results.right_data_vec {
            end_positions.insert(right_data.right_data_inner.fields1.position, false);
        }

        // 2. Run the regex incrementally and populate the map
        let mut current_offset = 0;
        let mut sorted_positions: Vec<usize> = end_positions.keys().cloned().collect();
        sorted_positions.sort();
        for position in sorted_positions {
            let new_offset = position - self.position;
            let slice = &bytes[current_offset..new_offset];
            self.regex_state.execute(slice);
            if self.regex_state.definitely_fully_matches() {
                end_positions.insert(position, true);
            }
            current_offset = new_offset;
        }
        // Run the regex to the end of the input
        self.regex_state.execute(&bytes[current_offset..]);

        // 3. Retain results based on the populated map
        parse_results.right_data_vec.retain(|right_data| {
            let end_position = right_data.right_data_inner.fields1.position;
            !end_positions.get(&end_position).cloned().unwrap_or(false)
        });

        self.position += bytes.len();

        parse_results
    }
}

pub fn exclude_strings(inner: impl IntoCombinator + 'static, bytestrings_to_exclude: Vec<&str>)-> impl CombinatorTrait {
    let expr = Expr::Choice(bytestrings_to_exclude.iter().map(|s| Expr::Seq(s.bytes().map(|c| Expr::U8(c)).collect())).collect());
    let regex = expr.build();
    ExcludeBytestrings {
        inner: Box::new(Box::new(inner.into_combinator())),
        regex,
    }
}