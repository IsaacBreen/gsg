use crate::finite_automata::{GroupID, Regex};
use crate::glr;
use crate::glr::table::StateID;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use kdam::tqdm;
use crate::trie::{dump_structure, TrieNode};

pub type TokenID = usize;

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
    ) -> Arc<Mutex<TrieNode<TokenID, Option<usize>>>> {
        // (position, state) -> node
        let mut queue: BTreeMap<(usize, usize), Arc<Mutex<TrieNode<TokenID, Option<usize>>>>> = BTreeMap::new();

        let root = Arc::new(Mutex::new(TrieNode::new(None)));

        // Initialize the queue with the starting state
        // todo: this can be simplified; any queue entries other than the first one should have initial state (i.e. 0)
        queue.insert((0, state), root.clone());

        while let Some(((position, state), node)) = queue.pop_first() {
            // todo: does it make sense to have this here?
            // if position > text.len() {
            //     continue;
            // }
            assert!(position <= text.len());
            
            if position == text.len() {
                node.lock().unwrap().value = Some(state);
                continue;
            }

            let remaining_text = &text[position..];
            let execute_result = self.execute_from_state(remaining_text, state);

            // assert_eq!(execute_result.matches.len(), execute_result.matches.iter().map(|m| m.id).collect::<BTreeSet<_>>().len());

            // Process all matches
            for token in &execute_result.matches {
                let new_position = position + token.width;
                assert_ne!(token.width, 0);
                assert!(new_position <= text.len());
                let new_state = execute_result.new_state.unwrap_or(0);
                if let Some(new_node) = queue.get_mut(&(new_position, new_state)) {
                    // Add an edge from the current node to the new node
                    node.lock().unwrap().insert(token.id as TokenID, new_node.clone());
                } else {
                    // Create a new node and add it to the queue
                    let new_node = Arc::new(Mutex::new(TrieNode::new(None)));
                    node.lock().unwrap().insert(token.id as TokenID, new_node.clone());
                    queue.insert((new_position, new_state), new_node.clone());
                }
            }
        }

        root
    }
}

// todo: remove this
/// Precomputes a map from each state and token sequence to a set of LLM token IDs.
pub fn precompute_llm_token_sets<'a>(
    precompute_map: &BTreeMap<StateID, BTreeMap<Vec<GroupID>, BTreeMap<&'a [u8], StateID>>>,
    llm_token_to_id: &BTreeMap<&'a [u8], usize>,
) -> BTreeMap<StateID, BTreeMap<Vec<GroupID>, BTreeSet<usize>>> {
    let mut result: BTreeMap<StateID, BTreeMap<Vec<GroupID>, BTreeSet<usize>>> = BTreeMap::new();

    for (&state_id, token_sequence_map) in precompute_map {
        let mut sequence_set_map: BTreeMap<Vec<GroupID>, BTreeSet<usize>> = BTreeMap::new();

        for (token_sequence, llm_token_state_map) in token_sequence_map {
            let mut set = BTreeSet::new();
            for (&llm_token, &_next_state) in llm_token_state_map {
                if let Some(&llm_token_id) = llm_token_to_id.get(llm_token) {
                    set.insert(llm_token_id);
                }
            }
            sequence_set_map.insert(token_sequence.clone(), set);
        }

        result.insert(state_id, sequence_set_map);
    }

    result
}

