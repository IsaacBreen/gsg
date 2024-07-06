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

    fn parser(&self, parse_data: ParseData) -> (ParseResult, Self::Parser) {
        (
            ParseResult::new(U8Set::from_u8(self.string[0]), None),
            EatStringParser::Predict {
                string: self.string.clone(),
                pos: 0,
                parse_data,
            }
        )
    }
}

impl Parser for EatStringParser {
    fn step(self, c: u8) -> (ParseResult, Self::Parser) {
        match self {
            EatStringParser::Predict { string, mut pos, parse_data } => {
                if string[pos] == c {
                    pos += 1;
                    if pos == string.len() {
                        (
                            ParseResult::new(U8Set::none(), Some(parse_data.clone())),
                            EatStringParser::Match { parse_data }
                        )
                    } else {
                        (
                            ParseResult::new(U8Set::from_u8(string[pos]), None),
                            EatStringParser::Predict { string, pos, parse_data }
                        )
                    }
                } else {
                    (
                        ParseResult::new(U8Set::none(), None),
                        EatStringParser::Mismatch
                    )
                }
            }
            EatStringParser::Match { parse_data } => (
                ParseResult::new(U8Set::none(), Some(parse_data.clone())),
                EatStringParser::Done
            ),
            EatStringParser::Mismatch => (
                ParseResult::new(U8Set::none(), None),
                EatStringParser::Done
            ),
            EatStringParser::Done => panic!("EatStringParser::Done"),
        }
    }
}

pub fn eat_string(string: &str) -> EatString {
    EatString { string: string.as_bytes().to_vec() }
}

pub fn eat_bytes(bytes: &[u8]) -> EatString {
    EatString { string: bytes.to_vec() }
}