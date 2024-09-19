// src/precompute.rs
use crate::finite_automata::{Regex};
use crate::frozenset::FrozenSet;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

type TokenID = usize;
type StateID = usize;
type Position = usize;

pub struct ExecuteResult {
    pub matches: HashMap<TokenID, Position>,
    pub new_state: Option<StateID>,
}

pub trait Tokenizer: Sized {
    fn execute_from_state(&self, text: &[u8], state: StateID) -> ExecuteResult;
    fn tokens_accessible_from_state(&self, state: StateID) -> Vec<TokenID>;
    fn max_state(&self) -> StateID;
    /// Executes the tokenizer on the entire string and returns all possible token sequences and final states.
    fn execute_all_from_state(&self, text: &[u8], state: StateID) -> HashMap<Vec<TokenID>, StateID>;
}

impl ExecuteResult {
    pub fn new(matches: HashMap<TokenID, Position>, new_state: Option<StateID>) -> Self {
        ExecuteResult { matches, new_state }
    }
}

/// Precomputes, for each tokenizer state, all possible token sequences that can be produced
/// by applying any of the given LLM tokens, along with the resulting new tokenizer states.
/// Returns a mapping from StateID to a mapping of token sequences to their resulting StateID and LLM tokens.
pub fn precompute<'a>(
    tokenizer: &impl Tokenizer,
    llm_tokens: &[&'a [u8]],
) -> HashMap<StateID, HashMap<Vec<TokenID>, (StateID, Vec<&'a [u8]>)>> {
    let mut result: HashMap<StateID, HashMap<Vec<TokenID>, (StateID, Vec<&[u8]>)>> = HashMap::new();

    let max_state = tokenizer.max_state();

    for state in 0..=max_state {
        let mut token_seq_map: HashMap<Vec<TokenID>, (StateID, Vec<&[u8]>)> = HashMap::new();

        for &llm_token in llm_tokens {
            // Execute the tokenizer with the LLM token's byte sequence from the current state
            let exec_result = tokenizer.execute_from_state(llm_token, state);

            // Extract the token sequence
            let mut token_seq: Vec<TokenID> = exec_result.matches.keys().cloned().collect();
            token_seq.sort(); // Ensure consistent ordering

            // Determine the new state
            let new_state = exec_result.new_state.unwrap_or(state);

            // Insert into the token_seq_map
            token_seq_map
                .entry(token_seq)
                .or_insert_with(|| (new_state, Vec::new()))
                .1
                .push(llm_token);
        }

        result.insert(state, token_seq_map);
    }

    result
}

