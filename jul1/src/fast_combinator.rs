use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::{Combinator, combinator, EatByteStringChoice, EatU8, eps, U8Set};
use crate::trie::{FinishReason, TrieNode};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FastParser {
    Seq(Vec<FastParser>),
    Choice(Vec<FastParser>),
    Opt(Box<FastParser>),
    Repeat1(Box<FastParser>),
    Eps,
    EatU8Parser(U8Set),
    EatByteStringChoiceFast(Rc<TrieNode>),
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
                // let all_next = root.all_next(bytes);
                // if root.all_next(bytes).0.len() > 1 {
                //     let s = String::from_utf8_lossy(bytes);
                //     panic!("Ambiguous parse: {:?}", s[..s.len().min(100)].to_string());
                // }
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
                crate::EatByteStringChoice { root: Rc::clone(root) }.into()
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

pub fn eat_char_negation_fast(c: char) -> FastParser {
    FastParser::EatU8Parser(U8Set::from_char(c).complement())
}

pub fn eat_char_choice_fast(chars: &str) -> FastParser {
    FastParser::EatU8Parser(U8Set::from_chars(chars))
}

pub fn eat_char_negation_choice_fast(chars: &str) -> FastParser {
    FastParser::EatU8Parser(U8Set::from_chars(chars).complement())
}

pub fn eat_byte_range_fast(start: u8, end: u8) -> FastParser {
    FastParser::EatU8Parser(U8Set::from_byte_range(start..=end))
}

pub fn eat_bytestring_choice_fast(bytestrings: Vec<Vec<u8>>) -> FastParser {
    FastParser::EatByteStringChoiceFast(Rc::new(bytestrings.into()))
}

pub fn eat_string_choice_fast(strings: &[&str]) -> FastParser {
    eat_bytestring_choice_fast(strings.into_iter().map(|s| s.as_bytes().to_vec()).collect())
}

pub fn eat_string_fast(s: &str) -> FastParser {
    let mut children = vec![];
    for c in s.bytes() {
        children.push(eat_byte_fast(c));
    }
    seq_fast(children)
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

pub fn repeatn_fast(n: usize, parser: FastParser) -> FastParser {
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
