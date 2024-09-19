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
    /// Executes the tokenizer on the entire string and returns all possible token sequences and final states.
    /// TODO: improve this explanation
    fn execute_all_from_state(&self, text: &[u8], state: StateID) -> BTreeMap<Vec<TokenID>, StateID> {
        // Implement using recursion? For each end position, start a new instance of the tokenizer and execute it on the remaining text.
        // Return all possible token sequences and the final state they lead to.
        todo!()
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

pub fn precompute<'a>(tokenizer: &impl Tokenizer, llm_tokens: &[&'a [u8]]) -> BTreeMap<StateID, BTreeMap<Vec<TokenID>, (StateID, Vec<&'a [u8]>)>> {
    todo!()
}

/// Tests for the precompute module.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::finite_automata::eat_u8;
    use crate::{groups, seq};

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
        assert_eq!(matches.get(&1), Some(&0)); // 'a' matched at position 1
        assert_eq!(matches.get(&2), Some(&2)); // 'ab' matched at position 2

        // Get all possible token sequences
        let results = tokenizer.execute_all_from_state(b"ab", 0);

        // The two possible token sequences are [0, 1] or [2]. In both cases, the final state should be the initial state.
        assert_eq!(results.get(&vec![0, 1]), Some(&0));
        assert_eq!(results.get(&vec![2]), Some(&0));

        // The third case is where we match the first two characters of token 3, but not the third yet.
        assert_eq!(results.get(&vec![]), new_state.as_ref());
    }

    #[test]
    fn test_precompute() {
        let tokenizer = groups![
            eat_u8(b'a'), // Token 0: 'a'
            eat_u8(b'b'), // Token 1: 'b'
            seq![eat_u8(b'a'), eat_u8(b'b')], // Token 2: 'ab'
            seq![eat_u8(b'a'), eat_u8(b'b'), eat_u8(b'c')], // Token 3: 'abc'
        ].build();

        let llm_tokens: &[&[u8]] = &[b"a", b"b", b"c", b"ab", b"bc", b"abc"];

        let result = precompute(&tokenizer, llm_tokens);

        let expected: BTreeMap<StateID, BTreeMap<Vec<TokenID>, (StateID, Vec<&[u8]>)>> = todo!();

        assert_eq!(result, expected);
    }
}