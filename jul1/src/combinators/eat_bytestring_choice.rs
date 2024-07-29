use std::hash::{Hash, Hasher};
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::{RightData, UpData};

const ALPHABET_SIZE: usize = 256;

#[derive(Clone, PartialEq, Eq)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; ALPHABET_SIZE],
    is_end: bool,
}

impl Hash for TrieNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.is_end.hash(state);
        for child in &self.children {
            child.is_some().hash(state);
        }
    }
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: [None; ALPHABET_SIZE],
            is_end: false,
        }
    }

    fn insert(&mut self, bytestring: &[u8]) {
        let mut node = self;
        for &byte in bytestring {
            node = node.children[byte as usize].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_end = true;
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EatByteStringChoice {
    root: Box<TrieNode>,
}

impl EatByteStringChoice {
    pub fn new(bytestrings: Vec<Vec<u8>>) -> Self {
        let mut root = Box::new(TrieNode::new());
        for bytestring in bytestrings {
            root.insert(&bytestring);
        }
        EatByteStringChoice { root }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EatByteStringChoiceParser {
    current_node: *const TrieNode,
    right_data: RightData,
}

impl CombinatorTrait for EatByteStringChoice {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let parser = EatByteStringChoiceParser {
            current_node: &*self.root,
            right_data,
        };
        (
            Parser::EatByteStringChoiceParser(parser),
            ParseResults {
                right_data_vec: vec![],
                up_data_vec: vec![UpData { u8set: U8Set::from_match_fn(|b| self.root.children[b as usize].is_some()) }],
                done: false,
            }
        )
    }
}

impl ParserTrait for EatByteStringChoiceParser {
    fn step(&mut self, c: u8) -> ParseResults {
        unsafe {
            if let Some(next_node) = (*self.current_node).children[c as usize].as_ref() {
                self.current_node = &**next_node;
                self.right_data.position += 1;

                if (*self.current_node).is_end {
                    ParseResults {
                        right_data_vec: vec![self.right_data.clone()],
                        up_data_vec: vec![UpData { u8set: U8Set::from_match_fn(|b| (*self.current_node).children[b as usize].is_some()) }],
                        done: (*self.current_node).children.iter().all(|child| child.is_none()),
                    }
                } else {
                    ParseResults {
                        right_data_vec: vec![],
                        up_data_vec: vec![UpData { u8set: U8Set::from_match_fn(|b| (*self.current_node).children[b as usize].is_some()) }],
                        done: false,
                    }
                }
            } else {
                ParseResults::empty_finished()
            }
        }
    }
}

pub fn eat_bytestring_choice(bytestrings: Vec<Vec<u8>>) -> Combinator {
    EatByteStringChoice::new(bytestrings).into()
}

impl From<EatByteStringChoice> for Combinator {
    fn from(value: EatByteStringChoice) -> Self {
        Combinator::EatByteStringChoice(value)
    }
}
