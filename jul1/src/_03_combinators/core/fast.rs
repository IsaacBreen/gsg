
// src/_03_combinators/core/fast.rs
use crate::tokenizer::finite_automata::{Expr, Regex, RegexState};
use crate::tokenizer::{choice_fast, eat_bytestring_fast, eat_string_fast};
use crate::BaseCombinatorTrait;
use crate::*;
// src/combinators/fast.rs
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

pub struct FastCombinatorWrapper {
    pub(crate) regex: Regex,
}

pub struct FastParserWrapper<'a> {
    pub(crate) regex_state: RegexState<'a>,
    pub(crate) down_data: Option<DownData>,
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

impl DynCombinatorTrait for FastCombinatorWrapper {
    fn parse_dyn(&self, down_data: DownData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(down_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(down_data, bytes)
    }
}

impl CombinatorTrait for FastCombinatorWrapper {
    type Parser<'a> = FastParserWrapper<'a>;
    type Output = ();
    type PartialOutput = ();

    fn one_shot_parse(&self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        let mut regex_state = self.regex.init();
        regex_state.execute(bytes);
        if !regex_state.done() {
            Err(UnambiguousParseError::Incomplete)
        } else if let Some(new_match) = regex_state.prev_match() {
            let mut right_data = down_data.just_right_data();
            let position = new_match.position;
            #[cfg(debug_assertions)]
            if position == 0 {
                panic!("FastCombinatorWrapper::one_shot_parse: regex matched the empty string");
            }
            right_data.advance(position);
            Ok(OneShotUpData::new(right_data))
        } else {
            assert!(regex_state.failed());
            Err(UnambiguousParseError::Fail)
        }
    }

    fn old_parse(&self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let mut regex_state = self.regex.init();
        regex_state.execute(bytes);
        if regex_state.failed() {
            (FastParserWrapper { regex_state, down_data: None }, ParseResults::empty_finished())
        } else {
            let mut up_data_vec: VecY<UpData> = vecy![];
            let done = regex_state.done();
            if let Some(new_match) = regex_state.prev_match() {
                let mut right_data = down_data.clone().just_right_data();
                let position = new_match.position;
                #[cfg(debug_assertions)]
                if position == 0 {
                    panic!("FastCombinatorWrapper::old_parse: regex matched the empty string");
                }
                right_data.advance(position);
                up_data_vec.push(UpData::new(right_data));
            }
            (FastParserWrapper { regex_state, down_data: Some(down_data) }, ParseResults::new(up_data_vec, done))
        }
    }
}

impl BaseCombinatorTrait for FastCombinatorWrapper {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

impl ParserTrait for FastParserWrapper<'_> {
    fn get_u8set(&self) -> U8Set {
        profile!("FastParserWrapper.get_u8set", self.regex_state.get_u8set())
}

fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut regex_state = &mut self.regex_state;
        let prev_match = regex_state.prev_match();
        regex_state.execute(bytes);
        if regex_state.failed() {
            ParseResults::empty_finished()
        } else {
            let mut up_data_vec: VecY<UpData> = vecy![];
            let done = regex_state.done();
            if let Some(new_match) = regex_state.prev_match() {
                if Some(new_match) != prev_match {
                    let mut right_data = self.down_data.clone().unwrap().just_right_data();
                    let position = new_match.position;
                    if position == 0 {
                        panic!("FastParserWrapper::parse: regex matched the empty string");
                    }
                    right_data.advance(position);
                    up_data_vec.push(UpData::new(right_data));
                }
            }
            ParseResults::new(up_data_vec, done)
        }
    }
}

pub fn fast_combinator(expr: Expr) -> FastCombinatorWrapper {
    let regex = expr.build();
    FastCombinatorWrapper { regex }
}

pub fn eat_bytestring_choice(bytestrings: Vec<Vec<u8>>) -> FastCombinatorWrapper {
    // Convert into a regex.
    let mut children = vec![];
    for bytes in bytestrings {
        children.push(eat_bytestring_fast(bytes));
    }
    fast_combinator(choice_fast(children))
}

pub fn eat_string_choice(strings: &[&str]) -> FastCombinatorWrapper {
    let mut children = vec![];
    for s in strings {
        children.push(eat_string_fast(s));
    }
    fast_combinator(choice_fast(children))
}