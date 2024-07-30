use crate::{Choice, EatU8, ParseResults, ParseState};
use crate::combinators::Seq;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Combinator {
    Seq(Seq),
    Choice(Choice),
    EatU8(EatU8),
}

pub trait CombinatorTrait {
    fn run(&self, c: u8, state: &mut ParseState) -> ParseResults;
}