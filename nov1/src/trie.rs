        use std::collections::{HashMap, VecDeque};
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

impl<T, E> TrieNode<T, E> {
    fn special_map<S, M, V>(initial_node: Arc<Mutex<TrieNode<T, E>>>, mut step: S, mut merge: M)
    where
        S: FnMut(&V, &E, &T) -> V,
        M: FnMut(Vec<V>) -> V,
        V: Clone + Default,
        E: Ord,
    {

        // A queue of active states (node and its associated value of type V)
        let mut active_states: VecDeque<(Arc<Mutex<TrieNode<T, E>>>, V)> = VecDeque::new();

        // A map of dormant states (node ID to a vector of values of type V)
        let mut dormant_states: HashMap<*const TrieNode<T, E>, Vec<V>> = HashMap::new();

        // Initialize the queue with the root node and the default initial value
        active_states.push_back((initial_node, V::default()));

        while let Some((node_arc, value)) = active_states.pop_front() {
            let node = node_arc.lock().unwrap();

            // Traverse each child of the current node
            for (edge, child_arc) in &node.children {
                let child = child_arc.lock().unwrap();

                // Apply the step function to compute the new value
                let new_value = step(&value, edge, node.value.as_ref().unwrap());

                // Get the raw pointer to the child node for identification
                let child_ptr = &*child as *const TrieNode<T, E>;

                // Update the dormant state map
                let entry = dormant_states.entry(child_ptr).or_insert_with(Vec::new);
                entry.push(new_value.clone());

                // Check if we've visited all parents of this child
                if entry.len() == child.children.len() {
                    // Merge the values and push the result to the active states queue
                    let merged_value = merge(entry.clone());
                    dormant_states.remove(&child_ptr); // Remove the entry from dormant states
                    active_states.push_back((child_arc.clone(), merged_value));
                }
            }
        }

        // At the end, merge all remaining dormant states (if any) and return the result
        let remaining_values: Vec<V> = dormant_states
            .into_iter()
            .flat_map(|(_, values)| values)
            .collect();

        merge(remaining_values);
    }
}