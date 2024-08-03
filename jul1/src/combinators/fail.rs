use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::RightData;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FailParser;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fail;

impl CombinatorTrait for Fail {
    fn parser_with_steps(&self, right_ RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        (Parser::FailParser(FailParser), ParseResults::empty_finished())
    }
}

impl ParserTrait for FailParser {
    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        panic!("FailParser already consumed")
    }

    fn valid_next_bytes(&self) -> U8Set {
        U8Set::none()
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
