use std::ops::RangeBounds;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::RightData;
#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatU8 {
    pub(crate) u8set: U8Set,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatU8Parser {
    pub(crate) u8set: U8Set,
    pub(crate) right_data: Option<RightData>,
}

impl CombinatorTrait for EatU8 {
    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let parser = EatU8Parser {
            u8set: self.u8set.clone(),
            right_data: Some(right_data),
        };
        (Parser::EatU8Parser(parser), ParseResults {
            right_data_vec: vec![],
            done: false,
        })
    }
}

impl ParserTrait for EatU8Parser {
    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        if bytes.is_empty() {
            return ParseResults::empty_unfinished();
        }

        if self.u8set.contains(bytes[0]) {
            if let Some(mut right_data) = self.right_data.take() {
                right_data.position += 1;
                return ParseResults {
                    right_data_vec: vec![right_data],
                    done: true,
                };
            }
        }

        if self.right_data.is_some() {
            return ParseResults::empty_finished();
        } else {
            panic!("EatU8Parser already consumed");
        }
    }

    fn next_u8set(&self, bytes: &[u8]) -> U8Set {
        self.u8set.clone()
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

pub fn eat_char_range(range: impl IntoIterator<Item=u8>) -> EatU8 {
    EatU8 { u8set: U8Set::from_byte_range(range) }
}

pub fn eat_byte_negation_range(range: impl IntoIterator<Item=u8>) -> EatU8 {
    EatU8 { u8set: U8Set::from_char_negation_range(range) }
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
