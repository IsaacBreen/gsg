use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set, VecY};
use crate::parse_state::RightData;
use crate::trie::{BuildTrieNode, TrieNode};

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
