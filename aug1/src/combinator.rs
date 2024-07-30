use crate::combinators::Seq;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Combinator {
    Seq(Seq),
    Choice(Choice),
    EatU8(EatU8),
}