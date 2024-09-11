
// src/_03_combinators/nullable/eps_and_fail.rs
use crate::_01_parse_state::{ParseResultTrait, RightData, RightDataGetters, UpData, OneShotUpData};
use crate::{BaseCombinatorTrait, DynCombinatorTrait, UnambiguousParseError, UnambiguousParseResults};
use crate::{CombinatorTrait, ParseResults, ParserTrait, U8Set};
#[derive(Debug)]
pub struct Eps;

#[derive(Debug)]
pub struct EpsParser;

impl DynCombinatorTrait for Eps {
    fn parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Box<dyn ParserTrait<Self::Output> + 'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b {
        self.one_shot_parse(right_data, bytes)
    }
}

impl CombinatorTrait for Eps {
    type Parser<'a> = EpsParser;
    type Output = ();

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b {
        Ok(OneShotUpData::new(right_data, ())) // Add output
    }

    fn old_parse<'a, 'b>(&self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        (EpsParser, ParseResults::new_single(UpData::new(right_data, ()), true)) // Add output
    }
}

impl BaseCombinatorTrait for Eps {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

impl<Output: OutputTrait> ParserTrait<Output> for EpsParser {
    fn get_u8set(&self) -> U8Set {
        panic!("EpsParser.get_u8set() called")
    }

    fn parse<'b>(&mut self, bytes: &'b [u8]) -> ParseResults<Output> where Output: 'b {
        panic!("EpsParser already consumed")
    }
}

pub fn eps() -> Eps {
    Eps
}


    
#[repr(transparent)]
#[derive(Debug)]
pub struct FailParser;

#[derive(Debug)]
pub struct Fail;

impl DynCombinatorTrait for Fail {
    fn parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Box<dyn ParserTrait<Self::Output> + 'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b {
        self.one_shot_parse(right_data, bytes)
    }
}

impl CombinatorTrait for Fail {
    type Parser<'a> = FailParser;
    type Output = ();

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b {
        Err(UnambiguousParseError::Fail)
    }

    fn old_parse<'a, 'b>(&self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        (FailParser, ParseResults::empty_finished())
    }
}

impl BaseCombinatorTrait for Fail {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

impl<Output: OutputTrait> ParserTrait<Output> for FailParser {
    fn get_u8set(&self) -> U8Set {
        panic!("FailParser.get_u8set() called")
    }

    fn parse<'b>(&mut self, bytes: &'b [u8]) -> ParseResults<Output> where Output: 'b {
        panic!("FailParser already consumed")
    }
}

pub fn fail() -> Fail {
    Fail
}