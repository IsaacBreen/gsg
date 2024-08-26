// src/combinators/negative_lookahead.rs
use std::collections::{HashMap, HashSet};
use crate::*;
use crate::tokenizer::finite_automata::{Expr, ExprGroups, Regex, RegexState, Match};
use crate::{BaseCombinatorTrait, VecX};

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
                let mut regex_state = self.regex.init();
                regex_state.execute(&bytes[..(end_position - start_position)]);
                if regex_state.definitely_fully_matches() {
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
        let mut position_to_match = HashMap::new();
        let mut positions = Vec::new();
        for right_data in &parse_results.right_data_vec {
            let position = right_data.right_data_inner.fields1.position - start_position;
            position_to_match.insert(position, false);
            positions.push(position);
        }
        positions.sort();

        let mut current_position = 0;
        for position in positions {
            let slice = &bytes[current_position..position];
            regex_state.execute(slice);
            if regex_state.definitely_fully_matches() {
                position_to_match.insert(position, true);
            }
            current_position = position;
        }

        // Run the regex to the end of the input
        regex_state.execute(&bytes[current_position..]);
        if regex_state.definitely_fully_matches() {
            position_to_match.insert(bytes.len(), true);
        }

        parse_results.right_data_vec.retain(|right_data| {
            let position = right_data.right_data_inner.fields1.position - start_position;
            !position_to_match.get(&position).cloned().unwrap_or(false)
        });

        (ExcludeBytestringsParser {
            inner: Box::new(inner),
            regex_state,
            position: start_position + bytes.len(),
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

        // Optimized logic
        let mut position_to_match = HashMap::new();
        let mut positions = Vec::new();
        for right_data in &parse_results.right_data_vec {
            let position = right_data.right_data_inner.fields1.position - self.position;
            position_to_match.insert(position, false);
            positions.push(position);
        }
        positions.sort();

        let mut current_position = 0;
        for position in positions {
            let slice = &bytes[current_position..position];
            self.regex_state.execute(slice);
            if self.regex_state.definitely_fully_matches() {
                position_to_match.insert(position, true);
            }
            current_position = position;
        }

        // Run the regex to the end of the input
        self.regex_state.execute(&bytes[current_position..]);
        if self.regex_state.definitely_fully_matches() {
            position_to_match.insert(bytes.len(), true);
        }

        parse_results.right_data_vec.retain(|right_data| {
            let position = right_data.right_data_inner.fields1.position - self.position;
            !position_to_match.get(&position).cloned().unwrap_or(false)
        });

        self.position += bytes.len();
        parse_results
    }
}

pub fn exclude_strings(inner: impl IntoCombinator + 'static, bytestrings_to_exclude: Vec<&str>)-> impl CombinatorTrait {
    let bytestrings_to_exclude: Vec<Vec<u8>> = bytestrings_to_exclude.iter().map(|s| s.as_bytes().to_vec()).collect();
    let expr = Expr::Choice(bytestrings_to_exclude.into_iter().map(|bytes| Expr::Seq(bytes.into_iter().map(|b| Expr::U8(b)).collect())).collect());
    let regex = expr.build();

    ExcludeBytestrings {
        inner: Box::new(Box::new(inner.into_combinator())),
        regex,
    }
}