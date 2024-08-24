// src/combinators/check_right_data.rs
use crate::{dumb_one_shot_parse, BaseCombinatorTrait, DynCombinatorTrait, ParserTrait, UnambiguousParseError, UnambiguousParseResults};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{CombinatorTrait, FailParser, ParseResults, RightData, ParseResultTrait};

pub struct CheckRightData {
    pub(crate) run: Box<dyn Fn(&RightData) -> bool>,
}

impl Hash for CheckRightData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ptr = std::ptr::addr_of!(self.run) as *const ();
        ptr.hash(state);
    }
}

impl PartialEq for CheckRightData {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&self.run, &other.run)
    }
}

impl Eq for CheckRightData {}

impl Debug for CheckRightData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CheckRightData").finish()
    }
}

impl DynCombinatorTrait for CheckRightData {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(right_data, bytes)
    }
}

impl CombinatorTrait for CheckRightData {
    type Parser<'a> = FailParser;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        if (self.run)(&right_data) {
            Ok(right_data)
        } else {
            Err(UnambiguousParseError::Fail)
        }
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        if (self.run)(&right_data) {
            (FailParser, ParseResults::new_single(right_data, true))
        } else {
            (FailParser, ParseResults::empty_finished())
        }
    }
}

impl BaseCombinatorTrait for CheckRightData {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub fn check_right_data(run: impl Fn(&RightData) -> bool + 'static) -> CheckRightData {
    CheckRightData { run: Box::new(run) }
}

// impl From<CheckRightData> for Combinator {
//     fn from(value: CheckRightData) -> Self {
//         Combinator::CheckRightData(value)
//     }
// 