// src/combinators/eat_string.rs
use crate::{dumb_one_shot_parse, UnambiguousParseError, UnambiguousParseResults};
use std::any::Any;
use std::rc::Rc;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set, VecX};
use crate::internal_vec::VecY;
use crate::parse_state::{RightData, ParseResultTrait};

#[derive(Debug)]
pub struct EatString {
    pub(crate) string: Vec<u8>,
}

// impl From<EatString> for Combinator {
//     fn from(value: EatString) -> Self {
//         Combinator::EatString(value)
//     }
// }

#[derive(Debug)]
pub struct EatStringParser<'a> {
    pub(crate) string: &'a [u8],
    index: usize,
    pub(crate) right_data: Option<RightData>,
}

impl CombinatorTrait for EatString {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

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

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let mut parser = EatStringParser {
            string: self.string.as_slice(),
            index: 0,
            right_data: Some(right_data),
        };
        let parse_results = parser.parse(bytes);
        (Parser::EatStringParser(parser), parse_results)
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