use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, Stats, U8Set};
use crate::parse_state::{RightData, UpData};

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatU8 {
    pub(crate) u8set: U8Set,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatU8Parser {
    pub(crate) u8set: U8Set,
    right_data: Option<RightData>,
}

impl CombinatorTrait for EatU8 {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let parser = EatU8Parser {
            u8set: self.u8set.clone(),
            right_data: Some(right_data),
        };
        (Parser::EatU8Parser(parser), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData {
                u8set: self.u8set.clone(),
            }],
            done: false,
        })
    }
}

impl ParserTrait for EatU8Parser {
    fn step(&mut self, c: u8) -> ParseResults {
        if self.u8set.contains(c) {
            if let Some(mut right_data) = self.right_data.take() {
                right_data.position += 1;
                return ParseResults {
                    right_data_vec: vec![right_data],
                    up_data_vec: vec![],
                    done: true,
                };
            }
        }
        if let Some(mut right_data) = self.right_data.take() {
            return ParseResults::empty_finished()
        } else {
            panic!("EatU8Parser already consumed")
        }
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        self.step(bytes[0])
    }
}

pub fn eat_byte(byte: u8) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_byte(byte),
    }
}

pub fn eat_char(c: char) -> EatU8 {
    eat_byte(c as u8)
}

pub fn eat_any_byte() -> EatU8 {
    EatU8 {
        u8set: U8Set::all(),
    }
}

pub fn eat_char_choice(chars: &str) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_chars(chars),
    }
}

pub fn eat_char_negation_choice(chars: &str) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_chars_negation(chars),
    }
}

pub fn eat_byte_choice(bytes: &[u8]) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_bytes(bytes),
    }
}

pub fn eat_byte_range(start: u8, end: u8) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_range(start, end),
    }
}

pub fn eat_char_negation(c: char) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_char_negation(c),
    }
}

pub fn eat_char_range(start: char, end: char) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_char_range(start, end),
    }
}

pub fn eat_char_negation_range(start: char, end: char) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_char_negation_range(start, end),
    }
}

pub fn eat_match_fn<F>(f: F) -> EatU8
where
    F: Fn(u8) -> bool,
{
    EatU8 {
        u8set: U8Set::from_match_fn(f),
    }
}

impl From<EatU8> for Combinator {
    fn from(value: EatU8) -> Self {
        Combinator::EatU8(value)
    }
}
