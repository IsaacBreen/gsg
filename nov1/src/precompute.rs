use crate::finite_automata::{GroupID, Regex};
use crate::glr;
use crate::glr::table::StateID;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use bitvec::prelude::BitVec;
use kdam::tqdm;
use crate::trie::{dump_structure, TrieNode};
use bimap::BiBTreeMap;

pub type TokenID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LLMTokenID(pub usize);

/// Represents a token with its ID and width.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub id: GroupID,
    pub width: usize,
}

/// Represents the result of executing the tokenizer from a specific state.
pub struct ExecuteResult {
    pub matches: Vec<Token>,
    pub new_state: Option<usize>,
}

// TODO: get rid of this trait. Just implement it directly on the Tokenizer struct.
/// Trait defining the tokenizer behavior.
pub trait Tokenizer: Sized {
    /// Returns the initial state ID.
    fn initial_state_id(&self) -> usize;

    /// Executes the tokenizer on the given text starting from the specified state.
    /// Returns all possible next tokens (**not** a sequence of tokens).
    fn execute_from_state(&self, text: &[u8], state: usize) -> ExecuteResult;

    /// Returns the list of tokens accessible from the given state.
    fn tokens_accessible_from_state(&self, state: usize) -> Vec<TokenID>;

    /// Returns the maximum state ID in the DFA.
    fn max_state(&self) -> usize;

