use crate::parsers::ParserState;
use crate::Readu8::Readu8;
use crate::u8set::u8set;

// seq combinator
#[derive(Clone)]
pub struct SeqState<S1: ParserState, S2: ParserState> {
    position: usize,
    state1: S1,
    state2: S2,
    phase: u8,
}

impl<S1: ParserState, S2: ParserState> ParserState for SeqState<S1, S2> {
    fn new(position: usize) -> Self {
        SeqState {
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
                    self.phase = 2;
                } else if self.state1.position() > self.position {
                    self.phase = 1;
                    self.state2 = S2::new(self.state1.position());
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

pub fn seq<S1: ParserState, S2: ParserState>(
    parser1: impl Fn(usize) -> S1,
    parser2: impl Fn(usize) -> S2,
) -> impl Fn(usize) -> SeqState<S1, S2> {
    move |position| SeqState {
        position,
        state1: parser1(position),
        state2: S2::new(position),
        phase: 0,
    }
}