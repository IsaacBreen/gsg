// src/precompute.rs
use crate::finite_automata::{Regex};
use std::collections::{BTreeMap, BTreeSet};

type TokenID = usize;
type StateID = usize;
type Position = usize;

pub struct ExecuteResult {
    pub matches: BTreeMap<TokenID, Position>,
    pub new_state: Option<StateID>,
}

pub trait Tokenizer: Sized {
    fn execute_from_state(&self, text: &[u8], state: StateID) -> ExecuteResult;
    fn tokens_accessible_from_state(&self, state: StateID) -> Vec<TokenID>;
    fn max_state(&self) -> StateID;
    /// Executes the tokenizer on the entire string and returns all possible token sequences and final states.
    /// TODO: improve this explanation
    fn execute_all_from_state(&self, text: &[u8], state: StateID) -> BTreeMap<Vec<TokenID>, StateID> {
        // Implement using recursion? For each end position, start a new instance of the tokenizer and execute it on the remaining text.
        // Return all possible token sequences and the final state they lead to.

        // Note: is *is* possible to do this using just `execute_from_state` - i.e. without access to private fields of the implementing type.
        //  How? `ExecuteResult` tells us, from the given state, which matches are possible and what the final state is.
        //  Here, the 'final' state is the state reached by running the tokenizer up to the end of the string.
        //  It is NOT the state reached for which a finalizer was encountered that triggered a match.
        //
        //  How do we use this? Consider a single (token, position) pair in a `ExecuteResult`.
        //  If the position is the end of the string, the state `new_state` is the final state.
        //  We add an entry to the final results map with the token sequence and the final state.
        //  If the position is not the end of the string, then we need to keep track of the token matched and then
        //  run the tokenizer again from the initial state (by convention, the initial state is 0). We keep doing this until.
        //  we reach the end of the string.
        struct QueueItem {
            tokens: Vec<TokenID>,
            position: usize,
            state: StateID,
        }

        let mut queue: Vec<QueueItem> = vec![QueueItem { tokens: vec![], position: 0, state }];
        let mut final_results: BTreeMap<Vec<TokenID>, StateID> = BTreeMap::new();

        while let Some(QueueItem { tokens, position, state }) = queue.pop() {
            let results = self.execute_from_state(&text[position..], state);

            for (&token, &offset) in &results.matches {
                let new_position = position + offset;
                let mut new_tokens = tokens.clone();
                new_tokens.push(token);

                if new_position == text.len() {
                    final_results.insert(new_tokens, 0);
                } else {
                    queue.push(QueueItem { tokens: new_tokens, position: new_position, state: 0 });
                }
            }

            if let Some(new_state) = results.new_state {
                final_results.insert(tokens, new_state);
            }
        }

        final_results
    }
}

impl Tokenizer for Regex {
    fn execute_from_state(&self, text: &[u8], state: StateID) -> ExecuteResult {
        let mut regex_state = self.init_to_state(state);
        regex_state.execute(text);
        ExecuteResult {
            matches: regex_state.matches,
            new_state: if regex_state.done { None } else { Some(regex_state.current_state) },
        }
    }

    fn tokens_accessible_from_state(&self, state: StateID) -> Vec<TokenID> {
        let regex_state = self.init_to_state(state);
        regex_state.possible_group_ids().into_iter().collect()
    }

    fn max_state(&self) -> StateID {
        self.dfa.states.len()
    }
}

