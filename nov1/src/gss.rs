use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GSSNode<T> {
    value: T,
    predecessors: Vec<Rc<GSSNode<T>>>,
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
            if let Ok(mut inner_node) = Rc::try_unwrap(node) {
                cur_nodes.append(&mut inner_node.predecessors);
            }
        }
    }
}

pub trait GSSTrait<T: Clone> {
    type Peek;
    fn peek(&self) -> Self::Peek;
    fn push(&self, value: T) -> GSSNode<T>;
    fn pop(&self) -> Vec<Rc<GSSNode<T>>>;
    fn popn(&self, n: usize) -> Vec<Rc<GSSNode<T>>>;
}

impl<T: Clone> GSSTrait<T> for GSSNode<T> {
    type Peek = &T;

    fn peek(&self) -> Self::Peek {
        &self.value
    }

    fn push(&self, value: T) -> GSSNode<T> {
        Rc::new(self.clone()).push(value)
    }

    fn pop(&self) -> Vec<Rc<GSSNode<T>>> {
        self.predecessors.clone()
    }

    fn popn(&self, n: usize) -> Vec<Rc<GSSNode<T>>> {
        if n == 0 {
            return vec![Rc::new(self.clone())];
        }
        let mut nodes = Vec::new();
        for popped in self.pop() {
            nodes.extend(popped.popn(n - 1));
        }
        nodes
    }
}



impl<T: Clone> GSSTrait<T> for Rc<GSSNode<T>> {
    type Peek = &T;

    fn peek(&self) -> Self::Peek {
        &self.value
    }

    fn push(&self, value: T) -> GSSNode<T> {
        let mut new_node = GSSNode::new(value);
        new_node.predecessors.push(self.clone());
        new_node
    }

    fn pop(&self) -> Vec<Rc<GSSNode<T>>> {
        self.predecessors.clone()
    }

    fn popn(&self, n: usize) -> Vec<Rc<GSSNode<T>>> {
        if n == 0 {
            return vec![self.clone()];
        }
        let mut nodes = Vec::new();
        for popped in self.pop() {
            nodes.extend(popped.popn(n - 1));
        }
        nodes
    }
}

impl<T: Clone> GSSTrait<T> for Option<Rc<GSSNode<T>>> {
    type Peek = Option<&T>;

    fn peek(&self) -> Self::Peek {
        self.as_ref().map(|node| node.peek())
    }

    fn push(&self, value: T) -> GSSNode<T> {
        self.clone().map(|node| node.push(value.clone())).unwrap_or_else(|| GSSNode::new(value))
    }

    fn pop(&self) -> Vec<Rc<GSSNode<T>>> {
        self.as_ref().map(|node| node.pop()).unwrap_or_default()
    }

    fn popn(&self, n: usize) -> Vec<Rc<GSSNode<T>>> {
        self.as_ref().map(|node| node.popn(n)).unwrap_or_default()
    }
}


impl<T: Clone> GSSTrait<T> for Option<GSSNode<T>> {
    type Peek = Option<&T>;

    fn peek(&self) -> Self::Peek {
        self.as_ref().map(|node| node.peek())
    }

    fn push(&self, value: T) -> GSSNode<T> {
        self.clone().map(|node| node.push(value)).unwrap_or_else(|| GSSNode::new(value))
    }

    fn pop(&self) -> Vec<Rc<GSSNode<T>>> {
        self.as_ref().map(|node| node.pop()).unwrap_or_default()
    }

    fn popn(&self, n: usize) -> Vec<Rc<GSSNode<T>>> {
        self.as_ref().map(|node| node.popn(n)).unwrap_or_default()
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

    pub fn merge(&mut self, mut other: Self) where T: PartialEq {
        assert!(self.value == other.value);
        self.predecessors.extend(other.predecessors.drain(..));
    }
}