/// Precomputes a map from state -> token sequence -> LLM token -> state.
pub fn precompute<'a>(
    tokenizer: &impl Tokenizer,
    llm_tokens: &[&'a [u8]],
) -> BTreeMap<StateID, TrieNode<GroupID, BTreeMap<&'a [u8], StateID>>> {
    let mut result: BTreeMap<StateID, TrieNode<GroupID, BTreeMap<&'a [u8], StateID>>> = BTreeMap::new();

    // Ensure the tokenizer doesn't match on empty strings
    crate::dbgprintln2!("Ensuring tokenizer doesn't match on empty strings");
    let execute_result = tokenizer.execute_from_state(&[], 0);
    if !execute_result.matches.is_empty() {
        panic!("Tokenizer should not match on empty string. If it did, there would be infinitely many possible token sequences for any LLM token.");
    }

    crate::dbgprintln2!("Precomputing");
    for state_id in tqdm!(0..tokenizer.max_state()) {
        // crate::dbgprintln2!("Precomputing state {}", state_id);
        // let mut state_map: BTreeMap<Vec<GroupID>, BTreeMap<&'a [u8], StateID>> = BTreeMap::new();
        let mut state_map_root_arc: Arc<Mutex<TrieNode<GroupID, BTreeMap<&'a [u8], StateID>>>> = Arc::new(Mutex::new(TrieNode::new(BTreeMap::new())));

        for &llm_token in llm_tokens {
            // let token_str = std::str::from_utf8(llm_token).unwrap_or("Invalid UTF-8");
            // crate::dbgprintln2!("Precomputing token {:?} ({:?})", llm_token, token_str);
            let token_tree = tokenizer.execute_all_from_state(llm_token, state_id);
            // for (x, y) in token_tree.lock().unwrap().flatten(Option::is_some) {
            //     crate::dbgprintln!("Precomputed token {:?} ({:?}) -> {:?} ({:?})", llm_token, token_str, x, y);
            // }
            // for node in TrieNode::all_nodes(token_tree.clone()) {
            //     // print the node address and value
            //     crate::dbgprintln!("Token tree node address: {:p}, value: {:?}", Arc::as_ptr(&node), node.lock().unwrap().value);
            //     // print edge values and destination addresses
            //     for (edge, dest) in node.lock().unwrap().children.iter() {
            //         crate::dbgprintln!("    Edge value: {:?}, destination address: {:p}", edge, Arc::as_ptr(&dest));
            //     }
            // }
            // Merge into the existing state map
            TrieNode::merge(
                state_map_root_arc.clone(),
                token_tree,
                |mut llm_token_to_state: BTreeMap<&'a [u8], StateID>, maybe_new_final_state_id: Option<usize>| {
                    if let Some(new_final_state_id) = maybe_new_final_state_id {
                        llm_token_to_state.insert(llm_token, StateID(new_final_state_id));
                    }
                    llm_token_to_state
                },
                || { BTreeMap::new() },
            );
            // for node in TrieNode::all_nodes(state_map_root_arc.clone()) {
            //     // print the node address and value
            //     crate::dbgprintln!("Node address: {:p}, value: {:?}", Arc::as_ptr(&node), node.lock().unwrap().value);
            //     // print edge values and destination addresses
            //     for (edge, dest) in node.lock().unwrap().children.iter() {
            //         crate::dbgprintln!("    Edge value: {:?}, destination address: {:p}", edge, Arc::as_ptr(&dest));
            //     }
            // }
            // for (x, y) in state_map_root_arc.lock().unwrap().flatten(|llm_token_to_state| !llm_token_to_state.is_empty()) {
            //     crate::dbgprintln!("HERE: Precomputed token {:?} ({:?}) -> {:?} ({:?})", llm_token, token_str, x, y);
            // }
            // dump_structure(state_map_root_arc.clone());
        }

        println!("Precomputing state {}", state_id);
        dump_structure(state_map_root_arc.clone());

        let state_map_root = state_map_root_arc.lock().unwrap().clone();
        result.insert(glr::table::StateID(state_id), state_map_root);
    }

    result
}

