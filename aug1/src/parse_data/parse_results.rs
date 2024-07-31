use crate::{Combinator, Parser, ParseState};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Continuation {
    pub head: Parser,
    pub tail: Vec<Combinator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WaitingContinuation {
    pub head: Combinator,
    pub tail: Vec<Combinator>,
    pub state: ParseState,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ParseResults {
    pub continuations: Vec<Continuation>,
    pub waiting_continuations: Vec<WaitingContinuation>,
    pub states: Vec<ParseState>,
}

impl ParseResults {
    pub fn extend_tail(&mut self, tail: &[Combinator]) {
        for continuation in self.continuations.iter_mut() {
            continuation.tail.extend(tail.iter().cloned());
        }
        for waiting_continuation in self.waiting_continuations.iter_mut() {
            waiting_continuation.tail.extend(tail.iter().cloned());
        }
    }

    pub fn merge(&mut self, mut p0: ParseResults) {
        self.continuations.append(&mut p0.continuations);
        self.waiting_continuations.append(&mut p0.waiting_continuations);
        self.states.append(&mut p0.states);
    }
}