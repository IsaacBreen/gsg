use crate::parsers::ParserState;
use crate::Readu8::Readu8;
use crate::u8set::u8set;

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