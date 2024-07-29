use std::collections::BTreeMap;

use crate::{_choice, choice, Choice, Combinator, CombinatorTrait, eat_byte, eat_byte_choice, eat_char_choice, eps, fail, Parser, ParseResults, ParserTrait, seq, Stats, U8Set};
use crate::combinators::derived::opt;
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

pub fn eat_bytestring_choice(bytestrings: Vec<Vec<u8>>) -> Combinator {
    let mut first_byte_map: BTreeMap<u8, Vec<Vec<u8>>> = BTreeMap::new();
    let mut empty_string_present = false;

    for bytestring in bytestrings {
        if bytestring.is_empty() {
            empty_string_present = true;
        } else {
            first_byte_map.entry(bytestring[0])
                .or_default()
                .push(bytestring[1..].to_vec());
        }
    }

    let mut choices: Vec<Combinator> = Vec::new();

    for (first_byte, rest_strings) in first_byte_map {
        let rest_combinator = if rest_strings.is_empty() {
            eps().into()
        } else {
            eat_bytestring_choice(rest_strings)
        };

        let combined_combinator = seq!(eat_byte(first_byte), rest_combinator);
        choices.push(combined_combinator);
    }

    let result = if choices.is_empty() {
        fail().into()
    } else if choices.len() == 1 {
        choices.remove(0)
    } else {
        Choice { children: choices.into_iter().map(|c| c.into()).collect() }.into()
    };

    if empty_string_present {
        opt(result).into()
    } else {
        result
    }
}