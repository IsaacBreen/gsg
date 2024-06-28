// flick between seq.rs and choice.rs to see how similar they are.
use crate::combinators::r#enum::{EnumCombinator};
use crate::parsers::ParserState;
use crate::Readu8::Readu8;
use crate::u8set::u8set;

#[derive(Clone)]
pub struct ChoiceState<EC: EnumCombinator> {
    initial_position: usize,
    state: EC,
}

impl<EC: EnumCombinator> ParserState for ChoiceState<EC> {
    fn new(position: usize) -> Self {
        ChoiceState {
            initial_position: position,
            state: EC::new(position),
        }
    }

    fn parse<F: Readu8>(&mut self, reader: &F) {
        while {
            self.state.parse(reader);
            !self.state.is_valid() && self.state.init_next(self.state.position())
        } {}
    }

    fn valid_next_u8set(&self) -> u8set {
        self.state.valid_next_u8set()
    }

    fn position(&self) -> usize {
        self.state.position()
    }
}
