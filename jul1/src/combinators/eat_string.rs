use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::{RightData};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatString {
    pub(crate) string: Vec<u8>,
}

impl From<EatString> for Combinator {
    fn from(value: EatString) -> Self {
        Combinator::EatString(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatStringParser {
    pub(crate) string: Vec<u8>,
    index: usize,
    pub(crate) right_: Option<RightData>,
}

impl CombinatorTrait for EatString {

    fn parser_with_steps(&self, right_ RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        fn parser(_self: &EatString, right_ RightData) -> (Parser, ParseResults) {
            let mut parser = EatStringParser {
                string: _self.string.clone(),
                index: 0,
                right_: Some(right_data),
            };
            // println!("EatStringParser: Starting {:?}", parser);
            (Parser::EatStringParser(parser), ParseResults {
                right_data_vec: vec![],
                done: false,
            })
        }
        let (mut parser, mut parse_results0) = parser(self, right_data);
        let parse_results1 = parser.steps(bytes);
        parse_results0.combine_seq(parse_results1);
        (parser, parse_results0)
    }
}

impl ParserTrait for EatStringParser {

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        if bytes.is_empty() {
            return ParseResults::empty_unfinished();
        }

        let mut right_data_vec = Vec::new();
        let mut done = false;

        for &byte in bytes {
            if self.index < self.string.len() {
                if self.string[self.index] == byte {
                    self.index += 1;
                    if self.index == self.string.len() {
                        if let Some(mut right_data) = self.right_.take() {
                            right_data.position += self.string.len();
                            right_data_vec.push(right_data);
                            done = true;
                            break;
                        }
                    }
                } else {
                    self.index = self.string.len();
                    done = true;
                    break;
                }
            } else {
                panic!("EatStringParser already consumed");
            }
        }

        ParseResults {
            right_data_vec,
            done,
        }
    }

    fn valid_next_bytes(&self) -> U8Set {
        if self.index < self.string.len() {
            U8Set::from_u8(self.string[self.index])
        } else {
            U8Set::none()
        }
    }
}

pub fn eat_string(string: &str) -> EatString {
    EatString {
        string: string.as_bytes().to_vec(),
    }
}

pub fn eat_bytes(bytes: &[u8]) -> EatString {
    EatString {
        string: bytes.to_vec()
    }
}

