use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, Stats};
use crate::parse_state::RightData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FailParser;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fail;

impl CombinatorTrait for Fail {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        (Parser::FailParser(FailParser), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            done: true,
        })
    }
}

impl ParserTrait for FailParser {
    fn step(&mut self, c: u8) -> ParseResults {
        panic!("FailParser already consumed")
    }
}

pub fn fail() -> Combinator {
    Combinator::Fail(Fail)
}