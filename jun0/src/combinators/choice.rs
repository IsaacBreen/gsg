use crate::parsers::ParserState;
use crate::Readu8::Readu8;
use crate::u8set::u8set;

#[derive(Clone)]
pub struct ChoiceState<S1: ParserState, S2: ParserState> {
    position: usize,
    state1: S1,
    state2: S2,
    phase: u8,
}

impl<S1: ParserState, S2: ParserState> ParserState for ChoiceState<S1, S2> {
    fn new(position: usize) -> Self {
        ChoiceState {
            position,
            state1: S1::new(position),
            state2: S2::new(position),
            phase: 0,
        }
    }

    fn parse<F: Readu8>(&mut self, reader: &F) {
        match self.phase {
            0 => {
                self.state1.parse(reader);
                if !self.state1.is_valid() {
                    self.phase = 1;
                    self.state2 = S2::new(self.position);
                }
            }
            1 => {
                self.state2.parse(reader);
                if !self.state2.is_valid() {
                    self.phase = 2;
                }
            }
            _ => {}
        }
    }

    fn valid_next_u8set(&self) -> u8set {
        match self.phase {
            0 => self.state1.valid_next_u8set(),
            1 => self.state2.valid_next_u8set(),
            _ => u8set::new(),
        }
    }

    fn position(&self) -> usize {
        match self.phase {
            0 => self.state1.position(),
            1 => self.state2.position(),
            _ => self.position,
        }
    }
}

pub fn choice<S1: ParserState, S2: ParserState>(
    parser1: impl Fn(usize) -> S1,
    parser2: impl Fn(usize) -> S2,
) -> impl Fn(usize) -> ChoiceState<S1, S2> {
    move |position| ChoiceState {
        position,
        state1: parser1(position),
        state2: S2::new(position),
        phase: 0,
    }
}