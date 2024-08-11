use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::{Combinator, U8Set};
use crate::trie::{BuildTrieNode, FinishReason, TrieNode};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FastParser {
    Seq(Vec<FastParser>),
    Choice(Vec<FastParser>),
    Opt(Box<FastParser>),
    Repeat1(Box<FastParser>),
    Eps,
    EatU8Parser(U8Set),
    EatByteStringChoiceFast(TrieNode),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FastParserResult {
    Success(usize),
    Failure,
    Incomplete,
}

impl FastParser {
    pub fn parse(&self, bytes: &[u8]) -> FastParserResult {
        match self {
            FastParser::Seq(children) => {
                let mut total_len = 0;
                for child in children {
                    match child.parse(&bytes[total_len..]) {
                        FastParserResult::Success(len) => {
                            total_len += len;
                        }
                        x => return x,
                    }
                }
                FastParserResult::Success(total_len)
            }
            FastParser::Choice(children) => {
                for child in children {
                    match child.parse(bytes) {
                        FastParserResult::Failure => continue,
                        x => return x,
                    }
                }
                FastParserResult::Failure
            }
            FastParser::Opt(parser) => match parser.parse(bytes) {
                FastParserResult::Failure => FastParserResult::Success(0),
                x => x,
            },
            FastParser::Repeat1(parser) => {
                let mut total_len = 0;
                loop {
                    match parser.parse(&bytes[total_len..]) {
                        FastParserResult::Success(len) => {
                            if len == 0 {
                                break;
                            }
                            total_len += len;
                        }
                        FastParserResult::Failure => {
                            if total_len == 0 {
                                return FastParserResult::Failure;
                            } else {
                                break;
                            }
                        }
                        FastParserResult::Incomplete => return FastParserResult::Incomplete,
                    }
                }
                FastParserResult::Success(total_len)
            }
            FastParser::Eps => FastParserResult::Success(0),
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
            FastParser::EatByteStringChoiceFast(root) => {
                let (current_node, bytes_consumed, finish_reason) = root.next(bytes);
                if root.all_next(bytes).0.is_empty() {
                    let s = String::from_utf8_lossy(bytes);
                    panic!("Ambiguous parse: {:?}", s[..s.len().min(100)].to_string());
                }
                match finish_reason {
                    FinishReason::Success => FastParserResult::Success(bytes_consumed),
                    FinishReason::EndOfInput => FastParserResult::Incomplete,
                    FinishReason::Failure => FastParserResult::Failure,
                }
            }
        }
    }

    pub fn slow(&self) -> Combinator {
        match self {
            FastParser::Seq(children) => {
                let mut all_children: crate::VecX<Combinator> = crate::vecx![];
                for child in children {
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
            FastParser::Choice(children) => {
                let mut all_children: crate::VecX<Combinator> = crate::vecx![];
                for child in children {
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
            FastParser::Opt(parser) => crate::opt(parser.slow()).into(),
            FastParser::Repeat1(parser) => crate::repeat1(parser.slow()).into(),
            FastParser::Eps => crate::eps().into(),
            FastParser::EatU8Parser(u8set) => crate::EatU8 { u8set: *u8set }.into(),
            FastParser::EatByteStringChoiceFast(root) => {
                crate::EatByteStringChoice { root: Rc::new(root.clone()) }.into()
            }
        }
    }
}

pub fn seq_fast(parsers: Vec<FastParser>) -> FastParser {
    FastParser::Seq(parsers)
}

pub fn choice_fast(parsers: Vec<FastParser>) -> FastParser {
    FastParser::Choice(parsers)
}

pub fn opt_fast(parser: FastParser) -> FastParser {
    FastParser::Opt(Box::new(parser))
}

pub fn repeat1_fast(parser: FastParser) -> FastParser {
    FastParser::Repeat1(Box::new(parser))
}

pub fn eat_char_fast(c: char) -> FastParser {
    FastParser::EatU8Parser(U8Set::from_char(c))
}

pub fn eat_byte_fast(byte: u8) -> FastParser {
    FastParser::EatU8Parser(U8Set::from_byte(byte))
}

pub fn eat_char_choice_fast(chars: &str) -> FastParser {
    FastParser::EatU8Parser(U8Set::from_chars(chars))
}

pub fn eat_bytestring_choice_fast(bytestrings: Vec<Vec<u8>>) -> FastParser {
    let mut build_root = BuildTrieNode::new();
    for bytestring in bytestrings {
        build_root.insert(&bytestring);
    }
    let root = build_root.to_optimized_trie_node();
    FastParser::EatByteStringChoiceFast(root)
}

pub fn repeat0_fast(parser: FastParser) -> FastParser {
    opt_fast(repeat1_fast(parser))
}

pub fn seprep1_fast(a: FastParser, b: FastParser) -> FastParser {
    seq_fast(vec![a.clone(), repeat0_fast(seq_fast(vec![b, a]))])
}

pub fn seprep0_fast(a: FastParser, b: FastParser) -> FastParser {
    opt_fast(seprep1_fast(a, b))
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