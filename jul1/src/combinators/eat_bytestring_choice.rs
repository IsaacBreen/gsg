use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set, vecx, VecY};
use crate::parse_state::{RightData};
use crate::VecX;

#[derive(Clone, PartialEq, Eq)]
pub struct BuildTrieNode {
    valid_bytes: U8Set,
    is_end: bool,
    children: VecX<Option<Rc<BuildTrieNode>>>,
}

impl BuildTrieNode {
    pub(crate) fn new() -> Self {
        BuildTrieNode {
            valid_bytes: U8Set::none(),
            is_end: false,
            children: vecx![None; 256],
        }
    }

    pub(crate) fn insert(&mut self, bytestring: &[u8]) {
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

    pub(crate) fn to_optimized_trie_node(&self) -> TrieNode {
        let children: Vec<Rc<TrieNode>> = self.children.iter()
            .filter_map(|child| child.as_ref().map(|c| Rc::new(c.to_optimized_trie_node())))
            .collect();

        TrieNode {
            valid_bytes: self.valid_bytes,
            is_end: self.is_end,
            children: children.into(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct TrieNode {
    pub(crate) valid_bytes: U8Set,
    pub(crate) is_end: bool,
    pub(crate) children: VecX<Rc<TrieNode>>,
}

impl Debug for TrieNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TrieNode")
            .field("valid_bytes", &self.valid_bytes)
            .field("is_end", &self.is_end)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatByteStringChoice {
    pub(crate) root: Rc<TrieNode>,
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
    pub(crate) current_node: Rc<TrieNode>,
    pub(crate) right_data: RightData,
}

impl CombinatorTrait for EatByteStringChoice {

    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let mut parser = EatByteStringChoiceParser {
            current_node: Rc::clone(&self.root),
            right_data,
        };
        let parse_results = parser.parse(bytes);
        (Parser::EatByteStringChoiceParser(parser), parse_results)
    }
}

impl ParserTrait for EatByteStringChoiceParser {
    fn get_u8set(&self) -> U8Set {
        if self.current_node.valid_bytes.is_empty() {
            U8Set::none()
        } else {
            self.current_node.valid_bytes.clone()
        }
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        if bytes.is_empty() {
            return ParseResults::empty_unfinished();
        }

        let mut right_data_vec = VecY::new();
        let mut done = false;

        for &byte in bytes {
            if self.current_node.valid_bytes.contains(byte) {
                let child_index = self.current_node.valid_bytes.bitset.count_bits_before(byte) as usize;
                if child_index < self.current_node.children.len() {
                    self.current_node = Rc::clone(&self.current_node.children[child_index]);
                    Rc::make_mut(&mut self.right_data.right_data_inner).position += 1;

                    if self.current_node.is_end {
                        right_data_vec.push(self.right_data.clone());
                        done = self.current_node.valid_bytes.is_empty();
                        break;
                    } else {
                    }
                } else {
                    done = true;
                    break;
                }
            } else {
                done = true;
                break;
            }
        }

        ParseResults::new(right_data_vec, done)
    }
}

pub fn eat_bytestring_choice(bytestrings: Vec<Vec<u8>>) -> Combinator {
    EatByteStringChoice::new(bytestrings).into()
}

pub fn eat_string_choice(strings: &[&str]) -> Combinator {
    eat_bytestring_choice(strings.iter().map(|s| s.as_bytes().to_vec()).collect())
}

impl From<EatByteStringChoice> for Combinator {
    fn from(value: EatByteStringChoice) -> Self {
        Combinator::EatByteStringChoice(value)
    }
}
