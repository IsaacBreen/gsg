use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, ParseState};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Eps;

impl CombinatorTrait for Eps {
    fn init_parser(&self, state: ParseState) -> ParseResults {
        ParseResults {
            continuations: vec![],
            waiting_continuations: vec![],
            states: vec![state],
        }
    }
}

pub fn eps() -> Eps {
    Eps
}

impl From<Eps> for Combinator {
    fn from(_: Eps) -> Self {
        Combinator::Eps(Eps)
    }
}
