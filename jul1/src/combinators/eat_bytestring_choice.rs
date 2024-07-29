use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::{RightData, UpData};

#[derive(Debug, Clone, PartialEq, Eq)]
struct BuildTrieNode {
    valid_bytes: U8Set,
    is_end: bool,
    children: Vec<Option<Rc<BuildTrieNode>>>,
}

impl BuildTrieNode {
    fn new() -> Self {
        BuildTrieNode {
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
                node.children[byte as usize] = Some(Rc::new(BuildTrieNode::new()));
            }
            node = Rc::make_mut(node.children[byte as usize].as_mut().unwrap());
        }
        node.is_end = true;
    }

    fn to_optimized_trie_node(&self) -> TrieNode {
        let children: Vec<Rc<TrieNode>> = self.children.iter()
            .filter_map(|child| child.as_ref().map(|c| Rc::new(c.to_optimized_trie_node())))
            .collect();

        TrieNode {
            valid_bytes: self.valid_bytes,
            is_end: self.is_end,
            children,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrieNode {
    valid_bytes: U8Set,
    is_end: bool,
    children: Vec<Rc<TrieNode>>,
}

impl Hash for TrieNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.valid_bytes.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatByteStringChoice {
    root: Rc<TrieNode>,
}

impl EatByteStringChoice {
    pub fn new(bytestrings: Vec<Vec<u8>>) -> Self {
        let mut build_root = BuildTrieNode::new();
        for bytestring in bytestrings {
            build_root.insert(&bytestring);
        }
        let root = build_root.to_optimized_trie_node();
        EatByteStringChoice { root: Rc::new(root) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
            let child_index = self.current_node.valid_bytes.bitset.count_bits_before(c) as usize;
            if child_index < self.current_node.children.len() {
                let next_node = &self.current_node.children[child_index];
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
