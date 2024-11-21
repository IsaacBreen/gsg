use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
struct TrieNode<T, E> {
    value: Option<T>,
    children: BTreeMap<E, Arc<Mutex<TrieNode<T, E>>>>,
}

impl<T, E> TrieNode<T, E> {
    fn new() -> Self {
        TrieNode {
            value: None,
            children: BTreeMap::new(),
        }
    }

    fn special_map<S, M, V>(&self, step: S, merge: M)
    where
        S: FnMut(&V, &E, &T) -> V,
        M: FnMut(Vec<V>) -> V,
        V: Clone,
    {
        todo!()
    }

    fn merge(&mut self, other: TrieNode<T, E>) {
        todo!()
    }
}