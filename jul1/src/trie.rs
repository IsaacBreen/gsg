use std::cmp::PartialEq;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::U8Set;

#[derive(Clone, PartialEq, Eq)]
pub struct BuildTrieNode {
    valid_bytes: U8Set,
    is_end: bool,
    children: Vec<Option<Rc<BuildTrieNode>>>,
}

impl BuildTrieNode {
    pub(crate) fn new() -> Self {
        BuildTrieNode {
            valid_bytes: U8Set::none(),
            is_end: false,
            children: vec![None; 256],
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
    pub(crate) children: Vec<Rc<TrieNode>>,
}

impl Debug for TrieNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TrieNode")
            .field("valid_bytes", &self.valid_bytes)
            .field("is_end", &self.is_end)
            .finish_non_exhaustive()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum FinishReason {
    Success,
    EndOfInput,
    Failure,
}

impl TrieNode {
    pub fn next(&self, bytes: &[u8]) -> (&TrieNode, usize, FinishReason) {
        let mut current_node = self;
        for (i, &byte) in bytes.iter().enumerate() {
            if current_node.valid_bytes.contains(byte) {
                let child_index = current_node.valid_bytes.bitset.count_bits_before(byte) as usize;
                current_node = &current_node.children[child_index];
                if current_node.is_end {
                    return (current_node, i + 1, FinishReason::Success);
                }
            } else {
                return (current_node, i, FinishReason::Failure);
            }
        }
        (current_node, bytes.len(), FinishReason::EndOfInput)
    }

    pub fn all_next(&self, bytes: &[u8]) -> (Vec<(&TrieNode, usize)>, (&TrieNode, usize, FinishReason)) {
        let mut results = vec![];
        let mut current_node = self;
        let mut i = 0;
        loop {
            let (node, di, reason) = current_node.next(&bytes[i..]);
            i += di;
            match reason {
                FinishReason::EndOfInput | FinishReason::Failure => {
                    return (results, (node, i, reason));
                }
                FinishReason::Success => {
                    results.push((node, i));
                    current_node = node;
                }
            }
        }
    }

    pub fn is_end(&self) -> bool {
        self.is_end
    }

    pub fn is_absolute_end(&self) -> bool {
        self.is_end && self.valid_bytes.is_empty()
    }
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            valid_bytes: U8Set::none(),
            is_end: false,
            children: vec![],
        }
    }

    fn insert_in_order(&mut self, bytestring: &[u8]) {
        let mut node = self;
        for &byte in bytestring {
            if node.valid_bytes.insert(byte) {
                node.children.push(Rc::new(TrieNode::new()));
            }
            assert_eq!(node.children.len(), node.valid_bytes.len());
            node = Rc::make_mut(node.children.last_mut().unwrap());
        }
        node.is_end = true;
   }
}

impl From<Vec<Vec<u8>>> for TrieNode {
    fn from(mut bytestrings: Vec<Vec<u8>>) -> Self {
        // Sort
        bytestrings.sort();
        let mut root = TrieNode::new();
        for bytestring in bytestrings {
            root.insert_in_order(&bytestring);
        }
        root
    }
}