use std::rc::Rc;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, ParseState};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seq {
    pub combinators: Rc<Vec<Combinator>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeqParser {
    pub combinators: Rc<Vec<Combinator>>,
    pub i: usize,
    pub parser: Box<Parser>,
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

impl CombinatorTrait for Seq {
    fn init_parser(&self, state: ParseState) -> ParseResults {
        let mut parsers = vec![];
        let mut states = vec![state];
        let mut continuations: Vec<Combinator> = vec![];
        for (i, combinator) in self.combinators.iter().enumerate() {
            let mut new_states = vec![];
            for state in states.drain(..) {
                let parse_results = combinator.init_parser(state);
                parsers.extend(parse_results.parsers);
                new_states.extend(parse_results.states);
                for continuation in parse_results.continuations {
                    let mut new_combinators = vec![continuation];
                    new_combinators.extend(self.combinators[i + 1..].iter().cloned());
                    continuations.push(seq(new_combinators).into());
                }
            }
            if new_states.is_empty() {
                break;
            }
            states = new_states;
        }
        ParseResults { parsers: parsers, continuations, states }
    }
}

impl ParserTrait for SeqParser {
    fn step(&self, c: u8) -> ParseResults {
        todo!()
    }
}
