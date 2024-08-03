use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait};
use crate::parse_state::RightData;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Eps;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EpsParser;

impl CombinatorTrait for Eps {
    fn parser_with_steps(&self, right_ RightData, bytes: &[u8]) -> (Parser, ParseResults) {
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

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
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
