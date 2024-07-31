use std::rc::Rc;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, ParseState};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Choice {
    pub combinators: Rc<Vec<Combinator>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChoiceParser {
    pub combinators: Rc<Vec<Combinator>>,
    pub parsers: Vec<Parser>,
}

impl CombinatorTrait for Choice {
    fn init_parser(&self, state: ParseState) -> ParseResults {
        let mut parsers = vec![];
        let mut states = vec![];
        let mut continuations = vec![];
        for combinator in self.combinators.iter().cloned() {
            let parse_results = combinator.init_parser(state.clone());
            parsers.extend(parse_results.parsers);
            states.extend(parse_results.states);
            continuations.extend(parse_results.continuations);
        }
        ParseResults { parsers: parsers, continuations, states }
    }
}

impl ParserTrait for ChoiceParser {
    fn step(&self, c: u8) -> ParseResults {
        todo!()
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


macro_rules! choice {
    ($($combinator:expr),*) => {
        $crate::combinators::choice(vec![$($combinator.into()),*])
    };
}