use crate::{Combinator, Parser, ParseResult, U8Set};
use crate::parse_data::ParseData;

#[derive(Clone)]
pub struct EatString {
    string: Vec<u8>,
}

pub struct EatStringParser {
    string: Vec<u8>,
    pos: usize,
    maybe_parse_data: Option<ParseData>,
}

impl Combinator for EatString {
    type Parser = EatStringParser;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let mut maybe_parse_data = Some(parse_data);
        let result = if self.string.is_empty() {
            ParseResult::new(U8Set::none(), maybe_parse_data.take())
        } else {
            ParseResult::new(U8Set::from_u8(self.string[0]), None)
        };
        (EatStringParser {
            string: self.string.clone(),
            pos: 0,
            maybe_parse_data,
        }, result)
    }
}

impl Parser for EatStringParser {
    fn step(&mut self, c: u8) -> ParseResult {
        if *self.string.get(self.pos).expect("EatStringParser::exhausted") == c {
            self.pos += 1;
            if self.pos == self.string.len() {
                ParseResult::new(U8Set::none(), self.maybe_parse_data.take())
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