    /// Executes the tokenizer on the entire string and returns all possible token sequences and final states.
    fn execute_all_from_state(
        &self,
        text: &[u8],
        state: usize,
        state_map_root_arc: Arc<Mutex<TrieNode<GroupID, (BTreeMap<LLMTokenID, TokenizerStateInfoForLLMToken>, BTreeMap<TokenID, BitVec>, Option<BitVec>)>>>,
        llm_token_id: LLMTokenID,
        max_token_id: usize,
    ) {
        // (position, state) -> [node]
        let mut queue: BTreeMap<(usize, Option<usize>), Vec<_>> = BTreeMap::new();

        let mut new_nodes: BTreeSet<*const TrieNode<GroupID, (BTreeMap<LLMTokenID, TokenizerStateInfoForLLMToken>, BTreeMap<TokenID, BitVec>, Option<BitVec>)>> = BTreeSet::new();

        // let root: Arc<Mutex<TrieNode<TokenID, TokenizerStateInfoForLLMToken>>> = Arc::new(Mutex::new(TrieNode::new(TokenizerStateInfoForLLMToken { tokenizer_state_id: state, position_in_llm_token: 0, dirty_end_state: None, clean_end: false })));
        let root = state_map_root_arc.clone();

        // Initialize the queue with the starting state
        // todo: this can be simplified; any queue entries other than the first one should have initial state (i.e. 0)
        queue.insert((0, Some(state)), vec![root.clone()]);

        while let Some(((position, maybe_state), nodes)) = queue.pop_first() {
            crate::dbgprintln2!("Popped from queue: ({}, {:?})", position, maybe_state);

            // todo: does it make sense to have this here?
            assert!(position <= text.len());

            for node in nodes {
                if position == text.len() {
                    assert!(!node.lock().unwrap().value.0.contains_key(&llm_token_id));
                    node.lock().unwrap().value.0.insert(llm_token_id, TokenizerStateInfoForLLMToken {
                        tokenizer_state_id: 99999999999,
                        position_in_llm_token: position,
                        dirty_end_state: maybe_state.map(|s| StateID(s)),
                        clean_end: maybe_state.is_none()
                    });
                    if let Some(state) = maybe_state {
                        for possible_grammar_token_id in &self.tokens_accessible_from_state(state) {
                            node.lock().unwrap().value.1.entry(*possible_grammar_token_id).or_insert_with(|| {
                                let mut bitset = BitVec::new();
                                bitset.resize(max_token_id, false);
                                bitset
                            }).set(llm_token_id.0, true);
                        }
                    } else {
                        crate::dbgprintln2!("No state. Clean end");
                        node.lock().unwrap().value.2.get_or_insert_with(|| {
                            let mut bitset = BitVec::new();
                            bitset.resize(max_token_id, false);
                            bitset
                        }).set(llm_token_id.0, true);
                    }
                    continue;
                }

                let remaining_text = &text[position..];
                let execute_result = self.execute_from_state(remaining_text, maybe_state.unwrap_or(0));

                // Process all matches
                for token in &execute_result.matches {
                    let new_position = position + token.width;
                    assert_ne!(token.width, 0);
                    assert!(new_position <= text.len());
                    let new_state = None;
                    crate::dbgprintln2!("Processing token {:?}", token);
                    if let Some(queued_nodes) = queue.get_mut(&(new_position, new_state)) {
                        crate::dbgprintln2!("Existing nodes in queue");
                        let child_exists = node.lock().unwrap().get(&token.id).is_some();
                        if child_exists {
                            // todo: can this ever actually happen?
                            crate::dbgprintln2!("Child exists in trie (1)");
                            let child = node.lock().unwrap().get(&token.id).unwrap();
                            // Check if the existing node is already in the queue
                            let mut child_already_queued = false;
                            for queued_node in queued_nodes.iter() {
                                if Arc::as_ptr(&queued_node) == Arc::as_ptr(&child) {
                                    child_already_queued = true;
                                    break;
                                }
                            }
                            if child_already_queued {
                                crate::dbgprintln2!("...and is already queued");
                            } else {
                                crate::dbgprintln2!("Pushing child to queue");
                                queued_nodes.push(child.clone());
                            }
                        } else {
                            // NOTE: Careful here. It's easy to cause cycles in the trie.
                            // Add an edge from the current node to any one of the new nodes (doesn't matter which)
                            // crate::dbgprintln2!("Adding edge to one of the new nodes");
                            // node.lock().unwrap().insert(token.id as TokenID, queued_nodes.first().unwrap().clone());  // Don't do this (can create a cycle)

                            // Only add an edge if it's a new node
                            let mut added = false;
                            for queued_node in &mut *queued_nodes {
                                if new_nodes.contains(&(&*queued_node.lock().unwrap() as *const TrieNode<_, _>)) {
                                    let node_ptr = &*node.lock().unwrap() as *const TrieNode<_, _>;
                                    crate::dbgprintln2!("Adding edge from {:?} to {:?}", node_ptr, &*queued_node.lock().unwrap() as *const TrieNode<_, _>);
                                    node.lock().unwrap().insert(token.id as TokenID, queued_nodes.first().unwrap().clone());
                                    added = true;
                                    break;
                                }
                            }
                            if !added {
                                // Create a new node
                                let new_child = Arc::new(Mutex::new(TrieNode::new((BTreeMap::new(), BTreeMap::new(), None))));
                                new_nodes.insert(&*new_child.lock().unwrap() as *const TrieNode<_, _>);
                                node.lock().unwrap().insert(token.id as TokenID, new_child.clone());
                                let node_ptr = &*node.lock().unwrap() as *const TrieNode<_, _>;
                                let new_child_ptr = &*new_child.lock().unwrap() as *const TrieNode<_, _>;
                                crate::dbgprintln2!("Creating new node (1) and adding edge from {:?} to {:?}", node_ptr, new_child_ptr);
                                queued_nodes.push(new_child.clone());
                            }
                        }
                    } else {
                        let child_exists = node.lock().unwrap().get(&token.id).is_some();
                        if child_exists {
                            let child = node.lock().unwrap().get(&token.id).unwrap();
                            let child_ptr = &*child.lock().unwrap() as *const TrieNode<_, _>;
                            crate::dbgprintln2!("Child exists in trie (2). Inserting child {:?} into queue", child_ptr);
                            queue.insert((new_position, new_state), vec![child.clone()]);
                        } else {
                            crate::dbgprintln2!("Creating new node (2)");
                            // Create a new node and add it to the queue
                            let new_child = Arc::new(Mutex::new(TrieNode::new((BTreeMap::new(), BTreeMap::new(), None))));
                            new_nodes.insert(&*new_child.lock().unwrap() as *const TrieNode<_, _>);
                            node.lock().unwrap().insert(token.id as TokenID, new_child.clone());
                            let node_ptr = &*node.lock().unwrap() as *const TrieNode<_, _>;
                            let new_child_ptr = &*new_child.lock().unwrap() as *const TrieNode<_, _>;
                            crate::dbgprintln2!("Creating new node (3) and adding edge from {:?} to {:?}", node_ptr, new_child_ptr);
                            queue.insert((new_position, new_state), vec![new_child.clone()]);
                        }
                    }

                    dump_structure(state_map_root_arc.clone());
                }
            }
        }
        crate::dbgprintln2!("Done processing token");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenizerStateInfoForLLMToken {
    pub tokenizer_state_id: usize,
    pub position_in_llm_token: usize,
    pub dirty_end_state: Option<StateID>,
    // todo: do we even need this?
    pub clean_end: bool,
}

/// Precomputes a map from state -> token sequence -> LLM token -> state.
pub fn precompute<'a>(
    tokenizer: &impl Tokenizer,
    llm_token_map: &BiBTreeMap<Vec<u8>, LLMTokenID>,
    eof_llm_token_id: LLMTokenID,
    max_token_id: usize,
) -> BTreeMap<StateID, TrieNode<TokenID, (BTreeMap<LLMTokenID, TokenizerStateInfoForLLMToken>, BTreeMap<TokenID, BitVec>, Option<BitVec>)>> {
    let mut result: BTreeMap<StateID, TrieNode<GroupID, _>> = BTreeMap::new();

    // Ensure the tokenizer doesn't match on empty strings
    crate::dbgprintln2!("Ensuring tokenizer doesn't match on empty strings");
    let execute_result = tokenizer.execute_from_state(&[], 0);
    if !execute_result.matches.is_empty() {
        panic!("Tokenizer should not match on empty string. If it did, there would be infinitely many possible token sequences for any LLM token.");
    }

    crate::dbgprintln2!("Precomputing in precompute");
    for state_id in tqdm!(0..tokenizer.max_state()) {
        crate::dbgprintln2!("Precomputing state {}", state_id);
        let mut state_map_root_arc: Arc<Mutex<TrieNode<GroupID, (BTreeMap<LLMTokenID, TokenizerStateInfoForLLMToken>, BTreeMap<TokenID, BitVec>, Option<BitVec>)>>> = Arc::new(Mutex::new(TrieNode::new((BTreeMap::new(), BTreeMap::new(), None))));

        for (i, (llm_token, llm_token_id)) in llm_token_map.iter().enumerate() {
            crate::dbgprintln2!("Precomputing for token {:?} ({:?}) ({})", llm_token_id, llm_token, i);
            // todo: REMOVE THIS
            if i < 121 {
                continue;
            }
            dump_structure(state_map_root_arc.clone());
            tokenizer.execute_all_from_state(
                llm_token,
                state_id,
                state_map_root_arc.clone(),
                *llm_token_id,
                max_token_id,
            );
        }

        crate::dbgprintln2!("Done precomputing state {}", state_id);
        let state_map_root = state_map_root_arc.lock().unwrap().clone();
        result.insert(glr::table::StateID(state_id), state_map_root);
    }

    result
}

impl Tokenizer for Regex {
    fn initial_state_id(&self) -> usize {
        0
    }

