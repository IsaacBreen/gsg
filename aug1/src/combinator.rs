use crate::{Choice, EatU8, EatU8Parser, ParseResults, ParseState};
use crate::combinators::Seq;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Combinator {
    Seq(Seq),
    Choice(Choice),
    EatU8(EatU8),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Parser {
    EatU8Parser(EatU8Parser),
}

pub trait CombinatorTrait {
    fn init_parser(&self, state: ParseState) -> ParseResults;
}

pub trait ParserTrait {
    fn step(self, c: u8) -> ParseResults;
}

impl CombinatorTrait for Combinator {
    fn init_parser(&self, state: ParseState) -> ParseResults {
        match self {
            Combinator::Seq(inner) => inner.init_parser(state),
            Combinator::Choice(inner) => inner.init_parser(state),
            Combinator::EatU8(inner) => inner.init_parser(state),
        }
    }
}

impl ParserTrait for Parser {
    fn step(self, c: u8) -> ParseResults {
        match self {
            Parser::EatU8Parser(inner) => inner.step(c),
        }
    }
}
