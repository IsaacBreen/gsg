use crate::finite_automata::{Regex, RegexState};
use std::collections::HashMap;
use std::hash::Hash;

/// A trait for tokenizers that can handle ambiguous token matches.
pub trait Tokenizer {
    /// Creates a new instance of the tokenizer in its initial state.
    fn initial_state(&self) -> Self;

    /// Gets all possible states for the tokenizer.
    fn states(&self) -> Vec<Self>;

    /// Executes the tokenizer on the given text and returns a map from positions to possible token sequences.
    fn execute(&mut self, text: &[u8]) -> HashMap<usize, Vec<usize>>;

    /// Returns the possible next tokens that could be matched.
    fn possible_next_tokens(&self) -> Vec<usize>;

    /// Returns true if the tokenizer has no more possible next tokens.
    fn done(&self) -> bool {
        self.possible_next_tokens().is_empty()
    }

    /// Executes the tokenizer on the entire string and returns all possible token sequences and final states.
    fn execute_all(&mut self, text: &[u8]) -> Vec<(Vec<usize>, Self)> {
        // Implement using recursion. For each end position, start a new instance of the tokenizer and execute it on the remaining text.
        // Return all possible token sequences and the final state they lead to.
        todo!()
    }
}

/// A struct that holds a set of regex-based tokenizers for each grammar token.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RegexTokenizer<'a> {
    states: Vec<RegexState<'a>>,
}

impl<'a> RegexTokenizer<'a> {
    /// Creates a new `RegexTokenizer` from a set of regexes, one for each grammar token.
    pub fn from_regexes(regexes: &'a [Regex]) -> Self {
        RegexTokenizer {
            states: regexes.iter().map(|regex| regex.init()).collect(),
        }
    }
}

impl<'a> Tokenizer for RegexTokenizer<'a> {
    fn initial_state(&self) -> Self {
        todo!()
    }

    fn states(&self) -> Vec<Self> {
        todo!()
    }

    fn execute(&mut self, text: &[u8]) -> HashMap<usize, Vec<usize>> {
        todo!()
    }

    fn possible_next_tokens(&self) -> Vec<usize> {
        todo!()
    }
}

/// Precomputes the possible token sequences for each LLM token.
pub fn precompute<T: Tokenizer + Clone + Eq + Hash>(
    tokenizer: T,
    llm_tokens: Vec<Vec<u8>>,
) -> HashMap<T, Vec<(Vec<usize>, T)>> {
    todo!()
}

/// Tests for the precompute module.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::finite_automata::eat_u8;
    use crate::seq;

    #[test]
    fn test_regex_tokenizer() {
        // Define some simple regexes for testing
        let regexes = vec![
            eat_u8(b'a').build(), // Token 0: 'a'
            eat_u8(b'b').build(), // Token 1: 'b'
            seq![eat_u8(b'a'), eat_u8(b'b')].build(), // Token 2: 'ab'
        ];

        let mut tokenizer = RegexTokenizer::from_regexes(&regexes);

        // Execute the tokenizer on a string
        let result = tokenizer.execute(b"ab");

        // Check the results
        assert_eq!(result.get(&1), Some(&vec![0])); // 'a' matched at position 1
        assert_eq!(result.get(&2), Some(&vec![2])); // 'c' matched at position 3

        // Get all possible token sequences
        let results = tokenizer.execute_all(b"ab");

        // The two possible token sequences are [0, 1] or [2]. In both cases, the final state should be the initial state.
        assert!(results.contains(&(vec![0, 1], tokenizer.())));
        assert!(results.contains(&(vec![2], tokenizer.clone())));
    }

    #[test]
    fn test_regex_tokenizer_2() {
        // Try with a regex that doesn't necessarily end in the initial state
        let regexes = vec![
            eat_u8(b'a').build(), // Token 0: 'a'
            eat_u8(b'b').build(), // Token 1: 'b'
            seq![eat_u8(b'a'), eat_u8(b'b'), eat_u8(b'c')].build(), // Token 2: 'abc'
        ];

        todo!()
    }

    #[test]
    fn test_precompute() {
        todo!()
    }
}