use std::rc::Rc;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, ParseState, Continuation, WaitingContinuation};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Repeat1 {
    pub combinator: Rc<Combinator>,
}

impl CombinatorTrait for Repeat1 {
    fn init_parser(&self, state: ParseState) -> ParseResults {
        let mut parse_results = self.combinator.init_parser(state);
        for continuation in parse_results.continuations.iter_mut() {
            continuation.tail.push(self.clone().into());
        }
        for waiting_continuation in parse_results.waiting_continuations.iter_mut() {
            waiting_continuation.tail.push(self.clone().into());
        }
        parse_results
    }
}

pub fn repeat1(combinator: impl Into<Combinator>) -> Repeat1 {
    Repeat1 { combinator: Rc::new(combinator.into()) }
}

impl From<Repeat1> for Combinator {
    fn from(repeat1: Repeat1) -> Self {
        Combinator::Repeat1(repeat1)
    }
}
