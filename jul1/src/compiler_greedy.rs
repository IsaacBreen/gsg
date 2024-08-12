use std::collections::HashSet;
use crate::*;

#[derive(Clone)]
pub enum SimpleCombinator {
    Eps,
    Fail,
    Seq(Vec<SimpleCombinator>),
    Choice(Vec<SimpleCombinator>),
    Opt(Box<SimpleCombinator>),
    Repeat1(Box<SimpleCombinator>),
    EatU8(U8Set),
    EatBytestringChoice(EatByteStringChoice),
    EatString(EatString),
}

impl Combinator {
    fn to_simple_combinator(&self) -> SimpleCombinator {
        match self {
            Combinator::Eps(_) => SimpleCombinator::Eps,
            Combinator::Fail(_) => SimpleCombinator::Fail,
            Combinator::Seq(Seq { children, start_index }) => SimpleCombinator::Seq(children.iter().map(|x| x.to_simple_combinator()).collect()),
            Combinator::Choice(Choice { children, greedy: true }) => SimpleCombinator::Choice(children.iter().map(|x| x.to_simple_combinator()).collect()),
            Combinator::Choice(Choice { greedy: false, .. }) => panic!("Choice with greedy=false is not supported"),
            Combinator::Opt(inner) => SimpleCombinator::Opt(Box::new(inner.inner.to_simple_combinator())),
            Combinator::Repeat1(Repeat1 { a, greedy: true }) => SimpleCombinator::Repeat1(Box::new(a.to_simple_combinator())),
            Combinator::Repeat1(Repeat1 { greedy: false, .. }) => panic!("Repeat1 with greedy=false is not supported"),
            Combinator::EatU8(EatU8 { u8set }) => SimpleCombinator::EatU8(u8set.clone()),
            Combinator::EatByteStringChoice(inner) => SimpleCombinator::EatBytestringChoice(inner.clone()),
            Combinator::EatString(inner) => SimpleCombinator::EatString(inner.clone()),
            _ => panic!("Unsupported combinator {:?}", self),
        }
    }

    pub fn compile_greedy(mut self) -> Combinator {
        let mut simple_combinator = self.to_simple_combinator();
        todo!()
    }
}