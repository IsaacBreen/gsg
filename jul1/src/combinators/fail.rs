use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait};
use crate::parse_state::RightData;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FailParser;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fail;

impl CombinatorTrait for Fail {
    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        (Parser::FailParser(FailParser), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            done: true,
        })
    }
}

impl ParserTrait for FailParser {
    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
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
