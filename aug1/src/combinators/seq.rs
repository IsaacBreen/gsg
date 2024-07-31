use std::rc::Rc;
use crate::{Combinator, CombinatorTrait, WaitingContinuation, Parser, ParseResults, ParserTrait, ParseState, Continuation};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seq {
    pub combinators: Rc<Vec<Combinator>>,
}

impl CombinatorTrait for Seq {
    fn init_parser(&self, state: ParseState) -> ParseResults {
        let mut parse_results = ParseResults::default();
        let [head, tail @ ..] = self.combinators.as_slice() else {
            return parse_results;
        };
        let mut parse_results = head.init_parser(state);
        // Add the rest of the combinators to the tail
        parse_results.extend_tail(tail);
        parse_results
    }
}

pub fn seq(combinators: Vec<Combinator>) -> Seq {
    Seq { combinators: Rc::new(combinators) }
}

impl From<Seq> for Combinator {
    fn from(seq: Seq) -> Self {
        Combinator::Seq(seq)
    }
}

#[macro_export]
macro_rules! seq {
    ($($combinator:expr),*) => {
        $crate::combinators::seq(vec![$($combinator.into()),*])
    };
}
