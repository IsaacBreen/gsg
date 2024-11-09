// src/precompute.rs
use std::collections::{BTreeMap, BTreeSet, HashMap};
use crate::finite_automata::GroupID;

type StateID = usize;
type TokenID = usize;

/// Represents a token with its ID and width.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    pub id: GroupID,
    pub width: usize,
}

/// Represents the result of executing the tokenizer from a specific state.
pub struct ExecuteResult {
    pub matches: Vec<Token>,
    pub new_state: Option<usize>,
}

impl ExecuteResult {
    pub fn new(mut matches: Vec<Token>, new_state: Option<usize>) -> Self {
        // Remove zero-width matches, since they cause infinite loops
        matches.retain(|token| token.width > 0);
        ExecuteResult { matches, new_state }
    }
}

/// Trait defining the tokenizer behavior.
pub trait Tokenizer: Sized {
    /// Executes the tokenizer on the given text starting from the specified state.
    fn execute_from_state(&self, text: &[u8], state: usize) -> ExecuteResult;

    /// Returns the list of tokens accessible from the given state.
    fn possible_tokens_from_state(&self, state: usize) -> Vec<Token>;

    /// Returns the maximum state ID in the DFA.
    fn max_state(&self) -> usize;

    /// Executes the tokenizer on the entire string and returns all possible token sequences and final states.
    fn execute_all_from_state(
        &self,
        text: &[u8],
        state: usize,
    ) -> BTreeMap<Vec<Token>, usize> {
        use std::collections::VecDeque;

        // Define a queue item structure
        struct QueueItem {
            tokens: Vec<Token>,
            position: usize,
            state: usize,
        }

        let mut queue: VecDeque<QueueItem> = VecDeque::new();
        let mut final_results: BTreeMap<Vec<Token>, usize> = BTreeMap::new();

        // Initialize the queue with the starting state
        queue.push_back(QueueItem {
            tokens: Vec::new(),
            position: 0,
            state,
        });

        while let Some(item) = queue.pop_front() {
            if item.position > text.len() {
                continue;
            }

            let remaining_text = &text[item.position..];
            let execute_result = self.execute_from_state(remaining_text, item.state);

            // Process all matches
            for token in &execute_result.matches {
                let new_position = item.position + token.width;
                let mut new_tokens = item.tokens.clone();
                new_tokens.push(token.clone());

                if new_position == text.len() {
                    final_results.insert(new_tokens, 0); // Assuming 0 is the final state
                } else {
                    queue.push_back(QueueItem {
                        tokens: new_tokens,
                        position: new_position,
                        state: 0, // Assuming 0 is the start state for new tokens
                    });
                }
            }

            // If there's a new state, continue processing
            if let Some(new_state) = execute_result.new_state {
                final_results.insert(item.tokens.clone(), new_state);
            }
        }

        final_results
    }
}

