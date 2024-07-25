use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use crate::{choice, choice_from_vec, CombinatorTrait, DynCombinator, eat_byte, eps, opt, ParseResults, ParserTrait, seq, Stats, U8Set};
use crate::parse_state::{RightData, UpData};

#[derive(PartialEq)]
pub struct EatString {
    string: Vec<u8>,
}

#[derive(PartialEq)]
pub struct EatStringParser {
    string: Vec<u8>,
    index: usize,
    right_data: Option<RightData>,
}

impl CombinatorTrait for EatString {
    type Parser = EatStringParser;

    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        let mut parser = EatStringParser {
            string: self.string.clone(),
            index: 0,
            right_data: Some(right_data),
        };
        (parser, ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_u8(self.string[0]) }],
            cut: false,
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
                    ParseResults {
                        right_data_vec: vec![right_data],
                        up_data_vec: vec![],
                        cut: false,
                    }
                } else {
                    ParseResults {
                        right_data_vec: vec![],
                        up_data_vec: vec![UpData { u8set: U8Set::from_u8(self.string[self.index]) }],
                        cut: false,
                    }
                }
            } else {
                ParseResults {
                    right_data_vec: vec![],
                    up_data_vec: vec![],
                        cut: false,
                }
            }
        } else {
            ParseResults {
                right_data_vec: vec![],
                up_data_vec: vec![],
                        cut: false,
            }
        }
    }
    fn collect_stats(&self, stats: &mut Stats) {
        stats.active_parser_type_counts.entry("EatStringParser".to_string()).and_modify(|c| *c += 1).or_insert(1);
        let string = std::str::from_utf8(&self.string).unwrap();
        stats.active_string_matchers.entry(string.to_string()).and_modify(|c| *c += 1).or_insert(1);
    }


    fn dyn_eq(&self, other: &dyn ParserTrait) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn eat_string(string: &str) -> EatString {
    EatString {
        string: string.as_bytes().to_vec(),
    }
}

pub fn eat_bytes(bytes: &[u8]) -> EatString {
    EatString {
        string: bytes.to_vec(),
    }
}

pub fn eat_bytestring_choice(mut bytestrings: Vec<Vec<u8>>) -> Box<DynCombinator> {
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
    let combinator = choice_from_vec(grouped_bytestrings.clone().into_iter().map(|(first, rests)| {
        seq!(eat_byte(first), eat_bytestring_choice(rests))
    }).collect());
    if any_done {
        assert!(grouped_bytestrings.is_empty());
        opt(combinator).into_box_dyn()
    } else {
        combinator
    }
}