use crate::{dumb_one_shot_parse, BaseCombinatorTrait, DynCombinatorTrait, UnambiguousParseError, UnambiguousParseResults};
use crate::{CombinatorTrait, ParseResults, ParserTrait, U8Set};
use crate::parse_state::{RightData, ParseResultTrait};

#[repr(transparent)]
#[derive(Debug)]
pub struct FailParser;

#[derive(Debug)]
pub struct Fail;

impl DynCombinatorTrait for Fail {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(right_data, bytes)
    }
}

impl CombinatorTrait for Fail {
    type Parser<'a> = FailParser;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        Err(UnambiguousParseError::Fail)
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
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
