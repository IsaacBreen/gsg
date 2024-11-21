use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct TrieNode<E, T> {
    pub value: Option<T>,
    pub children: BTreeMap<E, Arc<Mutex<TrieNode<E, T>>>>,
    pub num_parents: usize,
}

impl<T, E: Ord> TrieNode<E, T> {
    pub fn new() -> Self {
        TrieNode {
            value: None,
            children: BTreeMap::new(),
            num_parents: 0,
        }
    }

    pub fn insert(&mut self, edge: E, child: Arc<Mutex<TrieNode<E, T>>>) {
        child.lock().unwrap().num_parents += 1;
        self.children.insert(edge, child);
    }

    pub fn get(&self, edge: &E) -> Option<Arc<Mutex<TrieNode<E, T>>>> {
        self.children.get(edge).cloned()
    }

    pub fn flatten(&self) -> BTreeMap<Vec<E>, T>
    where
        E: Clone,
        T: Clone,
    {
        let mut result = BTreeMap::new();
        self.flatten_recursive(&mut result, Vec::new());
        result
    }

    fn flatten_recursive(
        &self,
        result: &mut BTreeMap<Vec<E>, T>,
        path: Vec<E>,
    )
    where
        E: Clone,
        T: Clone,
    {
        if let Some(value) = &self.value {
            result.insert(path.clone(), value.clone());
        }
        for (edge, child) in &self.children {
            let mut new_path = path.clone();
            new_path.push(edge.clone());
            child.lock().unwrap().flatten_recursive(result, new_path);
        }
    }
}

impl<T: Clone, E: Ord + Clone> TrieNode<E, T> {
    pub fn special_map<S, M, V>(initial_node: Arc<Mutex<TrieNode<E, T>>>, initial_value: V, mut step: S, mut merge: M)
    where
        S: FnMut(&V, &E, &T) -> V,
        M: FnMut(Vec<V>) -> V,
        V: Clone,
        E: Ord,
    {
        // A queue of active states (node and its associated value of type V)
        let mut active_states: VecDeque<(Arc<Mutex<TrieNode<E, T>>>, V)> = VecDeque::new();

        // A map of dormant states (node ID to a vector of values of type V)
        let mut dormant_states: HashMap<*const TrieNode<E, T>, Vec<V>> = HashMap::new();

        // Initialize the queue with the root node and the default initial value
        active_states.push_back((initial_node, initial_value));

        while let Some((node_arc, value)) = active_states.pop_front() {
            let node = node_arc.lock().unwrap();

            // Traverse each child of the current node
            for (edge, child_arc) in &node.children {
                let child = child_arc.lock().unwrap();

                // Apply the step function to compute the new value
                let new_value = step(&value, edge, node.value.as_ref().unwrap());

                // Get the raw pointer to the child node for identification
                let child_ptr = &*child as *const TrieNode<E, T>;

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

        // At the end, if there are any dormant states left, something went wrong
        if !dormant_states.is_empty() {
            panic!("Leftover dormant states");
        }
    }

    pub fn merge(
        node: Arc<Mutex<TrieNode<E, T>>>,
        other: Arc<Mutex<TrieNode<E, T>>>,
        t_merge: impl Fn(Vec<T>) -> T,
    ) {
        // A map to track the mapping of nodes from `other` to `self`
        let mut node_map: HashMap<*const TrieNode<E, T>, Arc<Mutex<TrieNode<E, T>>>> = HashMap::new();

        let mut already_merged_values: HashSet<*const TrieNode<E, T>> = HashSet::new();

        // Initialize the `special_map` algorithm
        TrieNode::special_map(
            other.clone(),
            vec![node.clone()],
            // Step function
            |current_nodes: &Vec<Arc<Mutex<TrieNode<E, T>>>>, edge: &E, value: &T| {
                let mut new_nodes = Vec::new();

                for current_node in current_nodes {
                    let mut current_node_guard = current_node.lock().unwrap();

                    // Check if the current node has an equivalent edge
                    if let Some(child) = current_node_guard.get(edge) {
                        if !already_merged_values.contains(&(&*child.lock().unwrap() as *const TrieNode<E, T>)) {
                            // Merge the values
                            let child_value = child.lock().unwrap().value.as_ref().unwrap().clone();
                            let merged_value = t_merge(vec![child_value, value.clone()]);
                            child.lock().unwrap().value = Some(merged_value);
                        }
                        new_nodes.push(child);
                    } else {
                        // Check if the `other` node is already mapped
                        let other_node_ptr = &other.lock().unwrap() as &TrieNode<E, T> as *const _;
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
            |values: Vec<Vec<Arc<Mutex<TrieNode<E, T>>>>>| {
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
    }
}