pub fn precompute<'a>(
    tokenizer: &impl Tokenizer,
    llm_tokens: &[&'a [u8]],
) -> BTreeMap<StateID, BTreeMap<Vec<TokenID>, (StateID, BTreeSet<&'a [u8]>)>> {
    let mut result = BTreeMap::new();

    for state_id in 0..tokenizer.max_state() {
        let mut state_map: BTreeMap<Vec<TokenID>, (StateID, BTreeSet<&'a [u8]>)> = BTreeMap::new();

        for &llm_token in llm_tokens {
            for (grammar_token_sequence, end_state) in tokenizer.execute_all_from_state(llm_token, state_id) {
                state_map
                    .entry(grammar_token_sequence)
                    .and_modify(|(existing_end_state, llm_token_subset)| {
                        assert!(*existing_end_state == end_state);
                        llm_token_subset.insert(llm_token);
                    })
                    .or_insert_with(|| (end_state, BTreeSet::from([llm_token])));
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
    use std::collections::BTreeSet;
    use super::*;
    use crate::finite_automata::{eat_u8, DFAState, DFA};
    use crate::{groups, seq};
    use crate::charmap::TrieMap;

    #[test]
    fn test_regex_tokenizer() {
        // Define some simple regexes for testing
        let tokenizer = groups![
            eat_u8(b'a'), // Token 0: 'a'
            eat_u8(b'b'), // Token 1: 'b'
            seq![eat_u8(b'a'), eat_u8(b'b')], // Token 2: 'ab'
            seq![eat_u8(b'a'), eat_u8(b'b'), eat_u8(b'c')], // Token 3: 'abc'
        ].build();

        // Execute the tokenizer on a string
        let ExecuteResult { matches, new_state } = tokenizer.execute_from_state(b"ab", 0);

        // Check the results
        assert_eq!(matches, BTreeMap::from([
            (0, 1), // 'a' matched at position 1
            (2, 2), // 'ab' matched at position 2
        ]));

        // Get all possible token sequences
        let results = tokenizer.execute_all_from_state(b"ab", 0);

        assert_eq!(results, BTreeMap::from([
            // The two possible token sequences are [0, 1] or [2]. In both cases, the final state should be the initial state.
            (vec![0, 1], 0),
            (vec![2], 0),
            // The third case is where we match the first two characters of token 3, but not the third yet.
            (vec![], new_state.unwrap()),
        ]));
    }

    #[test]
    fn test_precompute() {
        // let tokenizer = groups![
        //     eat_u8(b'a'), // Token 0: 'a'
        //     eat_u8(b'b'), // Token 1: 'b'
        //     seq![eat_u8(b'a'), eat_u8(b'b')], // Token 2: 'ab'
        //     seq![eat_u8(b'a'), eat_u8(b'b'), eat_u8(b'c')], // Token 3: 'abc'
        // ].build();

        let tokenizer = Regex {
            dfa: DFA {
                states: vec![
                    DFAState {
                        transitions: TrieMap::from_iter(vec![(b'a', 1), (b'b', 2)]),
                        finalizers: BTreeSet::new(),
                    },
                    DFAState {
                        transitions: TrieMap::from_iter(vec![(b'b', 3)]),
                        finalizers: BTreeSet::from([0]),
                    },
                    DFAState {
                        transitions: TrieMap::new(),
                        finalizers: BTreeSet::from([1]),
                    },
                    DFAState {
                        transitions: TrieMap::from_iter(vec![(b'c', 4)]),
                        finalizers: BTreeSet::from([2]),
                    },
                    DFAState {
                        transitions: TrieMap::new(),
                        finalizers: BTreeSet::from([3]),
                    },
                ],
                start_state: 0,
            },
        };

        // Define the LLM tokens
        let llm_tokens: &[&[u8]] = &[b"a", b"b", b"c", b"ab", b"bc", b"abc"];

        // Run precompute
        let result = precompute(&tokenizer, llm_tokens);

        // Build the expected output
        let mut state_0: BTreeMap<Vec<TokenID>, (StateID, BTreeSet<&[u8]>)> = BTreeMap::new();
        state_0.insert(vec![0], (0, BTreeSet::from([b"a".as_slice()])));
        state_0.insert(vec![1], (0, BTreeSet::from([b"b".as_slice(), b"bc"])));
        state_0.insert(vec![0, 1], (0, BTreeSet::from([b"ab".as_slice(), b"abc"])));
        state_0.insert(vec![2], (0, BTreeSet::from([b"ab".as_slice()])));
        state_0.insert(vec![3], (0, BTreeSet::from([b"abc".as_slice()])));
        state_0.insert(vec![0, 2], (0, BTreeSet::from([b"abc".as_slice()])));

        let mut state_1: BTreeMap<Vec<TokenID>, (StateID, BTreeSet<&[u8]>)> = BTreeMap::new();
        state_1.insert(vec![0], (3, BTreeSet::from([b"b".as_slice()])));
        state_1.insert(vec![0, 3], (0, BTreeSet::from([b"bc".as_slice()])));

        let mut state_3: BTreeMap<Vec<TokenID>, (StateID, BTreeSet<&[u8]>)> = BTreeMap::new();
        state_3.insert(vec![3], (0, BTreeSet::from([b"c".as_slice()])));

        let mut expected: BTreeMap<StateID, BTreeMap<Vec<TokenID>, (StateID, BTreeSet<&[u8]>)>> =
            BTreeMap::new();
        expected.insert(0, state_0);
        expected.insert(1, state_1);
        expected.insert(3, state_3);

        assert_eq!(result, expected);
    }
}
