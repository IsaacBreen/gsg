use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct TrieNode<E, T> {
    pub value: T,
    pub children: BTreeMap<E, Arc<Mutex<TrieNode<E, T>>>>,
    pub num_parents: usize,
}

impl<T, E: Ord> TrieNode<E, T> {
    pub fn new(value: T) -> TrieNode<E, T> {
        TrieNode {
            value,
            children: BTreeMap::new(),
            num_parents: 0,
        }
    }

    pub fn insert(&mut self, edge: E, child: Arc<Mutex<TrieNode<E, T>>>) -> Option<Arc<Mutex<TrieNode<E, T>>>> {
        if !self.children.contains_key(&edge) {
            child.lock().unwrap().num_parents += 1;
        }
        self.children.insert(edge, child)
    }

    pub fn get(&self, edge: &E) -> Option<Arc<Mutex<TrieNode<E, T>>>> {
        self.children.get(edge).cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn all_nodes(root: Arc<Mutex<TrieNode<E, T>>>) -> Vec<Arc<Mutex<TrieNode<E, T>>>> {
        let mut node_ptrs_in_order: Vec<*const TrieNode<E, T>> = Vec::new();
        let mut nodes: BTreeMap<*const TrieNode<E, T>, Arc<Mutex<TrieNode<E, T>>>> = BTreeMap::new();
        let mut queue: VecDeque<Arc<Mutex<TrieNode<E, T>>>> = VecDeque::new();
        queue.push_back(root);
        while let Some(node) = queue.pop_front() {
            node_ptrs_in_order.push(&*node.lock().unwrap() as *const TrieNode<E, T>);
            nodes.insert(&*node.lock().unwrap() as *const TrieNode<E, T>, node.clone());
            let node = node.lock().unwrap();
            for (_, child) in &node.children {
                queue.push_back(child.clone());
            }
        }
        node_ptrs_in_order.into_iter().map(|ptr| nodes.get(&ptr).unwrap().clone()).collect()
    }

    pub fn map_t<F, U>(self, f: F) -> Arc<Mutex<TrieNode<E, U>>>
    where
        T: Clone,
        E: Clone,
        // todo: is it 'proper' to use `Copy` here?
        F: Copy + Fn(T) -> U,
    {
        let mut active_states: Vec<(Arc<Mutex<TrieNode<E, T>>>, Arc<Mutex<TrieNode<E, U>>>)> = Vec::new();
        let mut dormant_states: HashMap<*const TrieNode<E, T>, (usize, Arc<Mutex<TrieNode<E, U>>>)> = HashMap::new();
        let root = Arc::new(Mutex::new(TrieNode::new(f(self.value.clone()))));
        active_states.push((Arc::new(Mutex::new(self)), root.clone()));

        while let Some((node, new_node)) = active_states.pop() {
            let node = node.lock().unwrap();
            for (edge, child_arc) in &node.children {
                let child = child_arc.lock().unwrap();
                if let Some((num_parents_seen, new_child)) = dormant_states.get_mut(&(&*child as *const TrieNode<E, T>)) {
                    new_node.lock().unwrap().insert(edge.clone(), new_child.clone());
                    *num_parents_seen += 1;
                    if *num_parents_seen == child.num_parents {
                        active_states.push((child_arc.clone(), new_child.clone()));
                    }
                } else {
                    let new_child = Arc::new(Mutex::new(TrieNode::new(f(child.value.clone()))));
                    new_node.lock().unwrap().insert(edge.clone(), new_child.clone());
                    if child.num_parents == 1 {
                        active_states.push((child_arc.clone(), new_child.clone()));
                    } else {
                        dormant_states.insert(&*child as *const TrieNode<E, T>, (1, new_child.clone()));
                    }
                }
            }
        }

        root
    }

    pub fn flatten<F>(&self, is_terminal: F) -> BTreeMap<Vec<E>, T>
    where
        E: Clone,
        T: Clone,
        F: Copy + Fn(&T) -> bool,
    {
        let mut result = BTreeMap::new();
        self.flatten_recursive(&mut result, Vec::new(), is_terminal);
        result
    }

    fn flatten_recursive<F>(
        &self,
        result: &mut BTreeMap<Vec<E>, T>,
        path: Vec<E>,
        is_terminal: F,
    )
    where
        E: Clone,
        T: Clone,
        F: Copy + Fn(&T) -> bool,
    {
        if is_terminal(&self.value) {
            result.insert(path.clone(), self.value.clone());
        }
        for (edge, child) in &self.children {
            let mut new_path = path.clone();
            new_path.push(edge.clone());
            child.lock().unwrap().flatten_recursive(result, new_path, is_terminal);
        }
    }
}

impl<T: Clone, E: Ord + Clone> TrieNode<E, T> {
    pub fn special_map<S, M, P, V>(initial_node: Arc<Mutex<TrieNode<E, T>>>, initial_value: V, mut step: S, mut merge: M, mut process: P)
    where
        S: FnMut(&V, &E, &TrieNode<E, T>) -> V,
        M: FnMut(Vec<V>) -> V,
        P: FnMut(&T, &V),
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

