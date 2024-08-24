use crate::{dumb_one_shot_parse, BaseCombinatorTrait, DynCombinatorTrait, UnambiguousParseResults};
use std::any::Any;
use std::ops::RangeBounds;
use std::rc::Rc;
use crate::{CombinatorTrait, ParseResults, ParserTrait, U8Set, ParseResultTrait};
use crate::parse_state::{RightData};
#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatU8 {
    pub(crate) u8set: U8Set,
}

#[derive(Debug)]
pub struct EatU8Parser {
    pub(crate) u8set: U8Set,
    pub(crate) right_data: Option<RightData>,
}

impl DynCombinatorTrait for EatU8 {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait>, ParseResults) {
        todo!()
    }
}

impl CombinatorTrait for EatU8 {
    type Parser<'a> = EatU8Parser;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        if bytes.is_empty() {
            return ParseResultTrait::empty_unfinished();
        }

        let mut right_data = right_data;
        if self.u8set.contains(bytes[0]) {
            right_data.get_inner_mut().fields1.position += 1;
            ParseResultTrait::new_single(right_data, true)
        } else {
            ParseResultTrait::empty_finished()
        }
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let mut parser = EatU8Parser {
            u8set: self.u8set.clone(),
            right_data: Some(right_data),
        };
        let parse_results = parser.parse(bytes);
        (parser, parse_results)
    }
}

impl BaseCombinatorTrait for EatU8 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ParserTrait for EatU8Parser {
    fn get_u8set(&self) -> U8Set {
        assert!(self.right_data.is_some(), "EatU8Parser.get_u8set() called but right_data is None");
        return self.u8set.clone();
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        if bytes.is_empty() {
            return ParseResults::empty_unfinished();
        }

        let mut right_data = self.right_data.take().unwrap();
        if self.u8set.contains(bytes[0]) {
            right_data.get_inner_mut().fields1.position += 1;
            ParseResults::new_single(right_data, true)
        } else {
            ParseResults::empty_finished()
        }
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

// impl From<EatU8> for Combinator {
//     fn from(value: EatU8) -> Self {
//         Combinator::EatU8(value)
//     }
// }