/// Precomputes a map from each state and token sequence to a set of LLM token IDs.
pub fn precompute_llm_token_sets<'a>(
    precompute_map: &BTreeMap<StateID, BTreeMap<Vec<Token>, BTreeMap<&'a [u8], usize>>>,
    llm_token_to_id: &HashMap<&'a [u8], usize>,
) -> BTreeMap<StateID, BTreeMap<Vec<Token>, BTreeSet<usize>>> {
    let mut result: BTreeMap<StateID, BTreeMap<Vec<Token>, BTreeSet<usize>>> = BTreeMap::new();

    for (&state_id, token_sequence_map) in precompute_map {
        let mut sequence_set_map: BTreeMap<Vec<Token>, BTreeSet<usize>> = BTreeMap::new();

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
) -> BTreeMap<StateID, BTreeMap<Vec<Token>, BTreeMap<&'a [u8], usize>>> {
    let mut result = BTreeMap::new();

    for state_id in 0..tokenizer.max_state() {
        let mut state_map: BTreeMap<Vec<Token>, BTreeMap<&'a [u8], usize>> = BTreeMap::new();

        for &llm_token in llm_tokens {
            let sequences = tokenizer.execute_all_from_state(llm_token, state_id);
            for (grammar_token_sequence, end_state) in sequences {
                state_map
                    .entry(grammar_token_sequence)
                    .and_modify(|llm_token_to_state| {
                        llm_token_to_state.insert(llm_token, end_state);
                    })
                    .or_insert_with(|| BTreeMap::from([(llm_token, end_state)]));
            }
        }

        if !state_map.is_empty() {
            result.insert(state_id, state_map);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeSet, HashMap};
    use crate::finite_automata::{eat_u8, DFAState, Regex, DFA};
    use crate::{groups, seq};
    use crate::charmap::TrieMap;
    use crate::u8set::U8Set;

    struct MockTokenizer {
        regex: Regex,
    }

    impl MockTokenizer {
        fn new(regex: Regex) -> Self {
            MockTokenizer { regex }
        }
    }

    impl Tokenizer for MockTokenizer {
        fn execute_from_state(&self, text: &[u8], state: usize) -> ExecuteResult {
            let mut regex_state = self.regex.init_to_state(state);
            regex_state.execute(text);

            let matches = regex_state.matches.iter().map(|(&id, &width)| Token { id, width }).collect();

            ExecuteResult::new(
                matches,
                if regex_state.done { None } else { Some(regex_state.current_state) },
            )
        }

        fn possible_tokens_from_state(&self, state: usize) -> Vec<Token> {
            todo!()
        }

        fn max_state(&self) -> usize {
            self.regex.dfa.states.len()
        }
    }

    #[test]
    fn test_precompute_llm_token_sets() {
        // Define LLM tokens
        let llm_tokens: &[&[u8]] = &[b"a", b"b", b"c", b"ab", b"ac"];

        // Map LLM tokens to unique IDs
        let llm_token_to_id: HashMap<&[u8], usize> = llm_tokens
            .iter()
            .enumerate()
            .map(|(i, &token)| (token, i))
            .collect();

        // Build the expected precompute_map
        // We will manually construct the expected output based on the DFA and LLM tokens

        // Initialize the expected map
        let mut precompute_map: BTreeMap<usize, BTreeMap<Vec<Token>, BTreeMap<&[u8], usize>>> = BTreeMap::new();

        // For DFA state 0 (start state)
        let mut state0_map: BTreeMap<Vec<Token>, BTreeMap<&[u8], usize>> = BTreeMap::new();

        // Analyze each LLM token starting from state 0

        // LLM token "ab"
        // - It matches the grammar token sequence [0] ("ab") and ends in an accepting state
        state0_map
            .entry(vec![Token { id: 0, width: 2 }])
            .or_insert_with(BTreeMap::new)
            .insert(b"ab", /* end state */ 0); // We can use 0 as the end state for simplicity

        // LLM token "ac"
        // - It matches the grammar token sequence [1] ("ac") and ends in an accepting state
        state0_map
            .entry(vec![Token { id: 1, width: 2 }])
            .or_insert_with(BTreeMap::new)
            .insert(b"ac", 0);

        // LLM tokens "a", "b", "c"
        // - These tokens do not produce any complete grammar token sequences starting from state 0
        // - Therefore, they are not included in the expected_precompute_map

        precompute_map.insert(0, state0_map);

        // Perform precompute
        let bitset_map = precompute_llm_token_sets(&precompute_map, &llm_token_to_id);

        // Build the expected bitset_map based on the expected_precompute_map
        let mut expected_bitset_map: BTreeMap<usize, BTreeMap<Vec<Token>, BTreeSet<usize>>> = BTreeMap::new();

        let mut state0_bitset_map: BTreeMap<Vec<Token>, BTreeSet<usize>> = BTreeMap::new();

        // For grammar token sequence [0] ("ab"), the LLM token is "ab" with ID 3
        let mut bitset_ab = BTreeSet::<usize>::new();
        let llm_token_id_ab = *llm_token_to_id.get(b"ab".as_slice()).unwrap();
        bitset_ab.insert(llm_token_id_ab);
        state0_bitset_map.insert(vec![Token { id: 0, width: 2 }], bitset_ab);

        // For grammar token sequence [1] ("ac"), the LLM token is "ac" with ID 4
        let mut bitset_ac = BTreeSet::<usize>::new();
        let llm_token_id_ac = *llm_token_to_id.get(b"ac".as_slice()).unwrap();
        bitset_ac.insert(llm_token_id_ac);
        state0_bitset_map.insert(vec![Token { id: 1, width: 2 }], bitset_ac);

        expected_bitset_map.insert(0, state0_bitset_map);

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

        let tokenizer = MockTokenizer {
            regex: Regex {
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
            }
        };
        assert_eq!(_tokenizer, tokenizer.regex);

        // Define the LLM tokens
        let llm_tokens: &[&[u8]] = &[b"a", b"b", b"c", b"ab", b"bc", b"abc"];

        // Run precompute
        let result = precompute(&tokenizer, llm_tokens);

        // Build the expected output
        let mut state_0: BTreeMap<Vec<Token>, BTreeMap<&[u8], StateID>> = BTreeMap::new();
        state_0.insert(vec![], BTreeMap::from([(b"a".as_slice(), 1), (b"ab", 3)]));
        state_0.insert(vec![Token { id: 0, width: 1 }], BTreeMap::from([(b"a".as_slice(), 0)]));
        state_0.insert(vec![Token { id: 0, width: 1 }, Token { id: 1, width: 1 }], BTreeMap::from([(b"ab".as_slice(), 0)]));
        state_0.insert(vec![Token { id: 1, width: 1 }], BTreeMap::from([(b"b".as_slice(), 0)]));
        state_0.insert(vec![Token { id: 2, width: 2 }], BTreeMap::from([(b"ab".as_slice(), 0)]));
        state_0.insert(vec![Token { id: 3, width: 3 }], BTreeMap::from([(b"abc".as_slice(), 0)]));

        let mut state_1: BTreeMap<Vec<Token>, BTreeMap<&[u8], StateID>> = BTreeMap::new();
        state_1.insert(vec![], BTreeMap::from([(b"b".as_slice(), 3)]));
        state_1.insert(vec![Token { id: 0, width: 1 }], BTreeMap::from([(b"a".as_slice(), 1), (b"ab".as_slice(), 3)]));
        state_1.insert(vec![Token { id: 0, width: 1 }, Token { id: 0, width: 1 }], BTreeMap::from([(b"a".as_slice(), 0)]));
        state_1.insert(vec![Token { id: 0, width: 1 }, Token { id: 0, width: 1 }, Token { id: 1, width: 1 }], BTreeMap::from([(b"ab".as_slice(), 0)]));
        state_1.insert(vec![Token { id: 0, width: 1 }, Token { id: 1, width: 1 }], BTreeMap::from([(b"b".as_slice(), 0)]));
        state_1.insert(vec![Token { id: 0, width: 1 }, Token { id: 2, width: 2 }], BTreeMap::from([(b"ab".as_slice(), 0)]));
        state_1.insert(vec![Token { id: 0, width: 1 }, Token { id: 3, width: 3 }], BTreeMap::from([(b"abc".as_slice(), 0)]));
        state_1.insert(vec![Token { id: 2, width: 2 }], BTreeMap::from([(b"b".as_slice(), 0)]));
        state_1.insert(vec![Token { id: 3, width: 3 }], BTreeMap::from([(b"bc".as_slice(), 0)]));

        let mut state_2: BTreeMap<Vec<Token>, BTreeMap<&[u8], StateID>> = BTreeMap::new();
        state_2.insert(vec![Token { id: 1, width: 1 }], BTreeMap::from([(b"a".as_slice(), 1), (b"ab".as_slice(), 3)]));
        state_2.insert(vec![Token { id: 1, width: 1 }, Token { id: 0, width: 1 }], BTreeMap::from([(b"a".as_slice(), 0)]));
        state_2.insert(vec![Token { id: 1, width: 1 }, Token { id: 0, width: 1 }, Token { id: 1, width: 1 }], BTreeMap::from([(b"ab".as_slice(), 0)]));
        state_2.insert(vec![Token { id: 1, width: 1 }, Token { id: 1, width: 1 }], BTreeMap::from([(b"b".as_slice(), 0)]));
        state_2.insert(vec![Token { id: 1, width: 1 }, Token { id: 2, width: 2 }], BTreeMap::from([(b"ab".as_slice(), 0)]));
        state_2.insert(vec![Token { id: 1, width: 1 }, Token { id: 3, width: 3 }], BTreeMap::from([(b"abc".as_slice(), 0)]));

        let mut state_3: BTreeMap<Vec<Token>, BTreeMap<&[u8], StateID>> = BTreeMap::new();
        state_3.insert(vec![Token { id: 2, width: 2 }], BTreeMap::from([(b"a".as_slice(), 1), (b"ab".as_slice(), 3)]));
        state_3.insert(vec![Token { id: 2, width: 2 }, Token { id: 0, width: 1 }], BTreeMap::from([(b"a".as_slice(), 0)]));
        state_3.insert(vec![Token { id: 2, width: 2 }, Token { id: 0, width: 1 }, Token { id: 1, width: 1 }], BTreeMap::from([(b"ab".as_slice(), 0)]));
        state_3.insert(vec![Token { id: 2, width: 2 }, Token { id: 1, width: 1 }], BTreeMap::from([(b"b".as_slice(), 0)]));
        state_3.insert(vec![Token { id: 2, width: 2 }, Token { id: 2, width: 2 }], BTreeMap::from([(b"ab".as_slice(), 0)]));
        state_3.insert(vec![Token { id: 2, width: 2 }, Token { id: 3, width: 3 }], BTreeMap::from([(b"abc".as_slice(), 0)]));
        state_3.insert(vec![Token { id: 3, width: 3 }], BTreeMap::from([(b"c".as_slice(), 0)]));

        let mut state_4: BTreeMap<Vec<Token>, BTreeMap<&[u8], StateID>> = BTreeMap::new();
        state_4.insert(vec![Token { id: 3, width: 3 }], BTreeMap::from([(b"a".as_slice(), 1), (b"ab".as_slice(), 3)]));
        state_4.insert(vec![Token { id: 3, width: 3 }, Token { id: 0, width: 1 }], BTreeMap::from([(b"a".as_slice(), 0)]));
        state_4.insert(vec![Token { id: 3, width: 3 }, Token { id: 0, width: 1 }, Token { id: 1, width: 1 }], BTreeMap::from([(b"ab".as_slice(), 0)]));
        state_4.insert(vec![Token { id: 3, width: 3 }, Token { id: 1, width: 1 }], BTreeMap::from([(b"b".as_slice(), 0)]));
        state_4.insert(vec![Token { id: 3, width: 3 }, Token { id: 2, width: 2 }], BTreeMap::from([(b"ab".as_slice(), 0)]));
        state_4.insert(vec![Token { id: 3, width: 3 }, Token { id: 3, width: 3 }], BTreeMap::from([(b"abc".as_slice(), 0)]));

        let mut expected: BTreeMap<StateID, BTreeMap<Vec<Token>, BTreeMap<&[u8], StateID>>> = BTreeMap::new();
        expected.insert(0, state_0);
        expected.insert(1, state_1);
        expected.insert(2, state_2);
        expected.insert(3, state_3);
        expected.insert(4, state_4);

        expected.retain(|_, v| !v.is_empty());

        for state_id in expected.keys().chain(result.keys()) {
            assert_eq!(expected.get(state_id), result.get(state_id), "State {}", state_id);
        }
    }
}