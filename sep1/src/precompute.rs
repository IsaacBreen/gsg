use crate::finite_automata::{Regex, RegexState};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// A trait for tokenizers that can handle ambiguous token matches.
pub trait Tokenizer {
    /// Executes the tokenizer on the given text and returns a map from positions to possible token sequences.
    fn execute(&mut self, text: &[u8]) -> HashMap<usize, Vec<usize>>;

    /// Returns the possible next tokens that could be matched.
    fn possible_next_tokens(&self) -> Vec<usize>;

    /// Returns true if the tokenizer has no more possible next tokens.
    fn done(&self) -> bool {
        self.possible_next_tokens().is_empty()
    }

    /// Executes the tokenizer on the entire string and returns all possible token sequences and final states.
    fn execute_all(&mut self, text: &[u8]) -> Vec<(Vec<usize>, Self)>
    where
        Self: Sized + Clone,
    {
        let mut results = Vec::new();
        let mut current_state = self.clone();
        let mut token_sequence = Vec::new();

        while !current_state.done() {
            let next_tokens = current_state.possible_next_tokens();
            if next_tokens.is_empty() {
                break;
            }

            for token in next_tokens {
                token_sequence.push(token);
                current_state.execute(&[token as u8]);
            }

            results.push((token_sequence.clone(), current_state.clone()));
        }

        results
    }
}

/// A struct that holds a set of regex-based tokenizers for each grammar token.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RegexTokenizer<'a> {
    states: Vec<RegexState<'a>>,
}

impl Clone for RegexTokenizer<'_> {
    fn clone(&self) -> Self {
        RegexTokenizer {
            states: self.states.clone(),
        }
    }
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
    fn execute(&mut self, text: &[u8]) -> HashMap<usize, Vec<usize>> {
        let mut result = HashMap::new();

        for (i, state) in self.states.iter_mut().enumerate() {
            state.execute(text);
            if let Some(matched) = state.prev_match() {
                result.entry(matched.position).or_insert_with(Vec::new).push(i);
            }
        }

        result
    }

    fn possible_next_tokens(&self) -> Vec<usize> {
        let mut possible_tokens = Vec::new();

        for (i, state) in self.states.iter().enumerate() {
            if !state.done() {
                possible_tokens.push(i);
            }
        }

        possible_tokens
    }
}

/// Precomputes the possible token sequences for each LLM token.
pub fn precompute<T: Tokenizer + Clone + Eq + Hash>(
    tokenizer: T,
    llm_tokens: Vec<Vec<u8>>,
) -> HashMap<T, Vec<(Vec<usize>, T)>> {
    let mut result = HashMap::new();

    for llm_token in llm_tokens {
        let mut tokenizer_clone = tokenizer.clone();
        let token_sequences = tokenizer_clone.execute_all(&llm_token);
        result.insert(tokenizer_clone, token_sequences);
    }

    result
}

/// Tests for the precompute module.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::finite_automata::{eat_u8, rep};
    use crate::seq;

    #[test]
    fn test_precompute() {
        // Define some simple regexes for testing
        let regexes = vec![
            eat_u8(b'a').build(), // Token 0: 'a'
            eat_u8(b'b').build(), // Token 1: 'b'
            rep(eat_u8(b'c')).build(), // Token 2: 'c*'
        ];

        let tokenizer = RegexTokenizer::from_regexes(&regexes);

        // Define some LLM tokens (in this case, just single bytes for simplicity)
        let llm_tokens = vec![vec![b'a'], vec![b'b'], vec![b'c'], vec![b'd']];

        // Precompute the possible token sequences for each LLM token
        let precomputed = precompute(tokenizer, llm_tokens);

        // Check the results
        for (tokenizer_state, sequences) in precomputed {
            for (sequence, final_state) in sequences {
                println!("Sequence: {:?}, Final State: {:?}", sequence, final_state);
            }
        }
    }

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
    }
}