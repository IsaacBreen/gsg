// src/precompute.rs
use std::collections::{BTreeMap, HashMap};
use crate::bitset256::BitSet256;
use crate::finite_automata::GroupID;

/// Represents the result of executing the tokenizer from a specific state.
pub struct ExecuteResult {
    pub matches: BTreeMap<GroupID, usize>, // GroupID to position
    pub new_state: Option<usize>,
}

/// Trait defining the tokenizer behavior.
pub trait Tokenizer: Sized {
    /// Executes the tokenizer on the given text starting from the specified state.
    fn execute_from_state(&self, text: &[u8], state: usize) -> ExecuteResult;

    /// Returns the list of token IDs accessible from the given state.
    fn tokens_accessible_from_state(&self, state: usize) -> Vec<GroupID>;

    /// Returns the maximum state ID in the DFA.
    fn max_state(&self) -> usize;

    /// Executes the tokenizer on the entire string and returns all possible token sequences and final states.
    fn execute_all_from_state(
        &self,
        text: &[u8],
        state: usize,
    ) -> BTreeMap<Vec<GroupID>, usize> {
        use std::collections::VecDeque;

        // Define a queue item structure
        struct QueueItem {
            tokens: Vec<GroupID>,
            position: usize,
            state: usize,
        }

        let mut queue: VecDeque<QueueItem> = VecDeque::new();
        let mut final_results: BTreeMap<Vec<GroupID>, usize> = BTreeMap::new();

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
            for (&token_id, &offset) in &execute_result.matches {
                let new_position = item.position + offset;
                let mut new_tokens = item.tokens.clone();
                new_tokens.push(token_id);

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

/// Precomputes a map from each state and token sequence to a bitset of LLM token IDs.
pub fn precompute_llm_token_bitsets<'a>(
    precompute_map: &BTreeMap<usize, BTreeMap<Vec<GroupID>, BTreeMap<&'a [u8], usize>>>,
    llm_token_to_id: &HashMap<&'a [u8], usize>,
    _total_llm_tokens: usize,
) -> BTreeMap<usize, BTreeMap<Vec<GroupID>, BitSet256>> {
    let mut result: BTreeMap<usize, BTreeMap<Vec<GroupID>, BitSet256>> = BTreeMap::new();

    for (&state_id, token_sequence_map) in precompute_map {
        let mut sequence_bitset_map: BTreeMap<Vec<GroupID>, BitSet256> = BTreeMap::new();

        for (token_sequence, llm_token_state_map) in token_sequence_map {
            let mut bitset = BitSet256::new();
            for (&llm_token, &_next_state) in llm_token_state_map {
                if let Some(&llm_token_id) = llm_token_to_id.get(llm_token) {
                    bitset.set_bit(llm_token_id as u8);
                }
            }
            sequence_bitset_map.insert(token_sequence.clone(), bitset);
        }

        result.insert(state_id, sequence_bitset_map);
    }

    result
}

/// Precomputes a map from state -> token sequence -> LLM token -> state.
pub fn precompute<'a>(
    tokenizer: &impl Tokenizer,
    llm_tokens: &[&'a [u8]],
) -> BTreeMap<usize, BTreeMap<Vec<GroupID>, BTreeMap<&'a [u8], usize>>> {
    let mut result: BTreeMap<usize, BTreeMap<Vec<GroupID>, BTreeMap<&'a [u8], usize>>> = BTreeMap::new();

    for state_id in 0..tokenizer.max_state() {
        let mut state_map: BTreeMap<Vec<GroupID>, BTreeMap<&'a [u8], usize>> = BTreeMap::new();

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
    use crate::bitset256::BitSet256;
    use std::collections::HashMap;
    use crate::finite_automata::{eat_u8, Regex};
    use crate::{groups, seq};

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
            ExecuteResult {
                matches: regex_state.matches.clone(),
                new_state: if regex_state.done { None } else { Some(regex_state.current_state) },
            }
        }

        fn tokens_accessible_from_state(&self, state: usize) -> Vec<GroupID> {
            let regex_state = self.regex.init_to_state(state);
            regex_state.possible_group_ids().into_iter().collect()
        }

        fn max_state(&self) -> usize {
            self.regex.dfa.states.len()
        }
    }

    #[test]
    fn test_precompute_llm_token_bitsets() {
        // Define a simple regex for testing: "ab" or "ac"
        let expr = groups![
            seq![eat_u8(b'a'), eat_u8(b'b')], // Token 0: "ab"
            seq![eat_u8(b'a'), eat_u8(b'c')], // Token 1: "ac"
        ];
        let regex = expr.build();

        // Create a mock tokenizer
        let tokenizer = MockTokenizer::new(regex);

        // Define LLM tokens
        let llm_tokens: &[&[u8]] = &[b"a", b"b", b"c", b"ab", b"ac"];

        // Map LLM tokens to unique IDs
        let llm_token_to_id: HashMap<&[u8], usize> = llm_tokens
            .iter()
            .enumerate()
            .map(|(i, &token)| (token, i))
            .collect();

        // Perform precompute
        let precompute_map = precompute(&tokenizer, llm_tokens);
        let bitset_map = precompute_llm_token_bitsets(&precompute_map, &llm_token_to_id, llm_tokens.len());

        // Verify the results
        // For state 0, matching "ab" and "ac"
        assert!(bitset_map.contains_key(&0));
        let state0_map = bitset_map.get(&0).unwrap();
        // Token sequence [0] corresponds to "ab" which maps to LLM token "ab" (ID 3)
        assert!(state0_map.contains_key(&vec![0]));
        let bitset_ab = state0_map.get(&vec![0]).unwrap();
        assert!(bitset_ab.is_set(3));

        // Token sequence [1] corresponds to "ac" which maps to LLM token "ac" (ID 4)
        assert!(state0_map.contains_key(&vec![1]));
        let bitset_ac = state0_map.get(&vec![1]).unwrap();
        assert!(bitset_ac.is_set(4));

        // There should be no other token sequences for state 0
        assert_eq!(state0_map.len(), 2);
    }

    #[test]
    fn test_precompute() {
        // Define a simple regex: "a" or "b"
        let expr = groups![
            eat_u8(b'a'), // Token 0: "a"
            eat_u8(b'b'), // Token 1: "b"
        ];
        let regex = expr.build();

        // Create a mock tokenizer
        let tokenizer = MockTokenizer::new(regex);

        // Define LLM tokens
        let llm_tokens: &[&[u8]] = &[b"a", b"b", b"c", b"ab"];

        // Perform precompute
        let precompute_map = precompute(&tokenizer, llm_tokens);

        // Expected map:
        // State 0:
        //   [0] -> "a" -> state 0
        //   [1] -> "b" -> state 0
        //
        // Since "c" and "ab" don't match any tokens, they should not be present.

        let expected_map: BTreeMap<usize, BTreeMap<Vec<GroupID>, BTreeMap<&[u8], usize>>> =
            BTreeMap::from([
                (
                    0,
                    BTreeMap::from([
                        (vec![0], BTreeMap::from([(&b"a"[..], 0)])),
                        (vec![1], BTreeMap::from([(&b"b"[..], 0)])),
                    ]),
                ),
            ]);

        assert_eq!(precompute_map, expected_map);
    }
}
