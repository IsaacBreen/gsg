use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::{Combinator, combinator, EatByteStringChoice, EatU8, eps, U8Set};
use crate::trie::{FinishReason, TrieNode};
use std::collections::HashMap;
use crate::FastCombinatorResult::Failure;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FastCombinator {
    Seq(Rc<Vec<FastCombinator>>),
    Choice(Rc<Vec<FastCombinator>>),
    Opt(Box<FastCombinator>),
    Repeat1(Rc<FastCombinator>),
    Eps,
    EatU8(U8Set),
    EatByteStringChoiceFast(Rc<TrieNode>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FastParser {
    SeqParser(Rc<Vec<FastCombinator>>, usize),
    ChoiceParser(Rc<Vec<FastCombinator>>, usize),
    Repeat1Parser(Rc<FastCombinator>),
    EatByteStringChoiceFastParser(Rc<TrieNode>, Rc<TrieNode>),
    EatU8Parser(U8Set),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FastCombinatorResult {
    Success(usize),
    Failure,
    Incomplete(FastParser, Vec<usize>),
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum FastParserResult {
//     Success(usize),
//     Failure,
//     Incomplete(Vec<usize>),
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FastParserResult {
    pub(crate) finish_positions: Vec<usize>,
    pub(crate) done: bool,
}

impl FastCombinator {
    pub fn slow(&self) -> Combinator {
        match self {
            FastCombinator::Seq(children) => {
                let mut all_children: crate::VecX<Combinator> = crate::vecx![];
                for child in children.iter() {
                    let child_slow = child.slow();
                    match child_slow {
                        Combinator::Seq(crate::Seq { children, .. }) => {
                            all_children.extend(children.iter().cloned());
                        }
                        _ => all_children.push(child_slow),
                    }
                }
                crate::_seq(all_children)
            }
            FastCombinator::Choice(children) => {
                let mut all_children: crate::VecX<Combinator> = crate::vecx![];
                for child in children.iter() {
                    let child_slow = child.slow();
                    match child_slow {
                        Combinator::Choice(crate::Choice { children, .. }) => {
                            all_children.extend(children.iter().cloned());
                        }
                        _ => all_children.push(child_slow),
                    }
                }
                crate::_choice(all_children)
            }
            FastCombinator::Opt(parser) => crate::opt(parser.slow()).into(),
            FastCombinator::Repeat1(parser) => crate::repeat1(parser.slow()).into(),
            FastCombinator::Eps => crate::eps().into(),
            FastCombinator::EatU8(u8set) => crate::EatU8 { u8set: *u8set }.into(),
            FastCombinator::EatByteStringChoiceFast(root) => {
                crate::EatByteStringChoice { root: Rc::clone(root) }.into()
            }
        }
    }
}

pub trait FastCombinatorTrait {
    fn parse(&self, bytes: &[u8]) -> FastCombinatorResult;
}

pub trait FastParserTrait {
    fn parse(&mut self, bytes: &[u8]) -> FastParserResult;
    fn get_u8set(&self) -> U8Set;
}

impl FastCombinatorTrait for FastCombinator {
    fn parse(&self, bytes: &[u8]) -> FastCombinatorResult {
        match self {
            FastCombinator::Seq(children) => {
                let mut position = 0;
                let mut finish_positions = vec![];
            }
            FastCombinator::Choice(children) => {
                for (i, child) in children.iter().enumerate() {
                    match child.parse(bytes) {
                        FastCombinatorResult::Success(len) => {
                            return FastCombinatorResult::Success(len);
                        }
                        FastCombinatorResult::Failure => {
                            continue;
                        }
                        FastCombinatorResult::Incomplete(parser, consumed) => {
                            let mut parsers = vec![];
                            let mut finish_positions = vec![];

                        }
                    }
                }
                FastCombinatorResult::Failure
            }
            FastCombinator::Opt(parser) => match parser.parse(bytes) {
                FastCombinatorResult::Failure => FastCombinatorResult::Success(0),
                FastCombinatorResult::Success(len) => FastCombinatorResult::Success(len),
                FastCombinatorResult::Incomplete(parser) => FastCombinatorResult::Incomplete(FastParser::Opt(Box::new(parser))),
            },
            FastCombinator::Repeat1(parser) => {
                FastCombinatorResult::Incomplete(FastParser::Repeat1Parser(parser.clone()))
            }
            FastCombinator::Eps => FastCombinatorResult::Success(0),
            FastCombinator::EatU8(u8set) => {
                if bytes.is_empty() {
                    return FastCombinatorResult::Incomplete(FastParser::EatU8Parser(*u8set));
                }
                if u8set.contains(bytes[0]) {
                    FastCombinatorResult::Success(1)
                } else {
                    FastCombinatorResult::Failure
                }
            }
            FastCombinator::EatByteStringChoiceFast(root) => {
                if bytes.is_empty() {
                    return FastCombinatorResult::Incomplete(FastParser::EatByteStringChoiceFastParser(root.clone(), root.clone()));
                }
                let (current_node, bytes_consumed, finish_reason) = root.next(bytes);
                match finish_reason {
                    FinishReason::Success => FastCombinatorResult::Success(bytes_consumed),
                    FinishReason::EndOfInput => FastCombinatorResult::Incomplete(FastParser::EatByteStringChoiceFastParser(root.clone(), Rc::new(current_node.clone()))),
                    FinishReason::Failure => FastCombinatorResult::Failure,
                }
            }
        }
    }
}

impl FastParserTrait for FastParser {
    fn parse(&mut self, bytes: &[u8]) -> FastParserResult {
        match self {
            FastParser::SeqParser(children, index) => {
                let mut total_len = 0;
                while *index < children.len() {
                    match children[*index].parse(&bytes[total_len..]) {
                        FastCombinatorResult::Success(len) => {
                            total_len += len;
                            *index += 1;
                        }
                        FastCombinatorResult::Failure => return FastParserResult::Failure,
                        FastCombinatorResult::Incomplete(parser) => {
                            *self = parser;
                            return FastParserResult::Incomplete;
                        }
                    }
                }
                FastParserResult::Success(total_len)
            }
            FastParser::ChoiceParser(children, index) => {
                while *index < children.len() {
                    match children[*index].parse(bytes) {
                        FastCombinatorResult::Failure => {
                            *index += 1;
                            continue;
                        }
                        FastCombinatorResult::Success(len) => return FastParserResult::Success(len),
                        FastCombinatorResult::Incomplete(parser) => {
                            *self = parser;
                            return FastParserResult::Incomplete(parser);
                        }
                    }
                }
                FastParserResult::Failure
            }
            FastParser::Repeat1Parser(parser) => {
                let mut total_len = 0;
                loop {
                    match parser.parse(&bytes[total_len..]) {
                        FastCombinatorResult::Success(len) => {
                            if len == 0 {
                                break;
                            }
                            total_len += len;
                        }
                        FastCombinatorResult::Failure => {
                            if total_len == 0 {
                                return FastParserResult::Failure;
                            } else {
                                break;
                            }
                        }
                        FastCombinatorResult::Incomplete(parser) => {
                            *self = parser;
                            return FastParserResult::Incomplete(parser);
                        }
                    }
                }
                FastParserResult::Success(total_len)
            }
            FastParser::EatByteStringChoiceFastParser(root, current_node) => {
                let (new_node, bytes_consumed, finish_reason) = current_node.next(bytes);
                match finish_reason {
                    FinishReason::Success => FastParserResult::Success(bytes_consumed),
                    FinishReason::EndOfInput => FastParserResult::Incomplete,
                    FinishReason::Failure => FastParserResult::Failure,
                }
            }
            FastParser::EatU8Parser(u8set) => {
                if bytes.is_empty() {
                    return FastParserResult::Incomplete;
                }
                if u8set.contains(bytes[0]) {
                    FastParserResult::Success(1)
                } else {
                    FastParserResult::Failure
                }
            }
        }
    }

    fn get_u8set(&self) -> U8Set {
        match self {
            FastParser::SeqParser(children, index) => {
                if *index >= children.len() {
                    return U8Set::none();
                }
                children[*index].get_u8set()
            }
            FastParser::ChoiceParser(children, index) => {
                let mut u8set = U8Set::none();
                for i in *index..children.len() {
                    u8set |= children[i].get_u8set();
                }
                u8set
            }
            FastParser::Opt(parser) => parser.get_u8set(),
            FastParser::Repeat1Parser(parser) => parser.get_u8set(),
            FastParser::EatByteStringChoiceFastParser(root, current_node) => current_node.valid_bytes.clone(),
            FastParser::EatU8Parser(u8set) => *u8set,
        }
    }
}

pub fn seq_fast(parsers: Vec<FastCombinator>) -> FastCombinator {
    FastCombinator::Seq(Rc::new(parsers))
}

pub fn choice_fast(parsers: Vec<FastCombinator>) -> FastCombinator {
    FastCombinator::Choice(Rc::new(parsers))
}

pub fn opt_fast(parser: FastCombinator) -> FastCombinator {
    FastCombinator::Opt(Box::new(parser))
}

pub fn repeat1_fast(parser: FastCombinator) -> FastCombinator {
    FastCombinator::Repeat1(Rc::new(parser))
}

pub fn eat_char_fast(c: char) -> FastCombinator {
    FastCombinator::EatU8(U8Set::from_char(c))
}

pub fn eat_byte_fast(byte: u8) -> FastCombinator {
    FastCombinator::EatU8(U8Set::from_byte(byte))
}

pub fn eat_char_negation_fast(c: char) -> FastCombinator {
    FastCombinator::EatU8(U8Set::from_char(c).complement())
}

pub fn eat_char_choice_fast(chars: &str) -> FastCombinator {
    FastCombinator::EatU8(U8Set::from_chars(chars))
}

pub fn eat_char_negation_choice_fast(chars: &str) -> FastCombinator {
    FastCombinator::EatU8(U8Set::from_chars(chars).complement())
}

pub fn eat_byte_range_fast(start: u8, end: u8) -> FastCombinator {
    FastCombinator::EatU8(U8Set::from_byte_range(start..=end))
}

pub fn eat_bytestring_choice_fast(bytestrings: Vec<Vec<u8>>) -> FastCombinator {
    FastCombinator::EatByteStringChoiceFast(Rc::new(bytestrings.into()))
}

pub fn eat_string_choice_fast(strings: &[&str]) -> FastCombinator {
    eat_bytestring_choice_fast(strings.into_iter().map(|s| s.as_bytes().to_vec()).collect())
}

pub fn eat_string_fast(s: &str) -> FastCombinator {
    let mut children = vec![];
    for c in s.bytes() {
        children.push(eat_byte_fast(c));
    }
    seq_fast(children)
}

pub fn repeat0_fast(parser: FastCombinator) -> FastCombinator {
    opt_fast(repeat1_fast(parser))
}

pub fn seprep1_fast(a: FastCombinator, b: FastCombinator) -> FastCombinator {
    seq_fast(vec![a.clone(), repeat0_fast(seq_fast(vec![b, a]))])
}

pub fn seprep0_fast(a: FastCombinator, b: FastCombinator) -> FastCombinator {
    opt_fast(seprep1_fast(a, b))
}

pub fn repeatn_fast(n: usize, parser: FastCombinator) -> FastCombinator {
    if n == 0 {
        return seq_fast(vec![]);
    }
    let mut parsers = Vec::new();
    for _ in 0..n {
        parsers.push(parser.clone());
    }
    seq_fast(parsers)
}

#[macro_export]
macro_rules! seq_fast {
    ($($x:expr),* $(,)?) => {
        $crate::seq_fast(vec![$($x),*])
    };
}

#[macro_export]
macro_rules! choice_fast {
    ($($x:expr),* $(,)?) => {
        $crate::choice_fast(vec![$($x),*])
    };
}