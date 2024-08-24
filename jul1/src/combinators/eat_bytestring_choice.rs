// src/combinators/eat_bytestring_choice.rs
use crate::{dumb_one_shot_parse, BaseCombinatorTrait, DynCombinatorTrait, UnambiguousParseError, UnambiguousParseResults};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use crate::{CombinatorTrait, ParseResults, ParserTrait, U8Set, VecY};
use crate::parse_state::{RightData, ParseResultTrait};
use crate::trie::{FinishReason, TrieNode};

#[derive(Debug)]
pub struct EatByteStringChoice {
    pub(crate) root: Rc<TrieNode>,
}

impl EatByteStringChoice {
    pub fn new(bytestrings: Vec<Vec<u8>>) -> Self {
        EatByteStringChoice { root: Rc::new(bytestrings.into()) }
    }
}

#[derive(Debug)]
pub struct EatByteStringChoiceParser<'a> {
    pub(crate) root: &'a TrieNode,
    pub(crate) current_node: &'a TrieNode,
    pub(crate) right_data: RightData,
}

impl DynCombinatorTrait for EatByteStringChoice {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait>, ParseResults) {
        todo!()
    }
}

impl CombinatorTrait for EatByteStringChoice {
    type Parser<'a> = EatByteStringChoiceParser<'a>;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        let node = self.root.eat_all(bytes);
        if let Some(node) = node {
            if !node.is_end() {
                Err(UnambiguousParseError::Incomplete)
            } else {
                if node.is_terminal() {
                    Ok(right_data)
                } else {
                    Err(UnambiguousParseError::Ambiguous)
                }
            }
        } else {
            Err(UnambiguousParseError::Fail)
        }
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let parser = EatByteStringChoiceParser {
            root: self.root.as_ref(),
            current_node: self.root.as_ref(),
            right_data,
        };
        let parse_results = parser.parse(bytes);
        (parser, parse_results)
    }
}

impl BaseCombinatorTrait for EatByteStringChoice {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ParserTrait for EatByteStringChoiceParser<'_> {
    fn get_u8set(&self) -> U8Set {
        if self.current_node.valid_bytes.is_empty() {
            U8Set::none()
        } else {
            self.current_node.valid_bytes.clone()
        }
    }

    fn parse<'b>(&'b mut self, bytes: &[u8]) -> ParseResults where Self: 'b {
        if bytes.is_empty() {
            return ParseResults::empty_unfinished();
        }
        let (results, last_result) = self.current_node.all_next(bytes);
        let mut right_data_vec = VecY::new();
        for (node, i) in results {
            let mut right_data = self.right_data.clone();
            right_data.get_inner_mut().fields1.position += i;
            right_data_vec.push(right_data);
        }
        let (node, i, reason) = last_result;
        if reason == FinishReason::Success {
            let mut right_data = self.right_data.clone();
            right_data.get_inner_mut().fields1.position += i;
            right_data_vec.push(right_data);
        }
        self.right_data.get_inner_mut().fields1.position += bytes.len();
        self.current_node = node;
        let done = reason != FinishReason::EndOfInput;
        ParseResults::new(right_data_vec, done)
    }
}

pub fn eat_bytestring_choice(bytestrings: Vec<Vec<u8>>)-> impl CombinatorTrait {
    EatByteStringChoice::new(bytestrings)
}

pub fn eat_string_choice(strings: &[&str])-> impl CombinatorTrait {
    eat_bytestring_choice(strings.iter().map(|s| s.as_bytes().to_vec()).collect())
}

// impl From<EatByteStringChoice> for Combinator {
//     fn from(value: EatByteStringChoice) -> Self {
//         Combinator::EatByteStringChoice(value)
//     }
// 