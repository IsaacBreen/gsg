use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GSSNode<T> {
    value: T,
    predecessors: Vec<Arc<GSSNode<T>>>,
}

impl<T> GSSNode<T> {
    pub(crate) fn new(value: T) -> Self {
        GSSNode {
            value,
            predecessors: Vec::new(),
        }
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> GSSNode<T> where <I as IntoIterator>::IntoIter: DoubleEndedIterator {
        let mut iter = iter.into_iter();
        let mut root = GSSNode::new(iter.next().unwrap());
        for value in iter {
            root = root.push(value);
        }
        root
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

pub trait GSSTrait<T> {
    fn peek(&self) -> &T;
    fn push(&self, value: T) -> GSSNode<T>;
    fn pop(&self) -> Vec<Self> where Self: Sized;
    fn popn(&self, n: usize) -> Vec<Self> where Self: Sized + Clone {
        let mut nodes = vec![self.clone()];
        for _ in 0..n {
            let mut new_nodes = Vec::new();
            for node in nodes {
                new_nodes.extend(node.pop());
            }
            nodes = new_nodes.clone();
        }
        nodes
    }
}

impl<T> GSSTrait<T> for Arc<GSSNode<T>> {
    fn peek(&self) -> &T {
        &self.value
    }

    fn push(&self, value: T) -> GSSNode<T> {
        let mut new_node = GSSNode::new(value);
        new_node.predecessors.push(self.clone());
        new_node
    }

    fn pop(&self) -> Vec<Self> {
        self.predecessors.clone()
    }
}

impl<T> GSSNode<T> {
    pub fn peek(&self) -> &T {
        &self.value
    }

    pub fn push(self, value: T) -> GSSNode<T> {
        Arc::new(self).push(value)
    }

    pub fn pop(&self) -> Vec<Arc<Self>> {
        self.predecessors.clone()
    }

    pub fn popn(&self, n: usize) -> Vec<Arc<Self>> where T: Clone {
        if n == 0 {
            return vec![Arc::new(self.clone())];
        }
        let mut nodes = Vec::new();
        for popped in self.pop() {
            nodes.extend(popped.popn(n - 1));
        }
        nodes
    }

    pub fn merge(&mut self, mut other: Self) where T: PartialEq {
        assert!(self.value == other.value);
        self.predecessors.extend(other.predecessors.drain(..));
    }
}

impl<T: Clone> GSSNode<T> {
    pub fn flatten(&self) -> Vec<Vec<T>> {
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
}