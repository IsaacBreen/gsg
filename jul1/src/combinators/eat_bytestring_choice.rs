use std::collections::BTreeMap;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::{RightData, UpData};

#[derive(Debug, Clone)]
pub struct EatByteStringChoice {
    root: TrieNode,
}

#[derive(Debug, Clone)]
struct TrieNode {
    valid_bytes: U8Set,
    next_nodes: Vec<Option<Box<TrieNode>>>,
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            valid_bytes: U8Set::none(),
            next_nodes: vec![None; 256],
            is_end: false,
        }
    }
}

impl EatByteStringChoice {
    pub fn new(bytestrings: Vec<Vec<u8>>) -> Self {
        let mut root = TrieNode::new();
        for bytestring in bytestrings {
            Self::insert(&mut root, &bytestring);
        }
        EatByteStringChoice { root }
    }

    fn insert(node: &mut TrieNode, bytestring: &[u8]) {
        let mut current = node;
        for &byte in bytestring {
            current.valid_bytes.insert(byte);
            if current.next_nodes[byte as usize].is_none() {
                current.next_nodes[byte as usize] = Some(Box::new(TrieNode::new()));
            }
            current = current.next_nodes[byte as usize].as_mut().unwrap();
        }
        current.is_end = true;
    }
}

impl CombinatorTrait for EatByteStringChoice {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let parser = EatByteStringChoiceParser {
            root: &self.root,
            current: &self.root,
            right_data: Some(right_data),
        };
        (Parser::EatByteStringChoiceParser(parser), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: self.root.valid_bytes }],
            done: false,
        })
    }
}

#[derive(Debug)]
pub struct EatByteStringChoiceParser<'a> {
    root: &'a TrieNode,
    current: &'a TrieNode,
    right_data: Option<RightData>,
}

impl<'a> ParserTrait for EatByteStringChoiceParser<'a> {
    fn step(&mut self, c: u8) -> ParseResults {
        if self.current.valid_bytes.contains(c) {
            let next_node = self.current.next_nodes[c as usize].as_ref().unwrap();
            self.current = next_node;
            if self.current.is_end {
                let mut right_data = self.right_data.take().unwrap();
                right_data.position += 1;
                ParseResults {
                    right_data_vec: vec![right_data],
                    up_data_vec: vec![],
                    done: true,
                }
            } else {
                ParseResults {
                    right_data_vec: vec![],
                    up_data_vec: vec![UpData { u8set: self.current.valid_bytes }],
                    done: false,
                }
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
