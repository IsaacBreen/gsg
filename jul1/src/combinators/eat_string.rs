use std::collections::BTreeMap;

use crate::{_choice, choice, Combinator, CombinatorTrait, eat_byte, eat_byte_choice, eat_char_choice, Parser, ParseResults, ParserTrait, seq, Stats, U8Set};
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

pub fn eat_bytestring_choice(mut bytestrings: Vec<Vec<u8>>) -> Combinator {
    // Group by first byte
    let mut grouped_bytestrings: BTreeMap<u8, (bool, Vec<Vec<u8>>)> = BTreeMap::new();
    let mut any_done = false;
    for bytestring in bytestrings {
        let [first, rest @ ..] = bytestring.as_slice() else {
            any_done = true;
            continue
        };
        grouped_bytestrings.entry(*first).or_default().0 |= rest.is_empty();
        if !rest.is_empty() {
            grouped_bytestrings.entry(*first).or_default().1.push((*rest).to_vec());
        }
    }
    // Create combinators for each group
    let combinator = choice!(_choice(grouped_bytestrings.clone().into_iter().map(|(first, (done, rests))| {
        if !rests.is_empty() { Some(seq!(eat_byte(first), eat_bytestring_choice(rests))) } else { None }
    }).filter_map(|x| x).collect()), eat_byte_choice(grouped_bytestrings.clone().into_iter().filter_map(|(first, (done, rests))| {
        if done { Some(first) } else { None }
    }).collect::<Vec<_>>().as_slice()));
    if any_done {
        assert!(grouped_bytestrings.is_empty());
        opt(combinator).into()
    } else {
        combinator
    }
}
