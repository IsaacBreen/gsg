use crate::parsers::ParserState;
use crate::Readu8::Readu8;
use crate::u8set::u8set;

#[derive(Clone)]
pub enum EnumCombinator<S1: ParserState, S2: ParserState> {
    S1(S1),
    S2(S2),
}

macro_rules! visit {
    ($self:expr, $state:ident => $action:expr) => {
        match $self {
            EnumCombinator::S1($state) => $action,
            EnumCombinator::S2($state) => $action,
        }
    };
}

impl<S1: ParserState, S2: ParserState> ParserState for EnumCombinator<S1, S2> {
    fn new(position: usize) -> Self { EnumCombinator::S1(S1::new(position)) }
    fn parse<F: Readu8>(&mut self, reader: &F) { visit!(self, state => state.parse(reader)) }
    fn valid_next_u8set(&self) -> u8set { visit!(self, state => state.valid_next_u8set()) }
    fn position(&self) -> usize { visit!(self, state => state.position()) }
}

impl<S1: ParserState, S2: ParserState> EnumCombinator<S1, S2> {
    pub fn init_next(&mut self, position: usize) -> bool {
        let new_state = match self {
            EnumCombinator::S1(state) => S2::new(position),
            EnumCombinator::S2(state) => return false,
        };
        *self = EnumCombinator::S2(new_state);
        true
    }
}