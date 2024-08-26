// src/combinators/negative_lookahead.rs
use std::collections::HashSet;
use crate::*;
use crate::tokenizer::finite_automata::{Expr, ExprGroups, Regex, RegexState};
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
    pub(crate) start_position: usize,
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
                let matched_bytes = &bytes[..end_position - start_position];
                let mut regex_state = self.regex.init();
                regex_state.execute(matched_bytes);
                if regex_state.done() && regex_state.prev_match().is_some() {
                    Err(UnambiguousParseError::Fail)
                } else {
                    Ok(right_data)
                }
            }
            Err(err) => Err(err),
        }
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let (inner, mut parse_results) = self.inner.parse(right_data.clone(), bytes);
        let start_position = right_data.right_data_inner.fields1.position;
        let regex_state = self.regex.init();

        parse_results.right_data_vec.retain(|right_data| {
            let end_position = right_data.right_data_inner.fields1.position;
            let matched_bytes = &bytes[..end_position - start_position];
            let mut regex_state = regex_state.clone();
            regex_state.execute(matched_bytes);
            !(regex_state.done() && regex_state.prev_match().is_some())
        });

        (ExcludeBytestringsParser {
            inner: Box::new(inner),
            regex_state,
            start_position,
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
        let mut regex_state = self.regex_state.clone();

        parse_results.right_data_vec.retain(|right_data| {
            let end_position = right_data.right_data_inner.fields1.position;
            let matched_bytes = &bytes[..end_position - self.start_position];
            regex_state.execute(matched_bytes);
            let result = !(regex_state.done() && regex_state.prev_match().is_some());
            regex_state = self.regex_state.clone(); // Reset the regex state
            result
        });

        self.start_position += bytes.len();
        parse_results
    }
}

pub fn exclude_strings(inner: impl IntoCombinator + 'static, bytestrings_to_exclude: Vec<&str>)-> impl CombinatorTrait {
    // Convert into a regex.
    let mut children = vec![];
    for bytes in bytestrings_to_exclude {
        let exprs: Vec<Expr> = bytes.bytes().map(|b| Expr::U8(b)).collect();
        children.push(Expr::Seq(exprs));
    }
    let regex = Expr::Choice(children).build();

    ExcludeBytestrings {
        inner: Box::new(Box::new(inner.into_combinator())),
        regex,
    }
}