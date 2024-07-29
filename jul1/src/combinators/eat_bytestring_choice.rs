use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::{RightData, UpData};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TrieNode {
    valid_bytes: U8Set,
    is_end: bool,
    children: Vec<TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            valid_bytes: U8Set::none(),
            is_end: false,
            children: Vec::new(),
        }
    }

    fn insert(&mut self, bytestring: &[u8]) {
        let mut node = self;
        for &byte in bytestring {
            if !node.valid_bytes.contains(byte) {
                node.valid_bytes.insert(byte);
                node.children.push(TrieNode::new());
            }
            let index = node.valid_bytes.iter().take_while(|&b| b != byte).count();
            node = &mut node.children[index];
        }
        node.is_end = true;
    }

    fn get_child(&self, byte: u8) -> Option<&TrieNode> {
        if self.valid_bytes.contains(byte) {
            let index = self.valid_bytes.iter().take_while(|&b| b != byte).count();
            Some(&self.children[index])
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatByteStringChoice {
    root: TrieNode,
}

impl EatByteStringChoice {
    pub fn new(bytestrings: Vec<Vec<u8>>) -> Self {
        let mut root = TrieNode::new();
        for bytestring in bytestrings {
            root.insert(&bytestring);
        }
        EatByteStringChoice { root }
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
            current_node: &self.root,
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
            if let Some(next_node) = node.get_child(c) {
                self.current_node = next_node;
                self.right_data.position += 1;

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
