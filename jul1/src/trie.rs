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