use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

/// Generalized Suffix Stack (GSS) Node representing a stack-like structure
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GSSNode<T> {
    /// The value stored in the current node
    value: T,
    /// Predecessor nodes in the stack
    predecessors: Vec<Arc<GSSNode<T>>>,
}

impl<T> GSSNode<T> {
    /// Creates a new GSS node with a single value
    pub fn new(value: T) -> Self {
        Self {
            value,
            predecessors: Vec::new(),
        }
    }

    /// Creates a GSS node from an iterator of values
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();
        let mut root = Self::new(iter.next().unwrap());
        for value in iter {
            root = root.push(value);
        }
        root
    }

    /// Pushes a new value onto the GSS node, creating a new node
    pub fn push(self, value: T) -> Self {
        let mut new_node = Self::new(value);
        new_node.predecessors.push(Arc::new(self));
        new_node
    }

    /// Returns all predecessor nodes
    pub fn pop(&self) -> Vec<Arc<Self>> {
        self.predecessors.clone()
    }

    /// Returns predecessor nodes up to a specified depth
    pub fn popn(&self, n: usize) -> Vec<Arc<Self>>
    where
        T: Clone,
    {
        if n == 0 {
            return vec![Arc::new(self.clone())];
        }
        let mut nodes = Vec::new();
        for popped in self.pop() {
            nodes.extend(popped.popn(n - 1));
        }
        nodes
    }

    /// Returns the current node's value
    pub fn peek(&self) -> &T {
        &self.value
    }

    /// Flattens the GSS node into all possible paths
    pub fn flatten(&self) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        stack.push((self, Vec::new()));
        while let Some((node, mut path)) = stack.pop() {
            path.push(node.value.clone());
            if node.predecessors.is_empty() {
                result.push(path);
            } else {
                for predecessor in &node.predecessors {
                    stack.push((predecessor, path.clone()));
                }
            }
        }
        result
    }

    /// Flattens multiple GSS nodes into their paths
    pub fn flatten_bulk(nodes: &[Self]) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        nodes.iter().flat_map(|node| node.flatten()).collect()
    }

    /// Merges two GSS nodes with the same value
    pub fn merge(&mut self, mut other: Self)
    where
        T: PartialEq,
    {
        assert!(self.value == other.value);
        self.predecessors.append(&mut other.predecessors);
    }

    /// Maps the GSS node's values using a transformation function
    pub fn map<F, U>(&self, f: F) -> GSSNode<U>
    where
        F: Copy + Fn(&T) -> U,
    {
        GSSNode {
            value: f(&self.value),
            predecessors: self.predecessors.clone().into_iter()
                .map(|node| Arc::new(node.map(f)))
                .collect(),
        }
    }
}

impl<T> Drop for GSSNode<T> {
    fn drop(&mut self) {
        let mut cur_nodes = std::mem::take(&mut self.predecessors);
        while let Some(node) = cur_nodes.pop() {
            if let Ok(mut inner_node) = Arc::try_unwrap(node) {
                cur_nodes.append(&mut inner_node.predecessors);
            }
        }
    }
}

// Trait and implementation details remain the same as in the original file