    fn execute_from_state(&self, text: &[u8], state: usize) -> ExecuteResult {
        let mut regex_state = self.init_to_state(state);
        regex_state.execute(text);

        let matches: Vec<_> = regex_state.matches.iter().map(|(&id, &width)| Token { id, width })
            // Filter out zero-width tokens
            .filter(|token| token.width != 0).collect();

        ExecuteResult {
            matches,
            new_state: if regex_state.done { None } else { Some(regex_state.current_state) },
        }
    }

    fn tokens_accessible_from_state(&self, state: usize) -> Vec<TokenID> {
        let regex_state = self.init_to_state(state);
        regex_state.possible_group_ids().iter().cloned().collect()
    }

    fn max_state(&self) -> usize {
        self.dfa.states.len()
    }
}

pub fn print_precomputed(precomputed: &BTreeMap<StateID, TrieNode<TokenID, (BTreeMap<LLMTokenID, TokenizerStateInfoForLLMToken>, BTreeMap<TokenID, BitVec>, Option<BitVec>)>>) {
    println!("Precomputed:");
    for (tokenizer_state, root) in precomputed {
        println!("  Tokenizer state: {}", tokenizer_state.0);
        for node in TrieNode::all_nodes(Arc::new(Mutex::new(root.clone()))) {
            println!("    Node address: {:p}, value: {:?}", Arc::as_ptr(&node), node.lock().unwrap().value);
            // print edge values and destination addresses
            for (edge, dest) in node.lock().unwrap().children() {
                println!("      Edge value: {:?}, destination address: {:p}", edge, Arc::as_ptr(&dest));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::charmap::TrieMap;
    use crate::finite_automata::{eat_u8, DFAState, Regex, DFA};
    use crate::u8set::U8Set;
    use crate::{groups, seq};
    use std::collections::{BTreeMap, BTreeSet};
    use bimap::BiBTreeMap;

    #[test]
    fn test_precompute() {
        let _tokenizer = groups![
            eat_u8(b'a'), // Token 0: 'a'
            eat_u8(b'b'), // Token 1: 'b'
            seq![eat_u8(b'a'), eat_u8(b'b')], // Token 2: 'ab'
            seq![eat_u8(b'a'), eat_u8(b'b'), eat_u8(b'c')], // Token 3: 'abc'
        ].build();

        let tokenizer = Regex {
            dfa: DFA {
                states: vec![
                    DFAState {
                        transitions: TrieMap::from_iter(vec![(b'a', 1), (b'b', 2)]),
                        finalizers: BTreeSet::new(),
                        possible_group_ids: BTreeSet::from([0, 1, 2, 3]),
                        group_id_to_u8set: BTreeMap::from([
                            (0, U8Set::from_bytes(b"a")),
                            (1, U8Set::from_bytes(b"b")),
                            (2, U8Set::from_bytes(b"a")),
                            (3, U8Set::from_bytes(b"a")),
                        ]),
                    },
                    DFAState {
                        transitions: TrieMap::from_iter(vec![(b'b', 3)]),
                        finalizers: BTreeSet::from([0]),
                        possible_group_ids: BTreeSet::from([0, 2, 3]),
                        group_id_to_u8set: BTreeMap::from([
                            (2, U8Set::from_bytes(b"b")),
                            (3, U8Set::from_bytes(b"b")),
                        ]),
                    },
                    DFAState {
                        transitions: TrieMap::new(),
                        finalizers: BTreeSet::from([1]),
                        possible_group_ids: BTreeSet::from([1]),
                        group_id_to_u8set: BTreeMap::new(),
                    },
                    DFAState {
                        transitions: TrieMap::from_iter(vec![(b'c', 4)]),
                        finalizers: BTreeSet::from([2]),
                        possible_group_ids: BTreeSet::from([2, 3]),
                        group_id_to_u8set: BTreeMap::from([(3, U8Set::from_bytes(b"c"))]),
                    },
                    DFAState {
                        transitions: TrieMap::new(),
                        finalizers: BTreeSet::from([3]),
                        possible_group_ids: BTreeSet::from([3]),
                        group_id_to_u8set: BTreeMap::new(),
                    },
                ],
                start_state: 0,
                non_greedy_finalizers: BTreeSet::new(),
            },
        };
        assert_eq!(_tokenizer, tokenizer);

        // Define the LLM tokens
        let llm_tokens: &[&[u8]] = &[b"a", b"b", b"c", b"ab", b"bc", b"abc"];
        let llm_token_map: BiBTreeMap<Vec<u8>, LLMTokenID> = llm_tokens.iter().enumerate().map(|(i, token)| (token.to_vec(), LLMTokenID(i))).collect();

        // Run precompute
        let max_token_id = llm_tokens.len() + 1;
        let result = precompute(&tokenizer, &llm_token_map, LLMTokenID(max_token_id), max_token_id);

        // todo: update this for TrieNode
        // // Build the expected output
        // let mut state_0: BTreeMap<Vec<GroupID>, BTreeMap<&[u8], StateID>> = BTreeMap::new();
        // state_0.insert(vec![], BTreeMap::from([(b"a".as_slice(), StateID(1)), (b"ab", StateID(3))]));
        // state_0.insert(vec![0], BTreeMap::from([(b"a".as_slice(), StateID(0))]));
        // state_0.insert(vec![0, 1], BTreeMap::from([(b"ab".as_slice(), StateID(0))]));
        // state_0.insert(vec![1], BTreeMap::from([(b"b".as_slice(), StateID(0))]));
        // state_0.insert(vec![2], BTreeMap::from([(b"ab".as_slice(), StateID(0))]));
        // state_0.insert(vec![3], BTreeMap::from([(b"abc".as_slice(), StateID(0))]));
        // assert_eq!(Some(&state_0), result.get(&StateID(0)));
        //
        // let mut state_1: BTreeMap<Vec<GroupID>, BTreeMap<&[u8], StateID>> = BTreeMap::new();
        // state_1.insert(vec![], BTreeMap::from([(b"b".as_slice(), StateID(3))]));
        // state_1.insert(vec![2], BTreeMap::from([(b"b".as_slice(), StateID(0))]));
        // state_1.insert(vec![3], BTreeMap::from([(b"bc".as_slice(), StateID(0))]));
        // assert_eq!(Some(&state_1), result.get(&StateID(1)));
        //
        // assert_eq!(None, result.get(&StateID(2)));
        //
        // let mut state_3: BTreeMap<Vec<GroupID>, BTreeMap<&[u8], StateID>> = BTreeMap::new();
        // state_3.insert(vec![3], BTreeMap::from([(b"c".as_slice(), StateID(0))]));
        // assert_eq!(Some(&state_3), result.get(&StateID(3)));
        //
        // assert_eq!(None, result.get(&StateID(4)));
        //
        // let mut expected: BTreeMap<StateID, BTreeMap<Vec<GroupID>, BTreeMap<&[u8], StateID>>> = BTreeMap::new();
        // expected.insert(StateID(0), state_0);
        // expected.insert(StateID(1), state_1);
        // expected.insert(StateID(3), state_3);
        //
        // assert_eq!(&expected, &result);
    }
}