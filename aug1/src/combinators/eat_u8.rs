use crate::{Combinator, CombinatorTrait, ParseResults, ParserTrait, ParseState};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatU8 {
    pub u8: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatU8Parser {
    pub u8: u8,
}

impl CombinatorTrait for EatU8 {
    fn init_parser(&self, _state: ParseState) -> ParseResults {
        ParseResults { parsers: vec![], continuations: vec![], states: vec![] }
    }
}

impl ParserTrait for EatU8Parser {
    fn step(&self, bytes: &[u8]) -> ParseResults {
        todo!()
    }
}

pub fn eat_u8(u8: u8) -> EatU8 {
    EatU8 { u8 }
}

impl From<EatU8> for Combinator {
    fn from(eat_u8: EatU8) -> Self {
        Combinator::EatU8(eat_u8)
    }
}