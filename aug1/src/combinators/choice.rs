use std::rc::Rc;
use crate::{Combinator, CombinatorTrait, WaitingContinuation, Parser, ParseResults, ParserTrait, ParseState};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Choice {
    pub combinators: Rc<Vec<Combinator>>,
}

impl CombinatorTrait for Choice {
    fn init_parser(&self, state: ParseState) -> ParseResults {
        let mut parse_results = ParseResults::default();
        for combinator in self.combinators.iter().cloned() {
            parse_results.merge(combinator.init_parser(state.clone()));
        }
        ParseResults {
            states: parse_results.states,
            continuations: parse_results.continuations,
            waiting_continuations: parse_results.waiting_continuations,
        }
    }
}

pub fn choice(combinators: Vec<Combinator>) -> Choice {
    Choice { combinators: Rc::new(combinators) }
}

impl From<Choice> for Combinator {
    fn from(choice: Choice) -> Self {
        Combinator::Choice(choice)
    }
}

#[macro_export]
macro_rules! choice {
    ($($combinator:expr),*) => {
        $crate::combinators::choice(vec![$($combinator.into()),*])
    };
}
