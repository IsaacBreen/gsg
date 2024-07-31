use crate::{Combinator, CombinatorTrait, Continuation, Parser, ParseResults, ParserTrait, ParseState};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatU8 {
    pub u8: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatU8Parser {
    pub u8: u8,
    pub state: ParseState,
}

impl CombinatorTrait for EatU8 {
    fn init_parser(&self, state: ParseState) -> ParseResults {
        let parser = EatU8Parser { u8: self.u8, state };
        let continuation = Continuation { head: parser.into(), tail: vec![] };
        ParseResults { continuations: vec![continuation], waiting_continuations: vec![], states: vec![] }
    }
}

impl ParserTrait for EatU8Parser {
    fn step(self, c: u8) -> ParseResults {
        if c == self.u8 {
            ParseResults {
                states: vec![self.state],
                ..Default::default()
            }
        } else {
            ParseResults {
                states: vec![],
                ..Default::default()
            }
        }
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

impl From<EatU8Parser> for Parser {
    fn from(eat_u8_parser: EatU8Parser) -> Self {
        Parser::EatU8Parser(eat_u8_parser)
    }
}
