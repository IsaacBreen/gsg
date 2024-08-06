use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::RightData;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FailParser;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fail;

impl CombinatorTrait for Fail {
    fn parse(&self, right_data: Box<RightData>, bytes: &[u8]) -> (Parser, ParseResults) {
        (Parser::FailParser(FailParser), ParseResults::empty_finished())
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

impl From<Fail> for Combinator {
    fn from(value: Fail) -> Self {
        Combinator::Fail(value)
    }
}
