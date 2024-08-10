use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::{Combinator, U8Set, VecX};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum FastParser {
    Seq(Vec<FastParser>),
    Choice(Vec<FastParser>),
    Repeat1(Box<FastParser>),
    EatU8Parser(U8Set),
    EatByteStringChoiceFast(Rc<TrieNode>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FastParserResult {
    Success(usize),
    Failure,
    Incomplete,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct TrieNode {
    valid_bytes: U8Set,
    is_end: bool,
    children: Vec<Option<Rc<TrieNode>>>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
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
                node.children[byte as usize] = Some(Rc::new(TrieNode::new()));
            }
            node = Rc::make_mut(node.children[byte as usize].as_mut().unwrap());
        }
        node.is_end = true;
    }
}

impl FastParser {
    pub fn parse(&self, bytes: &[u8]) -> FastParserResult {
        match self {
            FastParser::Seq(children) => {
                let mut total_len = 0;
                for child in children {
                    match child.parse(&bytes[total_len..]) {
                        FastParserResult::Success(len) => total_len += len,
                        FastParserResult::Failure => return FastParserResult::Failure,
                        FastParserResult::Incomplete => return FastParserResult::Incomplete,
                    }
                }
                FastParserResult::Success(total_len)
            }
            FastParser::Choice(children) => {
                for child in children {
                    match child.parse(bytes) {
                        FastParserResult::Success(len) => return FastParserResult::Success(len),
                        FastParserResult::Failure => continue,
                        FastParserResult::Incomplete => return FastParserResult::Incomplete,
                    }
                }
                FastParserResult::Failure
            }
            FastParser::Repeat1(parser) => {
                let mut total_len = 0;
                loop {
                    match parser.parse(&bytes[total_len..]) {
                        FastParserResult::Success(len) => {
                            if len == 0 { break; }
                            total_len += len;
                        }
                        FastParserResult::Failure => {
                            if total_len == 0 { return FastParserResult::Failure; }
                            break;
                        }
                        FastParserResult::Incomplete => return FastParserResult::Incomplete,
                    }
                }
                FastParserResult::Success(total_len)
            }
            FastParser::EatU8Parser(u8set) => {
                if bytes.is_empty() { return FastParserResult::Incomplete; }
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
                        if let Some(next_node) = &current_node.children[byte as usize] {
                            current_node = next_node;
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
                let all_children: VecX<Combinator> = children.iter().map(|c| c.slow()).collect();
                crate::_seq(all_children)
            }
            FastParser::Choice(children) => {
                let all_children: VecX<Combinator> = children.iter().map(|c| c.slow()).collect();
                crate::_choice(all_children)
            }
            FastParser::Repeat1(parser) => crate::repeat1(parser.slow()).into(),
            FastParser::EatU8Parser(u8set) => crate::EatU8 { u8set: *u8set }.into(),
            FastParser::EatByteStringChoiceFast(root) => {
                crate::EatByteStringChoice { root: root.clone() }.into()
            }
        }
    }

    pub(crate) fn optimize(self) -> Self {
        match self {
            FastParser::Seq(children) => {
                let mut new_children = Vec::new();
                for child in children {
                    match child.optimize() {
                        FastParser::Seq(mut sub_children) => new_children.append(&mut sub_children),
                        other => new_children.push(other),
                    }
                }
                if new_children.len() == 1 {
                    new_children.pop().unwrap()
                } else {
                    FastParser::Seq(new_children)
                }
            }
            FastParser::Choice(children) => {
                let mut new_children = Vec::new();
                for child in children {
                    match child.optimize() {
                        FastParser::Choice(mut sub_children) => new_children.append(&mut sub_children),
                        other => new_children.push(other),
                    }
                }
                if new_children.len() == 1 {
                    new_children.pop().unwrap()
                } else {
                    FastParser::Choice(new_children)
                }
            }
            FastParser::Repeat1(parser) => FastParser::Repeat1(Box::new(parser.optimize())),
            other => other,
        }
    }

    fn compile(self) -> Self {
        let optimized = self.optimize();
        if let Some(trie) = optimized.to_trie() {
            FastParser::EatByteStringChoiceFast(Rc::new(trie))
        } else {
            optimized
        }
    }

    fn to_trie(&self) -> Option<TrieNode> {
        match self {
            FastParser::EatU8Parser(u8set) => {
                let mut trie = TrieNode::new();
                for byte in u8set.iter() {
                    trie.insert(&[byte]);
                }
                Some(trie)
            }
            FastParser::EatByteStringChoiceFast(root) => Some((*root).clone()),
            FastParser::Seq(children) => {
                let mut trie = TrieNode::new();
                let mut current_path = Vec::new();
                Self::build_trie_from_seq(&mut trie, children, &mut current_path, 0)
                    .then_some(trie)
            }
            FastParser::Choice(children) => {
                let mut trie = TrieNode::new();
                for child in children {
                    if let Some(child_trie) = child.to_trie() {
                        Self::merge_tries(&mut trie, &child_trie);
                    } else {
                        return None;
                    }
                }
                Some(trie)
            }
            FastParser::Repeat1(_) => None,
        }
    }

    fn build_trie_from_seq(trie: &mut TrieNode, children: &[FastParser], current_path: &mut Vec<u8>, index: usize) -> bool {
        if index == children.len() {
            trie.insert(current_path);
            return true;
        }

        match &children[index] {
            FastParser::EatU8Parser(u8set) => {
                let mut success = false;
                for byte in u8set.iter() {
                    current_path.push(byte);
                    success |= Self::build_trie_from_seq(trie, children, current_path, index + 1);
                    current_path.pop();
                }
                success
            }
            FastParser::EatByteStringChoiceFast(root) => {
                Self::build_trie_from_node(trie, root, children, current_path, index)
            }
            _ => false,
        }
    }

    fn build_trie_from_node(trie: &mut TrieNode, node: &TrieNode, children: &[FastParser], current_path: &mut Vec<u8>, index: usize) -> bool {
        let mut success = false;
        if node.is_end {
            success |= Self::build_trie_from_seq(trie, children, current_path, index + 1);
        }
        for (byte, child) in node.children.iter().enumerate().filter_map(|(i, c)| c.as_ref().map(|c| (i as u8, c))) {
            current_path.push(byte);
            success |= Self::build_trie_from_node(trie, child, children, current_path, index);
            current_path.pop();
        }
        success
    }

    fn merge_tries(target: &mut TrieNode, source: &TrieNode) {
        if source.is_end {
            target.is_end = true;
        }
        target.valid_bytes = target.valid_bytes.union(source.valid_bytes);
        for (byte, child) in source.children.iter().enumerate().filter_map(|(i, c)| c.as_ref().map(|c| (i, c))) {
            if let Some(target_child) = &mut target.children[byte] {
                let target_child = Rc::make_mut(target_child);
                Self::merge_tries(target_child, child);
            } else {
                target.children[byte] = Some(child.clone());
            }
        }
    }
}

pub fn seq_fast(parsers: Vec<FastParser>) -> FastParser {
    FastParser::Seq(parsers).compile()
}

pub fn choice_fast(parsers: Vec<FastParser>) -> FastParser {
    FastParser::Choice(parsers).compile()
}

pub fn opt_fast(parser: FastParser) -> FastParser {
    choice_fast(vec![parser, FastParser::Seq(vec![])])
}

pub fn repeat1_fast(parser: FastParser) -> FastParser {
    FastParser::Repeat1(Box::new(parser.compile()))
}

pub fn eat_char_fast(c: char) -> FastParser {
    FastParser::EatU8Parser(U8Set::from_char(c))
}

pub fn eat_bytestring_choice_fast(bytestrings: Vec<Vec<u8>>) -> FastParser {
    let mut root = TrieNode::new();
    for bytestring in bytestrings {
        root.insert(&bytestring);
    }
    FastParser::EatByteStringChoiceFast(Rc::new(root))
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

impl Debug for FastParser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FastParser::Seq(children) => f.debug_tuple("Seq").field(children).finish(),
            FastParser::Choice(children) => f.debug_tuple("Choice").field(children).finish(),
            FastParser::Repeat1(parser) => f.debug_tuple("Repeat1").field(parser).finish(),
            FastParser::EatU8Parser(u8set) => f.debug_tuple("EatU8Parser").field(u8set).finish(),
            FastParser::EatByteStringChoiceFast(_) => f.debug_struct("EatByteStringChoiceFast").finish_non_exhaustive(),
        }
    }
}