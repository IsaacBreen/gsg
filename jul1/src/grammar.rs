use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::Combinator;

#[derive(Serialize, Deserialize)]
pub struct Grammar {
    pub root: Combinator,
    pub forward_refs: HashMap<usize, Combinator>,
}

impl Grammar {
    pub fn new(root: Combinator) -> Self {
        Grammar {
            root,
            forward_refs: HashMap::new(),
        }
    }

    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn deserialize(json: &str) -> Result<Self, serde_json::Error> {
        let mut grammar: Grammar = serde_json::from_str(json)?;
        grammar.resolve_forward_refs();
        Ok(grammar)
    }

    fn resolve_forward_refs(&mut self) {
        // Implement the logic to resolve forward references
        // This will involve traversing the Combinator tree and replacing ForwardRef nodes
        // with their actual Combinator values from the forward_refs HashMap
    }
}
