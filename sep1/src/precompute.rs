// src/precompute.rs
use crate::finite_automata::{Regex, RegexState};
use std::collections::BTreeMap;
use std::hash::Hash;

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
    /// Executes the tokenizer on the entire string and returns all possible token sequences.
    fn execute_all_from_state(&self, text: &[u8], state: StateID) -> Vec<TokenID>;
}

impl Tokenizer for Regex {
    fn execute_from_state(&self, text: &[u8], state: StateID) -> ExecuteResult {
        let mut regex_state = self.init_to_state(state);
        regex_state.execute(text);
        ExecuteResult {
            matches: regex_state.matches.clone(),
            new_state: if regex_state.done() {
                None
            } else {
                Some(regex_state.current_state)
            },
        }
    }

    fn tokens_accessible_from_state(&self, state: StateID) -> Vec<TokenID> {
        let state_data = &self.dfa.states[state];
        state_data
            .finalizers
            .iter()
            .cloned()
            .collect::<Vec<TokenID>>()
    }

    fn max_state(&self) -> StateID {
        self.dfa.states.len()
    }

    fn execute_all_from_state(&self, text: &[u8], state: StateID) -> Vec<TokenID> {
        let mut regex_state = self.init_to_state(state);
        regex_state.execute(text);
        regex_state.matches.keys().cloned().collect()
    }
}

pub fn precompute<'a>(
    tokenizer: &Regex,
    llm_tokens: &[&'a [u8]],
) -> BTreeMap<&'a [u8], Vec<TokenID>> {
    let mut result = BTreeMap::new();
    for &llm_token in llm_tokens {
        let matches = tokenizer.execute_all_from_state(llm_token, tokenizer.dfa.start_state);
        if !matches.is_empty() {
            result.insert(llm_token, matches);
        }
    }
    result
}

/// Tests for the precompute module.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::finite_automata::{eat_u8};
    use crate::{groups, seq};

    #[test]
    fn test_regex_tokenizer() {
        // Define some simple regexes for testing
        let tokenizer = groups![
            eat_u8(b'a'), // Token 0: 'a'
            eat_u8(b'b'), // Token 1: 'b'
            seq![eat_u8(b'a'), eat_u8(b'b')], // Token 2: 'ab'
            seq![eat_u8(b'a'), eat_u8(b'b'), eat_u8(b'c')], // Token 3: 'abc'
        ]
        .build();

        // Execute the tokenizer on a string
        let ExecuteResult { matches, new_state } = tokenizer.execute_from_state(b"ab", 0);

        // Check the results
        assert_eq!(matches.get(&0), Some(&1)); // 'a' matched at position 1
        assert_eq!(matches.get(&2), Some(&2)); // 'ab' matched at position 2

        // Get all possible token sequences
        let matches = tokenizer.execute_all_from_state(b"ab", 0);

        // The possible tokens are 0 ('a') and 2 ('ab')
        assert_eq!(matches, vec![0, 2]);
    }

    #[test]
    fn test_precompute() {
        let tokenizer = groups![
            eat_u8(b'a'), // Token 0: 'a'
            eat_u8(b'b'), // Token 1: 'b'
            seq![eat_u8(b'a'), eat_u8(b'b')], // Token 2: 'ab'
            seq![eat_u8(b'a'), eat_u8(b'b'), eat_u8(b'c')], // Token 3: 'abc'
        ]
        .build();

        let llm_tokens: &[&[u8]] = &[b"a", b"b", b"c", b"ab", b"bc", b"abc"];

        let result = precompute(&tokenizer, llm_tokens);

        let mut expected: BTreeMap<&[u8], Vec<TokenID>> = BTreeMap::new();
        expected.insert(b"a", vec![0]);
        expected.insert(b"b", vec![1]);
        expected.insert(b"ab", vec![0, 2]); // 'a' and 'ab' can match 'ab'
        expected.insert(b"abc", vec![0, 2, 3]); // 'a', 'ab', and 'abc' can match 'abc'

        assert_eq!(result, expected);
    }
}
