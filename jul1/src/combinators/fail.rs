use crate::{dumb_one_shot_parse, BaseCombinatorTrait, UnambiguousParseError, UnambiguousParseResults};
use crate::{CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::{RightData, ParseResultTrait};

#[repr(transparent)]
#[derive(Debug)]
pub struct FailParser;

#[derive(Debug)]
pub struct Fail;

impl CombinatorTrait for Fail {
    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        Err(UnambiguousParseError::Fail)
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        (Parser::FailParser(FailParser), ParseResults::empty_finished())
    }
}

impl BaseCombinatorTrait for Fail {
    fn as_any(&self) -> &dyn std::any::Any {
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

// impl From<Fail> for Combinator {
//     fn from(value: Fail) -> Self {
//         Combinator::Fail(value)
//     }
// }