            // Process
            process(&node.value, &value);

            // Traverse each child of the current node
            for (edge, child_arc) in &node.children {
                let child = child_arc.lock().unwrap();

                // Apply the step function to compute the new value
                let new_value = step(&value, edge, &child);

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

    pub fn merge<T2>(
        node: Arc<Mutex<TrieNode<E, T>>>,
        other: Arc<Mutex<TrieNode<E, T2>>>,
        t_merge: impl Fn(T, T2) -> T,
        t_init: impl Fn() -> T,
    )
    where
        T2: Clone,
    {
        // A map to track the mapping of nodes from `other` to `self`
        let mut node_map: HashMap<*const TrieNode<E, T2>, Arc<Mutex<TrieNode<E, T>>>> = HashMap::new();

        let mut already_merged_values: HashSet<*const TrieNode<E, T>> = HashSet::new();

        // Special case: merge T for the root node
        let existing_value = node.lock().unwrap().value.clone();
        let new_value = t_merge(existing_value, other.lock().unwrap().value.clone());
        node.lock().unwrap().value = new_value;

        TrieNode::special_map(
            other.clone(),
            vec![node.clone()],
            // Step function
            |current_nodes: &Vec<Arc<Mutex<TrieNode<E, T>>>>, edge: &E, dest_other_node: &TrieNode<E, T2>| {
                let mut new_nodes = Vec::new();

                for current_self_node in current_nodes {
                    let mut current_self_node_guard = current_self_node.lock().unwrap();

                    // Check if the current node has an equivalent edge
                    if let Some(child) = current_self_node_guard.get(edge) {
                        if !already_merged_values.contains(&(&*child.lock().unwrap() as *const TrieNode<E, T>)) {
                            // Merge the values
                            let child_value = child.lock().unwrap().value.clone();
                            let merged_value = t_merge(child_value, dest_other_node.value.clone());
                            child.lock().unwrap().value = merged_value;
                        }
                        new_nodes.push(child);
                    } else {
                        // Check if the `other` node is already mapped
                        let other_node_ptr = dest_other_node as *const TrieNode<E, T2>;
                        if let Some(mapped_node) = node_map.get(&other_node_ptr) {
                            // Add the mapped node as a child
                            current_self_node_guard.insert(edge.clone(), mapped_node.clone());
                            new_nodes.push(mapped_node.clone());
                        } else {
                            // Create a new node and map it
                            let new_node = Arc::new(Mutex::new(TrieNode {
                                value: t_merge(t_init(), dest_other_node.value.clone()),
                                children: BTreeMap::new(),
                                num_parents: 0,
                            }));
                            current_self_node_guard.insert(edge.clone(), new_node.clone());
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
            // Process function
            |_, _| {}
        );
    }
}