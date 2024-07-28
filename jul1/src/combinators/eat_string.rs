use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

use crate::{choice, Combinator, CombinatorTrait, eat_byte, Parser, ParseResults, ParserTrait, seq, Stats, U8Set};
use crate::combinators::derived::opt;
use crate::parse_state::{RightData, UpData};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EatString {
    string: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatStringParser {
    string: Vec<u8>,
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

    fn collect_stats(&self, stats: &mut Stats) {
        stats.active_parser_type_counts.entry("EatStringParser".to_string()).and_modify(|c| *c += 1).or_insert(1);
        let string = std::str::from_utf8(&self.string).unwrap();
        stats.active_string_matchers.entry(string.to_string()).and_modify(|c| *c += 1).or_insert(1);
    }
}

pub fn eat_string(string: &str) -> Combinator {
    Combinator::EatString(EatString {
        string: string.as_bytes().to_vec(),
    })
}

pub fn eat_bytes(bytes: &[u8]) -> Combinator {
    Combinator::EatString(EatString {
        string: bytes.to_vec()
    })
}

pub fn eat_bytestring_choice(mut bytestrings: Vec<Vec<u8>>) -> Combinator {
    // Group by first byte
    let mut grouped_bytestrings: BTreeMap<u8, Vec<Vec<u8>>> = BTreeMap::new();
    let mut any_done = false;
    for bytestring in bytestrings {
        let [first, rest @ ..] = bytestring.as_slice() else {
            any_done = true;
            continue
        };
        grouped_bytestrings.entry(*first).or_default().push((*rest).to_vec());
    }
    // Create combinators for each group
    let combinator = choice(grouped_bytestrings.clone().into_iter().map(|(first, rests)| {
        seq(vec![Combinator::EatU8(eat_byte(first)), eat_bytestring_choice(rests)])
    }).collect());
    if any_done {
        assert!(grouped_bytestrings.is_empty());
        opt(combinator)
    } else {
        combinator
    }
}
