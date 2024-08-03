use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::{RightData, UpData};

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
    pub(crate) right_data: Option<RightData>,
}

impl CombinatorTrait for EatString {

    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        fn parser(_self: &EatString, right_data: RightData) -> (Parser, ParseResults) {
            let mut parser = EatStringParser {
                string: _self.string.clone(),
                index: 0,
                right_data: Some(right_data),
            };
            // println!("EatStringParser: Starting {:?}", parser);
            (Parser::EatStringParser(parser), ParseResults {
                right_data_vec: vec![],
                up_data_vec: vec![UpData { u8set: U8Set::from_u8(_self.string[0]) }],
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
        fn step(_self: &mut EatStringParser, c: u8) -> ParseResults {
            if _self.index < _self.string.len() {
                if _self.string[_self.index] == c {
                    _self.index += 1;
                    if _self.index == _self.string.len() {
                        let mut right_data = _self.right_data.take().unwrap();
                        right_data.position += _self.string.len();
                        // println!("EatStringParser: Matched {:?}", _self);
                        ParseResults {
                            right_data_vec: vec![right_data],
                            up_data_vec: vec![],
                            done: true,
                        }
                    } else {
                        // println!("EatStringParser: Continuing {:?}", _self);
                        ParseResults {
                            right_data_vec: vec![],
                            up_data_vec: vec![UpData { u8set: U8Set::from_u8(_self.string[_self.index]) }],
                            done: false,
                        }
                    }
                } else {
                    // println!("EatStringParser: Failed {:?}", _self);
                    _self.index = _self.string.len();
                    ParseResults::empty_finished()
                }
            } else {
                // println!("EatStringParser: Done {:?}", _self);
                panic!("EatStringParser already consumed")
            }
        }
        if bytes.is_empty() {
            return ParseResults::empty_unfinished();
        }
        let mut right_data_vec = Vec::new();
        for i in 0..bytes.len() {
            let ParseResults { right_data_vec: mut new_right_data_vec, up_data_vec, done } = step(self, bytes[i]);
            right_data_vec.append(&mut new_right_data_vec);
            if done || i == bytes.len() - 1 {
                return ParseResults {
                    right_data_vec,
                    up_data_vec,
                    done,
                };
            }
        }
        unreachable!();
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

