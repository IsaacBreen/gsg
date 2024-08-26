// src/combinators/negative_lookahead.rs
use std::collections::HashSet;
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
        if bytes.len() == 19680 {
            println!("hi");
        }
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
        let matches = regex_state.find_matches(bytes); // Use find_matches
        let indices: HashSet<usize> = matches.iter().map(|m| m.position).collect();

        // Retain only results that don't coincide with the indices
        let start_position = right_data.right_data_inner.fields1.position;
        parse_results.right_data_vec.retain(|right_data| {
            !indices.contains(&(right_data.right_data_inner.fields1.position - start_position))
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
        // TODO: we should be able to make this faster by getting all positions in parse_results, partitioning the input by them,
        //  and executing the regex on each partition to see if that partition takes us to a terminal state.
        let mut parse_results = self.inner.as_mut().parse(bytes);

        // Use the renamed find_matches function
        let matches = self.regex_state.find_matches(bytes);
        let indices: HashSet<usize> = matches.iter().map(|m| m.position).collect();

        parse_results.right_data_vec.retain(|right_data| {
            !indices.contains(&(right_data.right_data_inner.fields1.position - self.start_position))
        });

        self.start_position += bytes.len();
        parse_results
    }
}

pub fn exclude_strings(inner: impl IntoCombinator + 'static, bytestrings_to_exclude: Vec<&str>)-> impl CombinatorTrait {
    let bytestrings_to_exclude: Vec<Vec<u8>> = bytestrings_to_exclude.iter().map(|s| s.as_bytes().to_vec()).collect();
    // Convert the bytestrings to exclude into a regex
    let expr = Expr::Choice(bytestrings_to_exclude.into_iter().map(|bytes| Expr::Seq(bytes.into_iter().map(|b| Expr::U8(b)).collect())).collect());
    let regex = expr.build();

    ExcludeBytestrings {
        inner: Box::new(Box::new(inner.into_combinator())),
        regex,
    }
}