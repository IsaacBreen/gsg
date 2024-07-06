use crate::{Combinator, ParseData, Parser, ParseResult, U8Set};

#[derive(Clone)]
pub struct EatString {
    string: Vec<u8>,
}

pub enum EatStringParser {
    Predict { string: Vec<u8>, pos: usize, parse_data: ParseData },
    Match { parse_data: ParseData },
    Mismatch,
    Done,
}

impl Combinator for EatString {
    type Parser = EatStringParser;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        if self.string.is_empty() {
            (EatStringParser::Match { parse_data: parse_data.clone() }, ParseResult::new(U8Set::none(), Some(parse_data)))
        } else {
            (EatStringParser::Predict {
                string: self.string.clone(),
                pos: 0,
                parse_data,
            }, ParseResult::new(U8Set::from_u8(self.string[0]), None))
        }
    }
}

impl Parser for EatStringParser {
    fn step(&mut self, c: u8) -> ParseResult {
        let mut new_self: Option<Self> = None;
        let result = match self {
            EatStringParser::Predict { string, pos, parse_data } => {
                if string[*pos] == c {
                    *pos += 1;
                    if *pos == string.len() {
                        new_self = Some(EatStringParser::Match { parse_data: parse_data.clone() });
                        ParseResult::new(U8Set::none(), Some(parse_data.clone()))
                    } else {
                        ParseResult::new(U8Set::from_u8(string[*pos]), None)
                    }
                } else {
                    new_self = Some(EatStringParser::Mismatch);
                    ParseResult::empty()
                }
            }
            EatStringParser::Match { parse_data } => {
                new_self = Some(EatStringParser::Done);
                ParseResult::new(U8Set::none(), Some(parse_data.clone()))
            }
            EatStringParser::Mismatch => {
                new_self = Some(EatStringParser::Done);
                ParseResult::empty()
            }
            EatStringParser::Done => panic!("EatStringParser::Done"),
        };
        if let Some(new_self) = new_self {
            *self = new_self;
        }
        result
    }
}

pub fn eat_string(string: &str) -> EatString {
    EatString { string: string.as_bytes().to_vec() }
}

pub fn eat_bytes(bytes: &[u8]) -> EatString {
    EatString { string: bytes.to_vec() }
}