/// Tests for the precompute module.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::finite_automata::{eat_u8, Expr, Regex};
    use std::collections::HashSet;

    #[test]
    fn test_precompute() {
        // Define a simple tokenizer with tokens: 'a', 'ab', 'abc', 'b', 'c'
        // This will create a DFA with states corresponding to these tokens
        let expr_a = eat_u8(b'a');
        let expr_ab = Expr::Seq(vec![eat_u8(b'a'), eat_u8(b'b')]);
        let expr_abc = Expr::Seq(vec![eat_u8(b'a'), eat_u8(b'b'), eat_u8(b'c')]);
        let expr_b = eat_u8(b'b');
        let expr_c = eat_u8(b'c');

        // Define groups for each token
        let regex = crate::finite_automata::ExprGroups {
            groups: vec![
                expr_a.into(),
                expr_ab.into(),
                expr_abc.into(),
                expr_b.into(),
                expr_c.into(),
            ],
        }
        .build();

        // Define a small set of LLM tokens as byte sequences
        let llm_tokens: Vec<&[u8]> = vec![
            b"a",    // Should match token 0 ('a') and possibly start tokens 1 and 2
            b"ab",   // Should match token 1 ('ab') and possibly start token 2
            b"abc",  // Should match token 2 ('abc')
            b"b",    // Should match token 3 ('b')
            b"c",    // Should match token 4 ('c')
            b"d",    // Invalid token
            b"",     // Empty token
        ];

        // Run precompute
        let precompute_map = precompute(&regex, &llm_tokens);

        // Expected mapping:
        // State 0: Start state
        // - 'a' leads to state where 'a' is matched (state_a)
        // - 'ab' leads to state where 'ab' is matched (state_ab)
        // - 'abc' leads to state where 'abc' is matched (state_abc)
        // - 'b' leads to state where 'b' is matched (state_b)
        // - 'c' leads to state where 'c' is matched (state_c)
        // - 'd' invalid, no token
        // - '' leads to no token, stays in state 0

        // Define expected token sequences per state
        // For simplicity, assume token IDs:
        // 0: 'a', 1: 'ab', 2: 'abc', 3: 'b', 4: 'c'

        // Collect the expected results
        let mut expected: HashMap<StateID, HashMap<Vec<TokenID>, (StateID, Vec<&[u8]>)>> =
            HashMap::new();

        // State 0
        let state0 = 0;
        let mut state0_map: HashMap<Vec<TokenID>, (StateID, Vec<&[u8]>)> = HashMap::new();
        state0_map.insert(vec![0], (1, vec![b"a"]));      // 'a' leads to state 1
        state0_map.insert(vec![1], (2, vec![b"ab"]));     // 'ab' leads to state 2
        state0_map.insert(vec![2], (3, vec![b"abc"]));    // 'abc' leads to state 3
        state0_map.insert(vec![3], (4, vec![b"b"]));      // 'b' leads to state 4
        state0_map.insert(vec![4], (5, vec![b"c"]));      // 'c' leads to state 5
        state0_map.insert(vec![], (0, vec![b""]));        // '' leads to state 0
        // 'd' is invalid, should not be present
        expected.insert(state0, state0_map);

        // State 1: After matching 'a'
        let state1 = 1;
        let mut state1_map: HashMap<Vec<TokenID>, (StateID, Vec<&[u8]>)> = HashMap::new();
        state1_map.insert(vec![1], (2, vec![b"b"]));      // 'b' after 'a' leads to state 2
        state1_map.insert(vec![0], (1, vec![b"a"]));      // 'a' after 'a' leads to state 1
        state1_map.insert(vec![], (1, vec![b""]));        // '' leads to state 1
        // 'c', 'd' invalid after 'a'
        expected.insert(state1, state1_map);

        // State 2: After matching 'ab'
        let state2 = 2;
        let mut state2_map: HashMap<Vec<TokenID>, (StateID, Vec<&[u8]>)> = HashMap::new();
        state2_map.insert(vec![2], (3, vec![b"c"]));      // 'c' after 'ab' leads to state 3
        state2_map.insert(vec![0], (1, vec![b"a"]));      // 'a' after 'ab' leads to state 1
        state2_map.insert(vec![], (2, vec![b""]));        // '' leads to state 2
        // 'b', 'd' invalid after 'ab'
        expected.insert(state2, state2_map);

        // State 3: After matching 'abc'
        let state3 = 3;
        let mut state3_map: HashMap<Vec<TokenID>, (StateID, Vec<&[u8]>)> = HashMap::new();
        state3_map.insert(vec![0], (1, vec![b"a"]));      // 'a' after 'abc' leads to state 1
        state3_map.insert(vec![1], (2, vec![b"ab"]));     // 'ab' after 'abc' leads to state 2
        state3_map.insert(vec![3], (4, vec![b"b"]));      // 'b' after 'abc' leads to state 4
        state3_map.insert(vec![4], (5, vec![b"c"]));      // 'c' after 'abc' leads to state 5
        state3_map.insert(vec![], (3, vec![b""]));        // '' leads to state 3
        // 'd' invalid after 'abc'
        expected.insert(state3, state3_map);

        // State 4: After matching 'b'
        let state4 = 4;
        let mut state4_map: HashMap<Vec<TokenID>, (StateID, Vec<&[u8]>)> = HashMap::new();
        state4_map.insert(vec![0], (1, vec![b"a"]));      // 'a' after 'b' leads to state 1
        state4_map.insert(vec![3], (4, vec![b"b"]));      // 'b' after 'b' leads to state 4
        state4_map.insert(vec![4], (5, vec![b"c"]));      // 'c' after 'b' leads to state 5
        state4_map.insert(vec![], (4, vec![b""]));        // '' leads to state 4
        // 'ab', 'abc', 'd' invalid after 'b'
        expected.insert(state4, state4_map);

        // State 5: After matching 'c'
        let state5 = 5;
        let mut state5_map: HashMap<Vec<TokenID>, (StateID, Vec<&[u8]>)> = HashMap::new();
        state5_map.insert(vec![0], (1, vec![b"a"]));      // 'a' after 'c' leads to state 1
        state5_map.insert(vec![1], (2, vec![b"ab"]));     // 'ab' after 'c' leads to state 2
        state5_map.insert(vec![3], (4, vec![b"b"]));      // 'b' after 'c' leads to state 4
        state5_map.insert(vec![4], (5, vec![b"c"]));      // 'c' after 'c' leads to state 5
        state5_map.insert(vec![], (5, vec![b""]));        // '' leads to state 5
        // 'd' invalid after 'c'
        expected.insert(state5, state5_map);

        // Now, compare the precompute_map with expected
        for (&state, token_map) in expected.iter() {
            let pre_map = precompute_map.get(&state).expect("State missing in precompute_map");

            // Check that the number of token sequences matches
            assert_eq!(
                pre_map.len(),
                token_map.len(),
                "State {}: Number of token sequences mismatch",
                state
            );

            for (token_seq, (expected_new_state, llm_tokens)) in token_map.iter() {
                let (pre_new_state, pre_llm_tokens) = pre_map
                    .get(token_seq)
                    .expect(&format!(
                        "State {}: Token sequence {:?} missing in precompute_map",
                        state, token_seq
                    ));

                // Check new state
                assert_eq!(
                    pre_new_state, expected_new_state,
                    "State {}: New state mismatch for token sequence {:?}",
                    state, token_seq
                );

                // Check LLM tokens
                let expected_set: HashSet<&[u8]> = llm_tokens.iter().cloned().collect();
                let pre_set: HashSet<&[u8]> = pre_llm_tokens.iter().cloned().collect();
                assert_eq!(
                    pre_set, expected_set,
                    "State {}: LLM tokens mismatch for token sequence {:?}",
                    state, token_seq
                );
            }
        }

        // Additionally, ensure that no unexpected states are present
        assert_eq!(
            precompute_map.len(),
            expected.len(),
            "Number of states in precompute_map does not match expected"
        );
    }
}
