use std::collections::BTreeMap;

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
    right_data: Option<RightData>,
}

impl CombinatorTrait for EatString {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let mut parser = EatStringParser {
            string: self.string.clone(),
            index: 0,
            right_data: Some(right_data),
        };
        // println!("EatStringParser: Starting {:?}", parser);
        (Parser::EatStringParser(parser), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_u8(self.string[0]) }],
            done: false,
        })
    }
}

impl ParserTrait for EatStringParser {
    fn step(&mut self, c: u8) -> ParseResults {
        if self.index < self.string.len() {
            if self.string[self.index] == c {
                self.index += 1;
                if self.index == self.string.len() {
                    let mut right_data = self.right_data.take().unwrap();
                    right_data.position += self.string.len();
                    // println!("EatStringParser: Matched {:?}", self);
                    ParseResults {
                        right_data_vec: vec![right_data],
                        up_data_vec: vec![],
                        done: true,
                    }
                } else {
                    // println!("EatStringParser: Continuing {:?}", self);
                    ParseResults {
                        right_data_vec: vec![],
                        up_data_vec: vec![UpData { u8set: U8Set::from_u8(self.string[self.index]) }],
                        done: false,
                    }
                }
            } else {
                // println!("EatStringParser: Failed {:?}", self);
                self.index = self.string.len();
                ParseResults::empty_finished()
            }
        } else {
            // println!("EatStringParser: Done {:?}", self);
            panic!("EatStringParser already consumed")
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

use crate::combinators::eat_bytestring_choice::eat_bytestring_choice;

pub use crate::combinators::eat_bytestring_choice::EatByteStringChoice;
