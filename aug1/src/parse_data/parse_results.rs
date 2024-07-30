use crate::{Combinator, ParseState};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseResults {
    pub continuations: Vec<Combinator>,
    pub next_parse_states: Vec<ParseState>,
}
