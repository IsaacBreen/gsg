use crate::finite_automata::{GroupID, Regex};
use crate::glr;
use crate::glr::table::StateID;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use bitvec::prelude::BitVec;
use kdam::tqdm;
use crate::trie::{dump_structure, TrieNode};

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
        num_llm_tokens: usize,
    ) {
        // (position, state) -> node
        let mut queue: BTreeMap<(usize, usize), _> = BTreeMap::new();

        // let root: Arc<Mutex<TrieNode<TokenID, TokenizerStateInfoForLLMToken>>> = Arc::new(Mutex::new(TrieNode::new(TokenizerStateInfoForLLMToken { tokenizer_state_id: state, position_in_llm_token: 0, dirty_end_state: None, clean_end: false })));
        let root = state_map_root_arc.clone();

        // Initialize the queue with the starting state
        // todo: this can be simplified; any queue entries other than the first one should have initial state (i.e. 0)
        queue.insert((0, state), root.clone());

        crate::dbgprintln2!("Precomputing state {}", state);

        while let Some(((position, state), node)) = queue.pop_first() {
            crate::dbgprintln2!("Popped from queue: ({}, {})", position, state);

            // todo: does it make sense to have this here?
            // if position > text.len() {
            //     continue;
            // }
            assert!(position <= text.len());

            if position == text.len() {
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
                let new_state = 0;
                if let Some(new_node) = queue.get(&(new_position, new_state)) {
                    if node.lock().unwrap().get(&token.id).is_some() {
                        // do nothing
                    } else {
                        // Add an edge from the current node to the new node
                        node.lock().unwrap().insert(token.id as TokenID, new_node.clone());
                    }
                } else {
                    // if let Some(existing) = node.lock().unwrap().get(&token.id) {
                    if node.lock().unwrap().get(&token.id).is_some() {
                        let existing = node.lock().unwrap().get(&token.id).unwrap();
                        // Add it to the queue
                        queue.insert((new_position, new_state), existing.clone());
                    } else {
                        // Create a new node and add it to the queue
                        // let new_node = Arc::new(Mutex::new(TrieNode::new(TokenizerStateInfoForLLMToken { tokenizer_state_id: new_state, position_in_llm_token: new_position, dirty_end_state: None, clean_end: new_position == text.len() })));
                        let new_node = Arc::new(Mutex::new(TrieNode::new((BTreeMap::new(), BTreeMap::new(), None))));
                        node.lock().unwrap().insert(token.id as TokenID, new_node.clone());
                        queue.insert((new_position, new_state), new_node.clone());
                    }
                }
            }

            if let Some(new_state) = execute_result.new_state {
                for possible_grammar_token_id in &self.tokens_accessible_from_state(new_state) {
                    node.lock().unwrap().value.1.entry(*possible_grammar_token_id).or_insert_with(|| {
                        let mut bitset = BitVec::new();
                        bitset.resize(num_llm_tokens, false);
                        bitset
                    }).insert(llm_token_id.0, true);
                }
            }
        }
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
    llm_tokens: &[&'a [u8]],
    eof_llm_token_id: LLMTokenID,
) -> BTreeMap<StateID, TrieNode<TokenID, (BTreeMap<LLMTokenID, TokenizerStateInfoForLLMToken>, BTreeMap<TokenID, BitVec>, Option<BitVec>)>> {
    let mut result: BTreeMap<StateID, TrieNode<GroupID, _>> = BTreeMap::new();

    // Ensure the tokenizer doesn't match on empty strings
    crate::dbgprintln2!("Ensuring tokenizer doesn't match on empty strings");
    let execute_result = tokenizer.execute_from_state(&[], 0);
    if !execute_result.matches.is_empty() {
        panic!("Tokenizer should not match on empty string. If it did, there would be infinitely many possible token sequences for any LLM token.");
    }

    crate::dbgprintln2!("Precomputing");
    for state_id in tqdm!(0..tokenizer.max_state()) {
        let mut state_map_root_arc: Arc<Mutex<TrieNode<GroupID, (BTreeMap<LLMTokenID, TokenizerStateInfoForLLMToken>, BTreeMap<TokenID, BitVec>, Option<BitVec>)>>> = Arc::new(Mutex::new(TrieNode::new((BTreeMap::new(), BTreeMap::new(), None))));

        for (llm_token_id, &llm_token) in llm_tokens.iter().enumerate() {
            crate::dbgprintln2!("Executing token");
            tokenizer.execute_all_from_state(llm_token, state_id, state_map_root_arc.clone(), LLMTokenID(llm_token_id), llm_tokens.len() + 1);
            crate::dbgprintln2!("Merge");
            // Merge into the existing state map
            // TrieNode::merge(
            //     state_map_root_arc.clone(),
            //     token_tree,
            //     |(mut llm_token_id_to_state, mut grammar_token_id_to_bitvec, mut maybe_clean_end_bitvec), info: TokenizerStateInfoForLLMToken| {
            //         if info.dirty_end_state.is_some() | info.clean_end {
            //             llm_token_id_to_state.insert(LLMTokenID(llm_token_id), info);
            //         }
            //         if let Some(dirty_end_state) = info.dirty_end_state {
            //             for possible_grammar_token_id in &tokenizer.tokens_accessible_from_state(dirty_end_state.0) {
            //                 grammar_token_id_to_bitvec.entry(*possible_grammar_token_id).or_insert_with(|| {
            //                     let mut bitset = BitVec::new();
            //                     bitset.resize(llm_tokens.len(), false);
            //                     bitset
            //                 }).set(llm_token_id, true);
            //             }
            //         }
            //         if info.clean_end {
            //             maybe_clean_end_bitvec.get_or_insert_with(|| {
            //                 let mut bitset = BitVec::new();
            //                 bitset.resize(llm_tokens.len(), false);
            //                 bitset
            //             }).set(llm_token_id, true);
            //         }
            //         (llm_token_id_to_state, grammar_token_id_to_bitvec, maybe_clean_end_bitvec)
            //     },
            //     || { (BTreeMap::new(), BTreeMap::new(), None) },
            // );
        }

        // println!("Precomputing state {}", state_id);
        // dump_structure(state_map_root_arc.clone());

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::charmap::TrieMap;
    use crate::finite_automata::{eat_u8, DFAState, Regex, DFA};
    use crate::u8set::U8Set;
    use crate::{groups, seq};
    use std::collections::{BTreeMap, BTreeSet};

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
        let result = precompute(&tokenizer, llm_tokens, LLMTokenID(llm_tokens.len() + 1));

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