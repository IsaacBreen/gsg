use std::collections::BTreeMap;

use crate::{_choice, choice, Combinator, CombinatorTrait, eat_byte, eat_byte_choice, eat_char_choice, eps, fail, Parser, ParseResults, ParserTrait, seq, Stats, U8Set};
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

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub fn eat_bytestring_choice(bytestrings: Vec<Vec<u8>>) -> Combinator {
    let mut trie = OptimizedTrie::new();
    for bytestring in bytestrings {
        trie.insert(&bytestring);
    }
    trie.to_combinator()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct OptimizedTrie {
    children: HashMap<u8, OptimizedTrie>,
    is_end: bool,
}

impl OptimizedTrie {
    fn new() -> Self {
        OptimizedTrie {
            children: HashMap::new(),
            is_end: false,
        }
    }

    fn insert(&mut self, bytestring: &[u8]) {
        let mut node = self;
        for &byte in bytestring {
            node = node.children.entry(byte).or_insert_with(OptimizedTrie::new);
        }
        node.is_end = true;
    }

    fn to_combinator(&'static self) -> Combinator {
        OptimizedByteStringChoice {
            root: self,
        }.into()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OptimizedByteStringChoice {
    root: &'static OptimizedTrie,
}

impl CombinatorTrait for OptimizedByteStringChoice {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let parser = OptimizedByteStringChoiceParser {
            trie: self.root,
            right_data,
        };

        let first_bytes: U8Set = self.root.children.keys().cloned().collect();

        (Parser::OptimizedByteStringChoiceParser(parser), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: first_bytes }],
            done: false,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OptimizedByteStringChoiceParser {
    trie: &'static OptimizedTrie,
    right_data: RightData,
}

impl Hash for OptimizedByteStringChoice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.root.hash(state);
    }
}

impl Hash for OptimizedByteStringChoiceParser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.trie.hash(state);
        self.right_data.hash(state);
    }
}

impl ParserTrait for OptimizedByteStringChoiceParser {
    fn step(&mut self, c: u8) -> ParseResults {
        if let Some(next_trie) = self.trie.children.get(&c) {
            self.trie = next_trie;
            self.right_data.position += 1;

            if self.trie.is_end {
                if self.trie.children.is_empty() {
                    return ParseResults {
                        right_data_vec: vec![self.right_data.clone()],
                        up_data_vec: vec![],
                        done: true,
                    };
                } else {
                    let next_bytes: U8Set = self.trie.children.keys().cloned().collect();
                    return ParseResults {
                        right_data_vec: vec![self.right_data.clone()],
                        up_data_vec: vec![UpData { u8set: next_bytes }],
                        done: false,
                    };
                }
            } else {
                let next_bytes: U8Set = self.trie.children.keys().cloned().collect();
                return ParseResults {
                    right_data_vec: vec![],
                    up_data_vec: vec![UpData { u8set: next_bytes }],
                    done: false,
                };
            }
        }

        ParseResults::empty_finished()
    }
}

impl From<OptimizedByteStringChoice> for Combinator {
    fn from(value: OptimizedByteStringChoice) -> Self {
        Combinator::OptimizedByteStringChoice(value)
    }
}