// src/combinators/fast.rs
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use crate::*;
use crate::tokenizer::finite_automata::{Expr, ExprGroups, Regex, RegexState};

pub struct FastCombinatorWrapper {
    pub(crate) regex: Regex,
}

pub struct FastParserWrapper<'a> {
    pub(crate) regex_state: RegexState<'a>,
    pub(crate) right_data: Option<RightData>,
}

impl Debug for FastCombinatorWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FastCombinatorWrapper").finish_non_exhaustive()
    }
}

impl Debug for FastParserWrapper<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FastParserWrapper").finish_non_exhaustive()
    }
}

impl CombinatorTrait for FastCombinatorWrapper {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        let mut regex_state = self.regex.init();
        regex_state.execute(bytes);
        if !regex_state.done() {
            Err(UnambiguousParseError::Incomplete)
        } else if let Some(new_match) = regex_state.prev_match() {
            let mut new_right_data = right_data.clone();
            let position = new_match.position;
            new_right_data.advance(position);
            Ok(new_right_data)
        } else {
            assert!(regex_state.failed());
            Err(UnambiguousParseError::Fail)
        }
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let mut regex_state = self.regex.init();
        regex_state.execute(bytes);
        if regex_state.failed() {
            (Parser::FailParser(FailParser), ParseResults::empty_finished())
        } else {
            let mut right_data_vec: VecY<RightData> = vecy![];
            let done = regex_state.done();
            if let Some(new_match) = regex_state.prev_match() {
                let mut new_right_data = right_data.clone();
                let position = new_match.position;
                new_right_data.advance(position);
                right_data_vec.push(new_right_data);
            }
            (Parser::FastParserWrapper(FastParserWrapper { regex_state, right_data: Some(right_data) }), ParseResults::new(right_data_vec, done))
        }
    }
}

impl ParserTrait for FastParserWrapper<'_> {
    fn get_u8set(&self) -> U8Set {
        self.regex_state.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut regex_state = &mut self.regex_state;
        let prev_match = regex_state.prev_match();
        regex_state.execute(bytes);
        if regex_state.failed() {
            ParseResults::empty_finished()
        } else {
            let mut right_data_vec: VecY<RightData> = vecy![];
            let done = regex_state.done();
            if let Some(new_match) = regex_state.prev_match() {
                if Some(new_match) != prev_match {
                    let mut new_right_data = self.right_data.clone().unwrap();
                    let position = new_match.position;
                    new_right_data.advance(position);
                    right_data_vec.push(new_right_data);
                }
            }
            ParseResults::new(right_data_vec, done)
        }
    }
}

pub fn fast_combinator(expr: Expr) -> FastCombinatorWrapper {
    let regex = expr.build();
    FastCombinatorWrapper { regex }
}