use crate::{Choice, ChoiceParser, EatU8, EatU8Parser, ParseResults, ParseState, SeqParser};
use crate::combinators::Seq;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Combinator {
    Seq(Seq),
    Choice(Choice),
    EatU8(EatU8),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Parser {
    SeqParser(SeqParser),
    ChoiceParser(ChoiceParser),
    EatU8Parser(EatU8Parser),
}

pub trait CombinatorTrait {
    fn init_parser(&self, state: ParseState) -> ParseResults;
}

pub trait ParserTrait {
    fn step(&self, c: u8) -> ParseResults;
}

impl CombinatorTrait for Combinator {
    fn init_parser(&self, state: ParseState) -> ParseResults {
        match self {
            Combinator::Seq(inner) => inner.init_parser(state),
            // Combinator::Choice(inner) => inner.init_parser(state),
            // Combinator::EatU8(inner) => inner.init_parser(state),
            _ => todo!(),
        }
    }
}

impl ParserTrait for Parser {
    fn step(&self, c: u8) -> ParseResults {
        match self {
            Parser::SeqParser(inner) => inner.step(c),
            // Parser::ChoiceParser(inner) => inner.step(bytes),
            // Parser::EatU8Parser(inner) => inner.step(bytes),
            _ => todo!()
        }
    }
}
