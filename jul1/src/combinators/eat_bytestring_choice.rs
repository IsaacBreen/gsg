use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::{RightData};

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq, Hash)]
struct TrieNode {
    valid_bytes: U8Set,
    is_end: bool,
    children: Vec<Rc<TrieNode>>,
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
    pub(crate) current_node: Rc<TrieNode>,
    pub(crate) right_data: RightData,
}

impl CombinatorTrait for EatByteStringChoice {

    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        fn parser(_self: &EatByteStringChoice, right_data: RightData) -> (Parser, ParseResults) {
            let parser = EatByteStringChoiceParser {
                current_node: Rc::clone(&_self.root),
                right_data,
            };
            (
                Parser::EatByteStringChoiceParser(parser),
                ParseResults {
                    right_data_vec: vec![],
                    done: false,
                }
            )
        }
        let (mut parser, mut parse_results0) = parser(self, right_data);
        let parse_results1 = parser.steps(bytes);
        parse_results0.combine_seq(parse_results1);
        (parser, parse_results0)
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

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        if bytes.is_empty() {
            return ParseResults::empty_unfinished();
        }

        let mut right_data_vec = Vec::new();
        let mut done = false;

        for &byte in bytes {
            if self.current_node.valid_bytes.contains(byte) {
                let child_index = self.current_node.valid_bytes.bitset.count_bits_before(byte) as usize;
                if child_index < self.current_node.children.len() {
                    self.current_node = Rc::clone(&self.current_node.children[child_index]);
                    self.right_data.position += 1;

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

        ParseResults {
            right_data_vec,
            done,
        }
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
