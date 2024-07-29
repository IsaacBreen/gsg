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

use std::collections::HashMap;
use std::hash::Hash;

pub fn eat_bytestring_choice(bytestrings: Vec<Vec<u8>>) -> Combinator {
    let mut trie = Trie::new();
    for bytestring in bytestrings {
        trie.insert(&bytestring);
    }
    trie.to_combinator()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Trie {
    children: HashMap<u8, Box<Trie>>,
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            is_end: false,
        }
    }

    fn insert(&mut self, bytestring: &[u8]) {
        let mut node = self;
        for &byte in bytestring {
            node = node.children.entry(byte).or_insert_with(|| Box::new(Trie::new()));
        }
        node.is_end = true;
    }

    fn to_combinator(&self) -> Combinator {
        if self.children.is_empty() {
            return if self.is_end { eps().into() } else { fail().into() };
        }

        let mut choices = Vec::with_capacity(self.children.len());
        for (&byte, child) in &self.children {
            let next = child.to_combinator();
            choices.push(seq!(eat_byte(byte), next));
        }

        let result = if choices.len() == 1 {
            choices.pop().unwrap()
        } else {
            Combinator::Choice(Choice { children: choices.into_iter().map(std::rc::Rc::new).collect() })
        };

        if self.is_end {
            opt(result)
        } else {
            result
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EatByteStringChoice {
    trie: Box<Trie>,
}

impl Hash for EatByteStringChoice {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    }
}

impl From<EatByteStringChoice> for Combinator {
    fn from(value: EatByteStringChoice) -> Self {
        Combinator::EatByteStringChoice(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EatByteStringChoiceParser {
    trie: Box<Trie>,
    current_node: *const Trie,
    right_data: RightData,
}

impl Hash for EatByteStringChoiceParser {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    }
}

impl CombinatorTrait for EatByteStringChoice {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let parser = EatByteStringChoiceParser {
            trie: self.trie.clone(),
            current_node: &*self.trie as *const Trie,
            right_data,
        };
        (Parser::EatByteStringChoiceParser(parser), ParseResults::empty_finished())
    }
}

impl ParserTrait for EatByteStringChoiceParser {
    fn step(&mut self, c: u8) -> ParseResults {
        unsafe {
            let node = &*self.current_node;
            if let Some(child) = node.children.get(&c) {
                self.current_node = &**child as *const Trie;
                self.right_data.position += 1;
                if (*self.current_node).is_end {
                    ParseResults {
                        right_data_vec: vec![self.right_data.clone()],
                        up_data_vec: vec![],
                        done: true,
                    }
                } else {
                    ParseResults::empty_finished()
                }
            } else {
                ParseResults::empty_finished()
            }
        }
    }
}