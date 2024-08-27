use crate::internal_vec::VecY;
use crate::parse_state::{ParseResultTrait, RightData};
// src/combinators/eat_string.rs
use crate::{BaseCombinatorTrait, DynCombinatorTrait, UnambiguousParseError, UnambiguousParseResults};
use crate::{CombinatorTrait, ParseResults, ParserTrait, U8Set};

#[derive(Debug)]
pub struct EatString {
    pub(crate) string: Vec<u8>,
}

#[derive(Debug)]
pub struct EatStringParser<'a> {
    pub(crate) string: &'a [u8],
    index: usize,
    pub(crate) right_data: Option<RightData>,
}

impl DynCombinatorTrait for EatString {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(right_data, bytes)
    }
}

impl CombinatorTrait for EatString {
    type Parser<'a> = EatStringParser<'a>;
    type Output = ();

    fn one_shot_parse(&self, mut right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        if bytes.len() < self.string.len() {
            return Err(UnambiguousParseError::Incomplete);
        }

        if self.string == bytes[..self.string.len()] {
            right_data.get_inner_mut().fields1.position += self.string.len();
            Ok(right_data)
        } else {
            Err(UnambiguousParseError::Fail)
        }
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let mut parser = EatStringParser {
            string: self.string.as_slice(),
            index: 0,
            right_data: Some(right_data),
        };
        let parse_results = parser.parse(bytes);
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

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        if bytes.is_empty() {
            return ParseResults::empty_unfinished();
        }

        let mut right_data_vec = VecY::new();
        let mut done = false;

        for &byte in bytes {
            if self.string[self.index] == byte {
                self.index += 1;
                if self.index == self.string.len() {
                    let mut right_data = self.right_data.take().expect("right_data already taken");
                    right_data.get_inner_mut().fields1.position += self.string.len();
                    right_data_vec.push(right_data);
                    done = true;
                    break;
                }
            } else {
                done = true;
                self.right_data.take();
                break;
            }
        }

        ParseResults::new(right_data_vec, done)
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