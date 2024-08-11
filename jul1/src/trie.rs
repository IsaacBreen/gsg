use std::cmp::PartialEq;
use std::fmt::{Debug, Formatter};

use crate::U8Set;

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct TrieNode {
    pub(crate) valid_bytes: U8Set,
    pub(crate) is_end: bool,
    pub(crate) children: Vec<TrieNode>,
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

    pub fn eat_all(&self, bytes: &[u8]) -> Option<&TrieNode> {
        let mut current_node = self;
        for &byte in bytes {
            if current_node.valid_bytes.contains(byte) {
                let child_index = current_node.valid_bytes.bitset.count_bits_before(byte) as usize;
                current_node = &current_node.children[child_index];
            } else {
                return None;
            }
        }
        Some(current_node)
    }

    pub fn get_indices(&self, bytes: &[u8]) -> (Vec<usize>, Option<&TrieNode>) {
        let mut indices = vec![];
        let mut current_node = self;
        let mut i = 0;
        while let (node, di, reason) = current_node.next(&bytes[i..]) {
            i += di;
            match reason {
                FinishReason::Failure => {
                    return (indices, None);
                }
                FinishReason::EndOfInput => {
                    return (indices, Some(node));
                }
                FinishReason::Success => {
                    indices.push(i);
                    current_node = node;
                }
            }
        }
        (indices, Some(current_node))
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
                node.children.push(TrieNode::new());
            }
            debug_assert_eq!(node.children.len(), node.valid_bytes.len());
            node = node.children.last_mut().unwrap();
        }
        node.is_end = true;
   }
}

impl From<Vec<Vec<u8>>> for TrieNode {
    fn from(mut bytestrings: Vec<Vec<u8>>) -> Self {
        // Sort lexicographically
        bytestrings.sort_unstable();
        let mut root = TrieNode::new();
        for bytestring in bytestrings {
            root.insert_in_order(&bytestring);
        }
        root
    }
}
