use crate::{dumb_one_shot_parse, ApplyToChildren, UnambiguousParseResults};
use std::any::Any;
use crate::{CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::{RightData, ParseResultTrait};
#[derive(Debug)]
pub struct Eps;

#[derive(Debug)]
pub struct EpsParser;

impl CombinatorTrait for Eps {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        Ok(right_data)
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        (Parser::EpsParser(EpsParser), ParseResults::new_single(right_data, true))
    }
}

impl ApplyToChildren for Eps {}

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
//
// impl From<Eps> for Combinator {
//     fn from(value: Eps) -> Self {
//         Combinator::Eps(value)
//     }
// }
