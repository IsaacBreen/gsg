use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::{Combinator, U8Set};
use crate::trie::{BuildTrieNode, TrieNode};

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
                let mut current_node = root;
                let mut bytes_consumed = 0;

                for &byte in bytes {
                    if current_node.valid_bytes.contains(byte) {
                        let child_index =
                            current_node.valid_bytes.bitset.count_bits_before(byte) as usize;
                        if child_index < current_node.children.len() {
                            current_node = &current_node.children[child_index];
                            bytes_consumed += 1;
                            if current_node.is_end {
                                return FastParserResult::Success(bytes_consumed);
                            }
                        } else {
                            return FastParserResult::Failure;
                        }
                    } else {
                        return FastParserResult::Failure;
                    }
                }

                if bytes_consumed > 0 && current_node.is_end {
                    FastParserResult::Success(bytes_consumed)
                } else {
                    FastParserResult::Incomplete
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

    pub(crate) fn optimize(self) -> FastParser {
        match self {
            FastParser::Seq(children) => {
                let children = children.into_iter().map(|c| c.optimize()).collect();
                flatten_seq(FastParser::Seq(children))
            }
            FastParser::Choice(children) => {
                let children = children.into_iter().map(|c| c.optimize()).collect();
                flatten_choice(FastParser::Choice(children))
            }
            FastParser::Opt(parser) => {
                let parser = parser.optimize();
                match parser {
                    FastParser::Eps => FastParser::Eps, // Opt(Eps) = Eps
                    _ => flatten_choice(FastParser::Choice(vec![parser, FastParser::Eps])),
                }
            }
            FastParser::Repeat1(parser) => FastParser::Repeat1(Box::new(parser.optimize())),
            FastParser::Eps => FastParser::Eps,
            FastParser::EatU8Parser(_) => self,
            FastParser::EatByteStringChoiceFast(_) => self,
        }
    }
}

fn flatten_seq(parser: FastParser) -> FastParser {
    match parser {
        FastParser::Seq(children) => {
            let mut new_children = Vec::new();
            for child in children {
                match child {
                    FastParser::Seq(grandchildren) => new_children.extend(grandchildren),
                    FastParser::Eps => {} // Skip epsilon in sequences
                    _ => new_children.push(child),
                }
            }
            if new_children.len() == 1 {
                new_children.pop().unwrap()
            } else {
                FastParser::Seq(new_children)
            }
        }
        _ => parser,
    }
}

fn flatten_choice(parser: FastParser) -> FastParser {
    match parser {
        FastParser::Choice(children) => {
            let mut new_children = Vec::new();
            for child in children {
                match child {
                    FastParser::Choice(grandchildren) => new_children.extend(grandchildren),
                    _ => new_children.push(child),
                }
            }
            if new_children.len() == 1 {
                new_children.pop().unwrap()
            } else {
                FastParser::Choice(new_children)
            }
        }
        _ => parser,
    }
}

fn to_trie(parser: FastParser) -> Option<FastParser> {
    match parser {
        FastParser::Seq(children) => {
            let mut bytestrings = vec![Vec::new()];
            for child in children {
                match child {
                    FastParser::EatU8Parser(u8set) => {
                        for bytestring in &mut bytestrings {
                            for byte in u8set.iter() {
                                bytestring.push(byte);
                            }
                        }
                    }
                    FastParser::EatByteStringChoiceFast(trie) => {
                        let mut new_bytestrings = Vec::new();
                        for bytestring in bytestrings {
                            for child in &trie.children {
                                let mut new_bytestring = bytestring.clone();
                                new_bytestring.extend(child.valid_bytes.iter());
                                new_bytestrings.push(new_bytestring);
                            }
                        }
                        bytestrings = new_bytestrings;
                    }
                    _ => return None, // Not a trie-compatible parser
                }
            }
            Some(eat_bytestring_choice_fast(bytestrings))
        }
        FastParser::Choice(children) => {
            let mut bytestrings = Vec::new();
            for child in children {
                match to_trie(child) {
                    Some(FastParser::EatByteStringChoiceFast(trie)) => {
                        for child in &trie.children {
                            bytestrings.push(child.valid_bytes.iter().collect());
                        }
                    }
                    _ => return None, // Not a trie-compatible parser
                }
            }
            Some(eat_bytestring_choice_fast(bytestrings))
        }
        FastParser::EatU8Parser(u8set) => {
            let bytestrings: Vec<Vec<u8>> = u8set.iter().map(|b| vec![b]).collect();
            Some(eat_bytestring_choice_fast(bytestrings))
        }
        FastParser::EatByteStringChoiceFast(_) => Some(parser),
        _ => None, // Not a trie-compatible parser
    }
}

pub fn seq_fast(parsers: Vec<FastParser>) -> FastParser {
    FastParser::Seq(parsers).optimize()
}

pub fn choice_fast(parsers: Vec<FastParser>) -> FastParser {
    FastParser::Choice(parsers).optimize()
}

pub fn opt_fast(parser: FastParser) -> FastParser {
    FastParser::Opt(Box::new(parser)).optimize()
}

pub fn repeat1_fast(parser: FastParser) -> FastParser {
    FastParser::Repeat1(Box::new(parser)).optimize()
}

pub fn eat_char_fast(c: char) -> FastParser {
    FastParser::EatU8Parser(U8Set::from_char(c)).optimize()
}

pub fn eat_bytestring_choice_fast(bytestrings: Vec<Vec<u8>>) -> FastParser {
    let mut build_root = BuildTrieNode::new();
    for bytestring in bytestrings {
        build_root.insert(&bytestring);
    }
    let root = build_root.to_optimized_trie_node();
    FastParser::EatByteStringChoiceFast(root).optimize()
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