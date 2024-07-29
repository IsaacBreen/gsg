use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, Stats};
use crate::parse_state::RightData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Eps;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EpsParser;

impl CombinatorTrait for Eps {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        (Parser::EpsParser(EpsParser), ParseResults {
            right_data_vec: vec![right_data],
            up_data_vec: vec![],
            done: true,
        })
    }
}

impl ParserTrait for EpsParser {
    fn step(&mut self, c: u8) -> ParseResults {
        panic!("EpsParser already consumed")
    }
}

pub fn eps() -> Eps {
    Eps
}

impl From<Eps> for Combinator {
    fn from(value: Eps) -> Self {
        Combinator::Eps(value)
    }
}
