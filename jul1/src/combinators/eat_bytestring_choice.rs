use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::{RightData, UpData};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TrieNode {
    valid_bytes: U8Set,
    is_end: bool,
    children: Box<[Option<Box<TrieNode>>; 256]>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            valid_bytes: U8Set::none(),
            is_end: false,
            children: Box::new([const { None }; 256]),
        }
    }

    fn insert(&mut self, bytestring: &[u8]) {
        let mut node = self;
        for &byte in bytestring {
            node.valid_bytes.insert(byte);
            node = node.children[byte as usize].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_end = true;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatByteStringChoice {
    root: Box<TrieNode>,
}

impl EatByteStringChoice {
    pub fn new(bytestrings: Vec<Vec<u8>>) -> Self {
        let mut root = TrieNode::new();
        for bytestring in bytestrings {
            root.insert(&bytestring);
        }
        EatByteStringChoice { root: Box::new(root) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
                up_data_vec: vec![UpData { u8set: self.root.valid_bytes }],
                done: false,
            }
        )
    }
}

impl ParserTrait for EatByteStringChoiceParser {
    fn step(&mut self, c: u8) -> ParseResults {
        unsafe {
            let node = &*self.current_node;
            if node.valid_bytes.contains(c) {
                if let Some(next_node) = &node.children[c as usize] {
                    self.current_node = &**next_node;
                    self.right_data.position += 1;

                    let next_node = &*self.current_node;
                    if next_node.is_end {
                        ParseResults {
                            right_data_vec: vec![self.right_data.clone()],
                            up_data_vec: vec![UpData { u8set: next_node.valid_bytes }],
                            done: next_node.valid_bytes.is_empty(),
                        }
                    } else {
                        ParseResults {
                            right_data_vec: vec![],
                            up_data_vec: vec![UpData { u8set: next_node.valid_bytes }],
                            done: false,
                        }
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

pub fn eat_bytestring_choice(bytestrings: Vec<Vec<u8>>) -> Combinator {
    EatByteStringChoice::new(bytestrings).into()
}

impl From<EatByteStringChoice> for Combinator {
    fn from(value: EatByteStringChoice) -> Self {
        Combinator::EatByteStringChoice(value)
    }
}
