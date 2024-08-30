
// src/_03_combinators/core/eat.rs
use crate::internal_vec::VecY;
use crate::_01_parse_state::{RightData, UpData};
use crate::UnambiguousParseError;
use crate::{BaseCombinatorTrait, DynCombinatorTrait, UnambiguousParseResults, DownData, OneShotUpData};
use crate::{CombinatorTrait, ParseResultTrait, ParseResults, ParserTrait, U8Set};
use std::ops::RangeBounds;

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatU8 {
    pub(crate) u8set: U8Set,
}

#[derive(Debug)]
pub struct EatU8Parser {
    pub(crate) u8set: U8Set,
    pub(crate) down_data: Option<DownData>,
}

impl DynCombinatorTrait for EatU8 {
    fn parse_dyn(&self, down_data: DownData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(down_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(down_data, bytes)
    }
}

impl CombinatorTrait for EatU8 {
    type Parser<'a> = EatU8Parser;
    type Output = ();
    type PartialOutput = ();

    fn one_shot_parse(&self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        if bytes.is_empty() {
            return ParseResultTrait::empty_unfinished();
        }

        let mut right_data = down_data.right_data;
        if self.u8set.contains(bytes[0]) {
            right_data.get_inner_mut().fields1.position += 1;
            ParseResultTrait::new_single(OneShotUpData { right_data }, true)
        } else {
            ParseResultTrait::empty_finished()
        }
    }

    fn old_parse(&self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let mut parser = EatU8Parser {
            u8set: self.u8set.clone(),
            down_data: Some(down_data),
        };
        let parse_results = parser.parse(down_data, bytes);
        (parser, parse_results)
    }
}

impl BaseCombinatorTrait for EatU8 {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

impl ParserTrait for EatU8Parser {
    fn get_u8set(&self) -> U8Set {
        assert!(self.down_data.is_some(), "EatU8Parser.get_u8set() called but right_data is None");
        return self.u8set.clone();
    }

    fn parse(&mut self, down_data: DownData, bytes: &[u8]) -> ParseResults {
        if bytes.is_empty() {
            return ParseResults::empty_unfinished();
        }

        let mut right_data = self.down_data.take().unwrap().right_data;
        if self.u8set.contains(bytes[0]) {
            right_data.get_inner_mut().fields1.position += 1;
            ParseResults::new_single(UpData { right_data }, true)
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


#[derive(Debug)]
pub struct EatString {
    pub(crate) string: Vec<u8>,
}

#[derive(Debug)]
pub struct EatStringParser<'a> {
    pub(crate) string: &'a [u8],
    index: usize,
    pub(crate) down_data: Option<DownData>,
}

impl DynCombinatorTrait for EatString {
    fn parse_dyn(&self, down_data: DownData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(down_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(down_data, bytes)
    }
}

impl CombinatorTrait for EatString {
    type Parser<'a> = EatStringParser<'a>;
    type Output = ();
    type PartialOutput = ();

    fn one_shot_parse(&self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        if bytes.len() < self.string.len() {
            return Err(UnambiguousParseError::Incomplete);
        }

        if self.string == bytes[..self.string.len()] {
            let mut right_data = down_data.right_data;
            right_data.get_inner_mut().fields1.position += self.string.len();
            Ok(OneShotUpData { right_data })
        } else {
            Err(UnambiguousParseError::Fail)
        }
    }

    fn old_parse(&self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let mut parser = EatStringParser {
            string: self.string.as_slice(),
            index: 0,
            down_data: Some(down_data),
        };
        let parse_results = parser.parse(down_data, bytes);
        (parser, parse_results)
    }
}

impl BaseCombinatorTrait for EatString {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

impl ParserTrait for EatStringParser<'_> {
    fn get_u8set(&self) -> U8Set {
        U8Set::from_byte(self.string[self.index])
    }

    fn parse(&mut self, down_data: DownData, bytes: &[u8]) -> ParseResults {
        if bytes.is_empty() {
            return ParseResults::empty_unfinished();
        }

        let mut up_data_vec = VecY::new();
        let mut done = false;

        for &byte in bytes {
            if self.string[self.index] == byte {
                self.index += 1;
                if self.index == self.string.len() {
                    let mut right_data = self.down_data.take().expect("right_data already taken").right_data;
                    right_data.get_inner_mut().fields1.position += self.string.len();
                    up_data_vec.push(UpData { right_data });
                    done = true;
                    break;
                }
            } else {
                done = true;
                self.down_data.take();
                break;
            }
        }

        ParseResults::new(up_data_vec, done)
    }
}

pub fn eat_string(string: &str) -> EatString {
    EatString { string: string.as_bytes().to_vec() }
}

pub fn eat_bytes(bytes: &[u8]) -> EatString {
    EatString { string: bytes.to_vec() }
}

pub fn eat(string: impl Into<String>) -> EatString {
    EatString { string: string.into().into_bytes() }
}