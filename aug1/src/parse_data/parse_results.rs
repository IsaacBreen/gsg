use crate::{Combinator, Parser, ParseState};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseResults {
    pub parsers: Vec<Parser>,
    pub continuations: Vec<Combinator>,
    pub states: Vec<ParseState>,
}
