use crate::{Combinator, Parser, ParseResult, U8Set};
use crate::parse_data::ParseData;

#[derive(Clone)]
pub struct EatString {
    string: Vec<u8>,
}

pub struct EatStringParser {
    string: Vec<u8>,
    pos: usize,
    parse_data: ParseData,
}

impl Combinator for EatString {
    type Parser = EatStringParser;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let result = if self.string.is_empty() {
            ParseResult::new(U8Set::none(), Some(parse_data.clone()))
        } else {
            ParseResult::new(U8Set::from_u8(self.string[0]), None)
        };
        (EatStringParser {
            string: self.string.clone(),
            pos: 0,
            parse_data,
        }, result)
    }
}

impl Parser for EatStringParser {
    fn step(&mut self, c: u8) -> ParseResult {
        if self.pos == self.string.len() {
            return ParseResult::new(U8Set::none(), Some(self.parse_data.clone()));
        }
        if self.string[self.pos] == c {
            self.pos += 1;
            if self.pos == self.string.len() {
                ParseResult::new(U8Set::none(), Some(self.parse_data.clone()))
            } else {
                ParseResult::new(U8Set::from_u8(self.string[self.pos]), None)
            }
        } else {
            ParseResult::default()
        }
    }
}

pub fn eat_string(string: &str) -> EatString {
    EatString { string: string.as_bytes().to_vec() }
}

pub fn eat_bytes(bytes: &[u8]) -> EatString {
    EatString { string: bytes.to_vec() }
}