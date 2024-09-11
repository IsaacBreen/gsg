
// src/_03_combinators/nullable/check_right_data.rs
// src/combinators/check_right_data.rs
use crate::{BaseCombinatorTrait, DynCombinatorTrait, ParserTrait, UnambiguousParseError, UnambiguousParseResults, RightData, UpData, OneShotUpData};
use crate::{CombinatorTrait, FailParser, ParseResultTrait, ParseResults, RightDataGetters};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

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
    fn parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Box<dyn ParserTrait<Self::Output> + 'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b {
        self.one_shot_parse(right_data, bytes)
    }
}

impl CombinatorTrait for CheckRightData {
    type Parser<'a> = FailParser;
    type Output = ();

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b {
        if (self.run)(&right_data.clone()) {
            Ok(OneShotUpData::new(right_data, ())) // Add output
        } else {
            Err(UnambiguousParseError::Fail)
        }
    }

    fn old_parse<'a, 'b>(&self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        if (self.run)(&right_data.clone()) {
            (FailParser, ParseResults::new_single(UpData::new(right_data, ()), true)) // Add output
        } else {
            (FailParser, ParseResults::empty_finished())
        }
    }
}

impl BaseCombinatorTrait for CheckRightData {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

pub fn check_right_data(run: impl Fn(&RightData) -> bool + 'static) -> CheckRightData {
    CheckRightData { run: Box::new(run) }
}

pub struct MutateRightData {
    pub(crate) run: Box<dyn Fn(&mut RightData) -> bool>,
}

impl Hash for MutateRightData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ptr = std::ptr::addr_of!(self.run) as *const ();
        std::ptr::hash(ptr, state);
    }
}

impl PartialEq for MutateRightData {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&self.run, &other.run)
    }
}

impl Eq for MutateRightData {}

impl Debug for MutateRightData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MutateRightData").finish()
    }
}

impl DynCombinatorTrait for MutateRightData {
    fn parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Box<dyn ParserTrait<Self::Output> + 'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b {
        self.one_shot_parse(right_data, bytes)
    }
}

impl CombinatorTrait for MutateRightData {
    type Parser<'a> = FailParser;
    type Output = ();

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b {
        let mut right_data = right_data;
        if (self.run)(&mut right_data) {
            Ok(OneShotUpData::new(right_data, ())) // Add output
        } else {
            Err(UnambiguousParseError::Fail)
        }
    }
    fn old_parse<'a, 'b>(&self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        let mut right_data = right_data;
        if (self.run)(&mut right_data) {
            (FailParser, ParseResults::new_single(UpData::new(right_data, ()), true)) // Add output
        } else {
            (FailParser, ParseResults::empty_finished())
        }
    }
}

impl BaseCombinatorTrait for MutateRightData {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

pub fn mutate_right_data(run: impl Fn(&mut RightData) -> bool + 'static) -> MutateRightData {
    MutateRightData { run: Box::new(run) }
}