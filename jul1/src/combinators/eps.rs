use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::RightData;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Eps;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EpsParser;

impl CombinatorTrait for Eps {

    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        (Parser::EpsParser(EpsParser), ParseResults::new(right_data, true))
    }
}

impl ParserTrait for EpsParser {
    fn get_u8set(&self) -> U8Set {
        todo!()
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
