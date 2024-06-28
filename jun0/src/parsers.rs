use crate::u8set::u8set;
use crate::Readu8::Readu8;

pub trait ParserState: Clone {
    fn new(position: usize) -> Self;
    fn parse<F: Readu8>(&mut self, reader: &F);
    fn valid_next_u8set(&self) -> u8set;
    fn is_valid(&self) -> bool {
        !self.valid_next_u8set().is_empty()
    }
    fn position(&self) -> usize;
}

// eat_u8 combinator
#[derive(Clone)]
pub struct EatU8State {
    position: usize,
    target: u8,
    consumed: bool,
}

impl ParserState for EatU8State {
    fn new(position: usize) -> Self {
        EatU8State {
            position,
            target: 0, // This will be set when creating the combinator
            consumed: false,
        }
    }

    fn parse<F: Readu8>(&mut self, reader: &F) {
        if !self.consumed {
            if let Some(byte) = reader.read_u8(self.position) {
                if byte == self.target {
                    self.position += 1;
                    self.consumed = true;
                }
            }
        }
    }

    fn valid_next_u8set(&self) -> u8set {
        let mut set = u8set::new();
        if !self.consumed {
            set.insert(self.target);
        }
        set
    }

    fn position(&self) -> usize {
        self.position
    }
}

pub fn eat_u8(target: u8) -> impl Fn(usize) -> EatU8State {
    move |position| EatU8State {
        position,
        target,
        consumed: false,
    }
}

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

// choice combinator
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
