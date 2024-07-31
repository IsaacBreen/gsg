use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, ParseState};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Done;

impl CombinatorTrait for Done {
    fn init_parser(&self, _state: ParseState) -> ParseResults {
        ParseResults {
            states: vec![ParseState::default()],
            ..Default::default()
        }
    }
}

pub fn done() -> Done {
    Done
}

impl From<Done> for Combinator {
    fn from(_: Done) -> Self {
        Combinator::Done(Done)
    }
}
