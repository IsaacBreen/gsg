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
}

use std::collections::HashMap;
use std::fmt::Debug;

impl<T: Debug + Clone, E: Debug + Ord + Clone> TrieNode<T, E> {
    /// Merges another trie into this one. The `merge_value` function defines how to merge node values.
    fn merge(
        &mut self,
        other: &Arc<Mutex<TrieNode<T, E>>>,
        merge_value: &dyn Fn(&Option<T>, &Option<T>) -> Option<T>,
    ) {
        // Map from raw pointers of `other` nodes to `Arc<Mutex<TrieNode<T, E>>>` in `self`
        let mut node_map: HashMap<*const TrieNode<T, E>, Arc<Mutex<TrieNode<T, E>>>> =
            HashMap::new();

        // Stack for iterative traversal
        let mut stack: Vec<(Arc<Mutex<TrieNode<T, E>>>, Arc<Mutex<TrieNode<T, E>>>)> = Vec::new();
        // Push the roots onto the stack
        let self_arc = Arc::new(Mutex::new(self.clone()));
        stack.push((self_arc.clone(), other.clone()));

        while let Some((self_node_arc, other_node_arc)) = stack.pop() {
            let self_node_ref: &TrieNode<_, _> = &self_node_arc.lock().unwrap();
            let other_node_ref: &TrieNode<_, _> = &other_node_arc.lock().unwrap();

            let self_node_ptr = self_node_ref as *const TrieNode<T, E>;
            let other_node_ptr = other_node_ref as *const TrieNode<T, E>;

            // Check if we've already processed this node
            if let Some(existing_self_arc) = node_map.get(&other_node_ptr) {
                // If the current `self_node_arc` is different, we need to ensure they are the same
                if !Arc::ptr_eq(&self_node_arc, existing_self_arc) {
                    // Merge the nodes
                    Self::merge_node_arcs(
                        &self_node_arc,
                        existing_self_arc,
                        merge_value,
                    );
                }
                continue;
            }

            // Record the mapping
            node_map.insert(other_node_ptr, self_node_arc.clone());

            // Lock both nodes
            let mut self_node = self_node_arc.lock().unwrap();
            let other_node = other_node_arc.lock().unwrap();

            // Merge the values
            self_node.value = merge_value(&self_node.value, &other_node.value);

            // Merge the children
            for (edge_label, child_other_arc) in &other_node.children {
                // Get or create the corresponding child in `self`
                let child_self_arc = self_node
                    .children
                    .entry(edge_label.clone())
                    .or_insert_with(|| Arc::new(Mutex::new(TrieNode::new())));

                // Push the pair onto the stack for further processing
                stack.push((child_self_arc.clone(), child_other_arc.clone()));
            }
        }

        // Update the root of `self` after merging
        *self = Arc::try_unwrap(self_arc).unwrap().into_inner().unwrap();
    }

    /// Merges two nodes represented by their `Arc<Mutex<>>` wrappers.
    fn merge_node_arcs(
        self_node_arc: &Arc<Mutex<TrieNode<T, E>>>,
        existing_self_arc: &Arc<Mutex<TrieNode<T, E>>>,
        merge_value: &dyn Fn(&Option<T>, &Option<T>) -> Option<T>,
    ) {
        // Lock both nodes
        let mut self_node = self_node_arc.lock().unwrap();
        let existing_node = existing_self_arc.lock().unwrap();

        // Merge the values
        self_node.value = merge_value(&self_node.value, &existing_node.value);

        // Merge the children
        for (edge_label, child_existing_arc) in &existing_node.children {
            let child_self_arc = self_node
                .children
                .entry(edge_label.clone())
                .or_insert_with(|| Arc::new(Mutex::new(TrieNode::new())));

            // Recursively merge the children
            Self::merge_node_arcs(child_self_arc, child_existing_arc, merge_value);
        }
    }
}