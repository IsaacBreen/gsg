use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};

// TrieNode definition
#[derive(Debug)]
struct TrieNode<T, E> {
    value: Option<T>,
    // Children are stored in an Arc to allow shared ownership
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

impl<T: Clone + Debug + PartialEq, E: Clone + Debug + Ord + PartialEq> TrieNode<T, E> {
    /// Merges another trie into this one. The `merge_value` function defines how to merge node values.
    fn merge(
        self,
        other: &Arc<Mutex<TrieNode<T, E>>>,
        merge_value: &dyn Fn(&Option<T>, &Option<T>) -> Option<T>,
    ) -> Arc<Mutex<TrieNode<T, E>>> {
        use std::ptr::NonNull;

        // Map from raw pointers of `other` nodes to `Arc<Mutex<TrieNode<T, E>>>` in `self`
        let mut node_map: HashMap<NonNull<TrieNode<T, E>>, Arc<Mutex<TrieNode<T, E>>>> =
            HashMap::new();

        // Stack for iterative traversal
        let mut stack: Vec<(Arc<Mutex<TrieNode<T, E>>>, Arc<Mutex<TrieNode<T, E>>>)> = Vec::new();

        // Wrap `self` in an Arc<Mutex<>> for consistency
        let self_arc = Arc::new(Mutex::new(self));

        // Push the roots onto the stack
        stack.push((self_arc.clone(), other.clone()));

        while let Some((self_node_arc, other_node_arc)) = stack.pop() {
            // Attempt to lock both nodes
            let mut self_node = self_node_arc.lock().unwrap();
            let other_node = other_node_arc.lock().unwrap();

            // Obtain raw non-null pointers for identification
            let other_node_ptr = NonNull::from(&*other_node);

            // Check if we've already processed this node
            if let Some(existing_self_arc) = node_map.get(&other_node_ptr) {
                // If the current `self_node_arc` is different, we need to ensure they are the same
                if !Arc::ptr_eq(&self_node_arc, existing_self_arc) {
                    // Merge the nodes
                    Self::merge_node_arcs(
                        &self_node_arc,
                        existing_self_arc,
                        merge_value,
                        &mut node_map,
                    );
                }
                continue;
            }

            // Record the mapping
            node_map.insert(other_node_ptr, self_node_arc.clone());

            // Merge the values
            self_node.value = merge_value(&self_node.value, &other_node.value);

            // Merge the children
            for (edge_label, child_other_arc) in &other_node.children {
                // Get or create the corresponding child in `self`
                let child_self_arc = self_node
                    .children
                    .entry(edge_label.clone())
                    .or_insert_with(|| Arc::new(Mutex::new(TrieNode::new())))
                    .clone();

                // Push the pair onto the stack for further processing
                stack.push((child_self_arc, child_other_arc.clone()));
            }
        }

        // Update the root of `self` after merging
        let merged_root = Arc::try_unwrap(self_arc).unwrap().into_inner().unwrap();
        Arc::new(Mutex::new(merged_root))
    }

    /// Merges two nodes represented by their `Arc<Mutex<>>` wrappers.
    fn merge_node_arcs(
        self_node_arc: &Arc<Mutex<TrieNode<T, E>>>,
        existing_self_arc: &Arc<Mutex<TrieNode<T, E>>>,
        merge_value: &dyn Fn(&Option<T>, &Option<T>) -> Option<T>,
        node_map: &mut HashMap<NonNull<TrieNode<T, E>>, Arc<Mutex<TrieNode<T, E>>>>,
    ) {
        let mut self_node = self_node_arc.lock().unwrap();
        let existing_node = existing_self_arc.lock().unwrap();

        // Merge the values
        self_node.value = merge_value(&self_node.value, &existing_node.value);

        // Merge the children
        for (edge_label, child_existing_arc) in &existing_node.children {
            let child_self_arc = self_node
                .children
                .entry(edge_label.clone())
                .or_insert_with(|| Arc::new(Mutex::new(TrieNode::new())))
                .clone();

            // Recursively merge the children
            Self::merge_node_arcs(
                &child_self_arc,
                child_existing_arc,
                merge_value,
                node_map,
            );
        }
    }
}

// For testing purposes
impl<T: PartialEq + Debug, E: PartialEq + Debug> PartialEq for TrieNode<T, E> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.children == other.children
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn merge_option_values(a: &Option<String>, b: &Option<String>) -> Option<String> {
        match (a, b) {
            (Some(a_val), Some(b_val)) if a_val == b_val => Some(a_val.clone()),
            (Some(a_val), None) => Some(a_val.clone()),
            (None, Some(b_val)) => Some(b_val.clone()),
            (None, None) => None,
            // For simplicity, concatenate the values if they differ
            (Some(a_val), Some(b_val)) => Some(format!("{}|{}", a_val, b_val)),
        }
    }

