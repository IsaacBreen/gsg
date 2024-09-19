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

        let mut queue: Vec<QueueItem> = vec![];
        let mut final_results: BTreeMap<Vec<TokenID>, StateID> = BTreeMap::new();

        queue.push(QueueItem { tokens: vec![], position: 0, state });

        while let Some(QueueItem { tokens, position: start_position, state }) = queue.pop() {
            let mut results = self.execute_from_state(&text[start_position..], state);
            for (token, offset) in results.matches {
                let position = start_position + offset;
                let mut new_tokens = tokens.clone();
                new_tokens.push(token);
                if position == text.len() {
                    final_results.insert(new_tokens, 0);
                } else {
                    queue.push(QueueItem { tokens: new_tokens, position, state: 0 });
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
) -> BTreeMap<StateID, BTreeMap<&'a [u8], BTreeMap<Vec<TokenID>, StateID>>> {
    let mut result = BTreeMap::new();

    for state in 0..tokenizer.max_state() {
        let mut state_map = BTreeMap::new();
        for &llm_token in llm_tokens {
            let token_sequences = tokenizer.execute_all_from_state(llm_token, state);
            if !token_sequences.is_empty() {
                state_map.insert(llm_token, token_sequences);
            }
        }
        if !state_map.is_empty() {
            result.insert(state, state_map);
        }
    }
    result
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
        // Define a tokenizer with overlapping tokens
        let tokenizer = groups![
            eat_u8(b'a'),                           // Token 0: 'a'
            eat_u8(b'b'),                           // Token 1: 'b'
            seq![eat_u8(b'a'), eat_u8(b'b')],       // Token 2: 'ab'
            seq![eat_u8(b'a'), eat_u8(b'b'), eat_u8(b'c')], // Token 3: 'abc'
        ].build();

        let llm_tokens: &[&[u8]] = &[b"a", b"b", b"c", b"ab", b"abc", b"bc"];

        let result = precompute(&tokenizer, llm_tokens);

        // Manually compute the expected output
        use std::collections::BTreeMap;

        let mut expected = BTreeMap::new();

        // For each state, build the expected mappings
        // State 0
        let mut state_0_map: BTreeMap<&[u8], BTreeMap<Vec<TokenID>, StateID>> = BTreeMap::new();

        // LLM Token: b"a"
        state_0_map.insert(b"a", BTreeMap::from([
            (vec![0], 0),    // Matched 'a', Token 0
            (vec![], 1),     // Partial match, State 1
        ]));

        // LLM Token: b"b"
        state_0_map.insert(b"b", BTreeMap::from([
            (vec![1], 0),    // Matched 'b', Token 1
        ]));

        // LLM Token: b"ab"
        state_0_map.insert(b"ab", BTreeMap::from([
            (vec![2], 0),    // Matched 'ab', Token 2
            (vec![0, 1], 0), // Matched 'a' and 'b', Tokens 0 and 1
            (vec![], 3),     // Partial match, State 3
        ]));

        // LLM Token: b"abc"
        state_0_map.insert(b"abc", BTreeMap::from([
            (vec![3], 0),    // Matched 'abc', Token 3
            (vec![0, 1], 0), // Matched 'a' and 'b', Tokens 0 and 1
        ]));

        // LLM Token: b"bc"
        state_0_map.insert(b"bc", BTreeMap::from([
            (vec![1], 0),    // Matched 'b', Token 1
        ]));

        expected.insert(0, state_0_map);

        // State 1
        let mut state_1_map: BTreeMap<&[u8], BTreeMap<Vec<TokenID>, StateID>> = BTreeMap::new();

        // LLM Token: b"b"
        state_1_map.insert(b"b", BTreeMap::from([
            (vec![2], 0),    // Matched 'ab', Token 2
            (vec![], 3),     // Partial match, State 3
        ]));

        // LLM Token: b"bc"
        state_1_map.insert(b"bc", BTreeMap::from([
            (vec![3], 0),    // Matched 'abc', Token 3
        ]));

        expected.insert(1, state_1_map);

        // State 3
        let mut state_3_map: BTreeMap<&[u8], BTreeMap<Vec<TokenID>, StateID>> = BTreeMap::new();

        // LLM Token: b"c"
        state_3_map.insert(b"c", BTreeMap::from([
            (vec![3], 0),    // Matched 'abc', Token 3
        ]));

        expected.insert(3, state_3_map);

        // Now assert that the actual result matches the expected
        assert_eq!(result, expected);
    }
}
