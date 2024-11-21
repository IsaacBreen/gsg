        use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
struct TrieNode<T, E> {
    value: Option<T>,
    children: BTreeMap<E, Arc<Mutex<TrieNode<T, E>>>>,
    num_parents: usize,
}

impl<T, E: Ord> TrieNode<T, E> {
    fn new() -> Self {
        TrieNode {
            value: None,
            children: BTreeMap::new(),
            num_parents: 0,
        }
    }

    fn insert(&mut self, edge: E, child: Arc<Mutex<TrieNode<T, E>>>) {
        child.lock().unwrap().num_parents += 1;
        self.children.insert(edge, child);
    }

    fn get(&self, edge: &E) -> Option<Arc<Mutex<TrieNode<T, E>>>> {
        self.children.get(edge).cloned()
    }
}

impl<T: Clone, E: Ord + Clone> TrieNode<T, E> {
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
                if entry.len() == child.num_parents {
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

    fn merge(
        node: Arc<Mutex<TrieNode<T, E>>>,
        other: Arc<Mutex<TrieNode<T, E>>>,
        t_merge: impl Fn(Vec<T>) -> T,
    ) {
        // A map to track the mapping of nodes from `other` to `self`
        let mut node_map: HashMap<*const TrieNode<T, E>, Arc<Mutex<TrieNode<T, E>>>> = HashMap::new();

        // Initialize the `special_map` algorithm
        TrieNode::special_map(
            other.clone(),
            // Step function
            |current_nodes: &Vec<Arc<Mutex<TrieNode<T, E>>>>, edge: &E, value: &T| {
                let mut new_nodes = Vec::new();

                for current_node in current_nodes {
                    let mut current_node_guard = current_node.lock().unwrap();

                    // Check if the current node has an equivalent edge
                    if let Some(child) = current_node_guard.get(edge) {
                        new_nodes.push(child);
                    } else {
                        // Check if the `other` node is already mapped
                        let other_node_ptr = &other.lock().unwrap() as &TrieNode<T, E> as *const _;
                        if let Some(mapped_node) = node_map.get(&other_node_ptr) {
                            // Add the mapped node as a child
                            current_node_guard.insert(edge.clone(), mapped_node.clone());
                            new_nodes.push(mapped_node.clone());
                        } else {
                            // Create a new node and map it
                            let new_node = Arc::new(Mutex::new(TrieNode {
                                value: Some(value.clone()),
                                children: BTreeMap::new(),
                                num_parents: 0,
                            }));
                            current_node_guard.insert(edge.clone(), new_node.clone());
                            node_map.insert(other_node_ptr, new_node.clone());
                            new_nodes.push(new_node);
                        }
                    }
                }

                new_nodes
            },
            // Merge function
            |values: Vec<Vec<Arc<Mutex<TrieNode<T, E>>>>>| {
                // Flatten the vectors and remove duplicates
                let mut merged_nodes = Vec::new();
                let mut seen = HashSet::new();

                for value in values {
                    for node in value {
                        let node_ptr = Arc::as_ptr(&node);
                        if seen.insert(node_ptr) {
                            merged_nodes.push(node);
                        }
                    }
                }

                merged_nodes
            },
        );

        // Traverse the `other` trie and merge its nodes into `self`
        let mut active_states: VecDeque<(Arc<Mutex<TrieNode<T, E>>>, Arc<Mutex<TrieNode<T, E>>>)> =
            VecDeque::new();
        active_states.push_back((node.clone(), other.clone()));

        while let Some((self_node, other_node)) = active_states.pop_front() {
            let mut self_node_guard = self_node.lock().unwrap();
            let other_node_guard = other_node.lock().unwrap();

            // Merge the values of the current nodes
            if let Some(other_value) = &other_node_guard.value {
                let merged_value = if let Some(self_value) = &self_node_guard.value {
                    t_merge(vec![self_value.clone(), other_value.clone()])
                } else {
                    other_value.clone()
                };
                self_node_guard.value = Some(merged_value);
            }

            // Traverse the children of the `other` node
            for (edge, other_child) in &other_node_guard.children {
                if let Some(self_child) = self_node_guard.get(edge) {
                    // If the edge exists in `self`, add the child pair to the active states
                    active_states.push_back((self_child, other_child.clone()));
                } else {
                    // If the edge does not exist in `self`, clone the `other` child and add it
                    let new_child = Arc::new(Mutex::new(TrieNode {
                        value: other_child.lock().unwrap().value.clone(),
                        children: BTreeMap::new(),
                        num_parents: 1,
                    }));
                    self_node_guard.insert(edge.clone(), new_child.clone());
                    active_states.push_back((new_child, other_child.clone()));
                }
            }
        }
    }
}