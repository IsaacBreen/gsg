use std::rc::Rc;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::{RightData, UpData};

#[derive(Debug, Clone)]
struct TrieNode {
    valid_bytes: U8Set,
    is_end: bool,
    children: Vec<Option<Rc<TrieNode>>>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            valid_bytes: U8Set::none(),
            is_end: false,
            children: vec![None; 256],
        }
    }

    fn insert(&mut self, bytestring: &[u8]) {
        let mut node = self;
        for &byte in bytestring {
            node.valid_bytes.insert(byte);
            if node.children[byte as usize].is_none() {
                node.children[byte as usize] = Some(Rc::new(TrieNode::new()));
            }
            node = Rc::make_mut(node.children[byte as usize].as_mut().unwrap());
        }
        node.is_end = true;
    }
}

#[derive(Debug, Clone)]
pub struct EatByteStringChoice {
    root: Rc<TrieNode>,
}

impl EatByteStringChoice {
    pub fn new(bytestrings: Vec<Vec<u8>>) -> Self {
        let mut root = TrieNode::new();
        for bytestring in bytestrings {
            root.insert(&bytestring);
        }
        EatByteStringChoice { root: Rc::new(root) }
    }
}

#[derive(Debug, Clone)]
pub struct EatByteStringChoiceParser {
    current_node: Rc<TrieNode>,
    right_data: RightData,
}

impl CombinatorTrait for EatByteStringChoice {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let parser = EatByteStringChoiceParser {
            current_node: Rc::clone(&self.root),
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
        if self.current_node.valid_bytes.contains(c) {
            let next_node = &self.current_node.children[c as usize];
            if let Some(next_node) = next_node {
                self.current_node = Rc::clone(next_node);
                self.right_data.position += 1;

                if self.current_node.is_end {
                    ParseResults {
                        right_data_vec: vec![self.right_data.clone()],
                        up_data_vec: vec![UpData { u8set: self.current_node.valid_bytes }],
                        done: self.current_node.valid_bytes.is_empty(),
                    }
                } else {
                    ParseResults {
                        right_data_vec: vec![],
                        up_data_vec: vec![UpData { u8set: self.current_node.valid_bytes }],
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

pub fn eat_bytestring_choice(bytestrings: Vec<Vec<u8>>) -> Combinator {
    EatByteStringChoice::new(bytestrings).into()
}

impl From<EatByteStringChoice> for Combinator {
    fn from(value: EatByteStringChoice) -> Self {
        Combinator::EatByteStringChoice(value)
    }
}