pub fn precompute_add_incomplete_token<'a>(
    tokenizer: &impl Tokenizer,
    precomputed: BTreeMap<StateID, TrieNode<GroupID, BTreeMap<&'a [u8], StateID>>>,
) -> BTreeMap<StateID, TrieNode<TokenID, BTreeMap<&'a [u8], StateID>>> {
    // let mut result: BTreeMap<StateID, BTreeMap<Vec<TokenID>, BTreeMap<&'a [u8], StateID>>> = BTreeMap::new();
    // for (state_id, token_sequence_map) in precomputed {
    //     for (token_id_sequence, llm_token_state_map) in token_sequence_map {
    //         for (llm_token, next_state_id) in llm_token_state_map {
    //             for possible_next_token_id in tokenizer.tokens_accessible_from_state(next_state_id.0) {
    //                 let mut new_token_sequence = token_id_sequence.clone();
    //                 new_token_sequence.push(possible_next_token_id);
    //                 // todo: this shouldn't be necessary. Just a sanity check. Consider removing.
    //                 if let Some(existing) = result.entry(state_id).or_default().entry(new_token_sequence.clone()).or_default().get(llm_token) {
    //                     assert_eq!(*existing, next_state_id);
    //                 }
    //                 result.entry(state_id).or_default().entry(new_token_sequence).or_default().insert(llm_token, next_state_id);
    //             }
    //         }
    //     }
    // }
    precomputed
    // todo: remove this function
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::charmap::TrieMap;
    use crate::finite_automata::{eat_u8, DFAState, Regex, DFA};
    use crate::u8set::U8Set;
    use crate::{groups, seq};
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn test_precompute_llm_token_sets() {
        // Define LLM tokens
        let llm_tokens: &[&[u8]] = &[b"a", b"b", b"c", b"ab", b"ac"];

        // Map LLM tokens to unique IDs
        let llm_token_to_id: BTreeMap<&[u8], usize> = llm_tokens.iter().enumerate().map(|(i, &token)| (token, i)).collect();

        // Build the expected precompute_map
        // We will manually construct the expected output based on the DFA and LLM tokens

        // Initialize the expected map
        let mut precompute_map: BTreeMap<StateID, BTreeMap<Vec<GroupID>, BTreeMap<&[u8], StateID>>> = BTreeMap::new();

        // For DFA state 0 (start state)
        let mut state0_map: BTreeMap<Vec<GroupID>, BTreeMap<&[u8], StateID>> = BTreeMap::new();

        // Analyze each LLM token starting from state 0

        // LLM token "ab"
        // - It matches the grammar token sequence [0] ("ab") and ends in an accepting state
        state0_map.entry(vec![0]).or_insert_with(BTreeMap::new).insert(b"ab", /* end state */ StateID(0)); // We can use 0 as the end state for simplicity

        // LLM token "ac"
        // - It matches the grammar token sequence [1] ("ac") and ends in an accepting state
        state0_map.entry(vec![1]).or_insert_with(BTreeMap::new).insert(b"ac", StateID(0));

        // LLM tokens "a", "b", "c"
        // - These tokens do not produce any complete grammar token sequences starting from state 0
        // - Therefore, they are not included in the expected_precompute_map

        precompute_map.insert(StateID(0), state0_map);

        // Perform precompute
        let bitset_map = precompute_llm_token_sets(&precompute_map, &llm_token_to_id);

        // Build the expected bitset_map based on the expected_precompute_map
        let mut expected_bitset_map: BTreeMap<StateID, BTreeMap<Vec<GroupID>, BTreeSet<usize>>> = BTreeMap::new();

        let mut state0_bitset_map: BTreeMap<Vec<GroupID>, BTreeSet<usize>> = BTreeMap::new();

        // For grammar token sequence [0] ("ab"), the LLM token is "ab" with ID 3
        let mut bitset_ab = BTreeSet::<usize>::new();
        let llm_token_id_ab = *llm_token_to_id.get(b"ab".as_slice()).unwrap();
        bitset_ab.insert(llm_token_id_ab);
        state0_bitset_map.insert(vec![0], bitset_ab);

        // For grammar token sequence [1] ("ac"), the LLM token is "ac" with ID 4
        let mut bitset_ac = BTreeSet::<usize>::new();
        let llm_token_id_ac = *llm_token_to_id.get(b"ac".as_slice()).unwrap();
        bitset_ac.insert(llm_token_id_ac);
        state0_bitset_map.insert(vec![1], bitset_ac);

        expected_bitset_map.insert(StateID(0), state0_bitset_map);

        // Compare the actual bitset_map to the expected one
        assert_eq!(
            bitset_map, expected_bitset_map,
            "The bitset_map does not match the expected map.\nExpected: {:?}\nActual: {:?}",
            expected_bitset_map, bitset_map
        );
    }

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

        // Run precompute
        let result = precompute(&tokenizer, llm_tokens);

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