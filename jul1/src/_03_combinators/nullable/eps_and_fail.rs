
// src/_03_combinators/nullable/eps_and_fail.rs
use crate::_01_parse_state::{ParseResultTrait, RightData, RightDataGetters, UpData, OneShotUpData};
use crate::{BaseCombinatorTrait, DynCombinatorTrait, UnambiguousParseError, UnambiguousParseResults, DownData};
use crate::{CombinatorTrait, ParseResults, ParserTrait, U8Set};
#[derive(Debug)]
pub struct Eps;

#[derive(Debug)]
pub struct EpsParser;

impl DynCombinatorTrait for Eps {
    fn parse_dyn(&self, down_data: DownData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(down_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(down_data, bytes)
    }
}

impl CombinatorTrait for Eps {
    type Parser<'a> = EpsParser;
    type Output = ();
    type PartialOutput = ();

    fn one_shot_parse(&self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        Ok(OneShotUpData::new(down_data.just_right_data()))
    }

    fn old_parse(&self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        (EpsParser, ParseResults::new_single(UpData::new(down_data.just_right_data()), true))
    }
}

impl BaseCombinatorTrait for Eps {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

impl ParserTrait for EpsParser {
    fn get_u8set(&self) -> U8Set {
        panic!("EpsParser.get_u8set() called")
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
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
    fn parse_dyn(&self, down_data: DownData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(down_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(down_data, bytes)
    }
}

impl CombinatorTrait for Fail {
    type Parser<'a> = FailParser;
    type Output = ();
    type PartialOutput = ();

    fn one_shot_parse(&self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        Err(UnambiguousParseError::Fail)
    }

    fn old_parse(&self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        (FailParser, ParseResults::empty_finished())
    }
}

impl BaseCombinatorTrait for Fail {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

impl ParserTrait for FailParser {
    fn get_u8set(&self) -> U8Set {
        panic!("FailParser.get_u8set() called")
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        panic!("FailParser already consumed")
    }
}

pub fn fail() -> Fail {
    Fail
}