    #[test]
    fn test_merge_tries_with_shared_nodes() {
        // Construct trie1
        let mut trie1 = TrieNode::new();
        trie1.value = Some("root1".to_string());
        {
            let child_a = Arc::new(Mutex::new(TrieNode::new()));
            child_a.lock().unwrap().value = Some("child_a".to_string());
            trie1.children.insert('a', child_a.clone());

            let child_b = Arc::new(Mutex::new(TrieNode::new()));
            child_b.lock().unwrap().value = Some("child_b".to_string());
            trie1.children.insert('b', child_b.clone());

            let shared_child = Arc::new(Mutex::new(TrieNode::new()));
            shared_child.lock().unwrap().value = Some("shared_child".to_string());

            // Both child_a and child_b have the shared_child as 'c'
            child_a
                .lock()
                .unwrap()
                .children
                .insert('c', shared_child.clone());
            child_b
                .lock()
                .unwrap()
                .children
                .insert('c', shared_child.clone());
        }

        // Construct trie2
        let mut trie2 = TrieNode::new();
        trie2.value = Some("root2".to_string());
        {
            let child_a = Arc::new(Mutex::new(TrieNode::new()));
            child_a.lock().unwrap().value = Some("child_a".to_string());
            trie2.children.insert('a', child_a.clone());

            let child_d = Arc::new(Mutex::new(TrieNode::new()));
            child_d.lock().unwrap().value = Some("child_d".to_string());
            trie2.children.insert('d', child_d.clone());

            let shared_child = Arc::new(Mutex::new(TrieNode::new()));
            shared_child.lock().unwrap().value = Some("shared_child".to_string());

            // Both child_a and child_d have the shared_child as 'c'
            child_a
                .lock()
                .unwrap()
                .children
                .insert('c', shared_child.clone());
            child_d
                .lock()
                .unwrap()
                .children
                .insert('c', shared_child.clone());
        }

        // Wrap trie2 in an Arc<Mutex<>>
        let trie2_arc = Arc::new(Mutex::new(trie2));

        // Merge trie2 into trie1
        trie1.merge(&trie2_arc, &merge_option_values);

        // Now, trie1 should contain all nodes from both tries, with shared nodes preserved

        // Construct the expected merged trie
        let mut expected_trie = TrieNode::new();
        expected_trie.value = Some("root1|root2".to_string());
        {
            let child_a = Arc::new(Mutex::new(TrieNode::new()));
            child_a.lock().unwrap().value = Some("child_a".to_string());
            expected_trie.children.insert('a', child_a.clone());

            let child_b = Arc::new(Mutex::new(TrieNode::new()));
            child_b.lock().unwrap().value = Some("child_b".to_string());
            expected_trie.children.insert('b', child_b.clone());

            let child_d = Arc::new(Mutex::new(TrieNode::new()));
            child_d.lock().unwrap().value = Some("child_d".to_string());
            expected_trie.children.insert('d', child_d.clone());

            let shared_child = Arc::new(Mutex::new(TrieNode::new()));
            shared_child.lock().unwrap().value = Some("shared_child".to_string());

            // child_a, child_b, and child_d have the shared_child as 'c'
            child_a
                .lock()
                .unwrap()
                .children
                .insert('c', shared_child.clone());
            child_b
                .lock()
                .unwrap()
                .children
                .insert('c', shared_child.clone());
            child_d
                .lock()
                .unwrap()
                .children
                .insert('c', shared_child.clone());
        }

        // For comparison, we need to normalize the trie into a comparable structure
        fn normalize_trie(
            node: &TrieNode<String, char>,
        ) -> TrieNode<String, char> {
            let mut normalized_node = TrieNode {
                value: node.value.clone(),
                children: BTreeMap::new(),
            };
            for (edge, child_arc) in &node.children {
                let child_node = child_arc.lock().unwrap();
                normalized_node
                    .children
                    .insert(*edge, Arc::new(Mutex::new(normalize_trie(&child_node))));
            }
            normalized_node
        }

        let normalized_trie1 = normalize_trie(&trie1);
        let normalized_expected_trie = normalize_trie(&expected_trie);

        // Assert that the merged trie matches the expected trie
        assert_eq!(normalized_trie1, normalized_expected_trie);
    }
}