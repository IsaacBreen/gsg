use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GSSNode<T> {
    value: T,
    predecessors: Vec<Rc<GSSNode<T>>>,
}

impl<T> GSSNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            predecessors: Vec::new(),
        }
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        <I as IntoIterator>::IntoIter: DoubleEndedIterator,
    {
        let mut iter = iter.into_iter();
        let mut root = Self::new(iter.next().unwrap());
        for value in iter {
            root = root.push(value);
        }
        root
    }

    pub fn push(self, value: T) -> Self {
        let mut new_node = Self::new(value);
        new_node.predecessors.push(Rc::new(self));
        new_node
    }

    pub fn pop(&self) -> Vec<Rc<Self>> {
        self.predecessors.clone()
    }

    pub fn popn(&self, n: usize) -> Vec<Rc<Self>>
    where
        T: Clone,
    {
        if n == 0 {
            return vec![Rc::new(self.clone())];
        }
        let mut nodes = Vec::new();
        for popped in self.pop() {
            nodes.extend(popped.popn(n - 1));
        }
        nodes
    }

    pub fn peek(&self) -> &T {
        &self.value
    }

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

    pub fn merge(&mut self, mut other: Self)
    where
        T: PartialEq,
    {
        assert!(self.value == other.value);
        self.predecessors.append(&mut other.predecessors);
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
    type Peek<'a> where T: 'a, Self: 'a;
    fn peek(&self) -> Self::Peek<'_>;
    fn push(&self, value: T) -> GSSNode<T>;
    fn pop(&self) -> Vec<Rc<GSSNode<T>>>;
    fn popn(&self, n: usize) -> Vec<Rc<GSSNode<T>>>;
}

impl<T: Clone> GSSTrait<T> for GSSNode<T> {
    type Peek<'a> = &'a T where T: 'a;

    fn peek(&self) -> Self::Peek<'_> {
        &self.value
    }

    fn push(&self, value: T) -> GSSNode<T> {
        let mut new_node = GSSNode::new(value);
        new_node.predecessors.push(Rc::new(self.clone()));
        new_node
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
    type Peek<'a> = &'a T where T: 'a;

    fn peek(&self) -> Self::Peek<'_> {
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
    type Peek<'a> = Option<&'a T> where T: 'a;

    fn peek(&self) -> Self::Peek<'_> {
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
    type Peek<'a> = Option<&'a T> where T: 'a;

    fn peek(&self) -> Self::Peek<'_> {
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