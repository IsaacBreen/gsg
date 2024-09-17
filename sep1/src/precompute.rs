use crate::finite_automata::{Expr, Regex, RegexState};
use std::collections::{HashMap, HashSet};

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

        for (pos, tokens) in current_state.execute(text) {
            for token in tokens {
                token_sequence.push(token);
                results.push((token_sequence.clone(), current_state.clone()));
            }
        }

        results
    }
}

/// A struct that holds a set of regex-based tokenizers for each grammar token.
pub struct RegexTokenizer {
    regexes: Vec<Regex>,
    states: Vec<RegexState<'static>>,
}

impl RegexTokenizer {
    /// Creates a new `RegexTokenizer` from a set of regexes, one for each grammar token.
    pub fn new(regexes: Vec<Regex>) -> Self {
        let states = regexes.iter().map(|r| r.init()).collect();
        RegexTokenizer { regexes, states }
    }
}

impl Tokenizer for RegexTokenizer {
    fn execute(&mut self, text: &[u8]) -> HashMap<usize, Vec<usize>> {
        let mut result = HashMap::new();

        for (i, state) in self.states.iter_mut().enumerate() {
            state.execute(text);
            if let Some(m) = state.prev_match() {
                result.entry(m.position).or_insert_with(Vec::new).push(i);
            }
        }

        result
    }

    fn possible_next_tokens(&self) -> Vec<usize> {
        let mut tokens = Vec::new();
        for (i, state) in self.states.iter().enumerate() {
            if !state.done() {
                tokens.push(i);
            }
        }
        tokens
    }
}

/// A struct that precomputes valid token sets for each possible next string segment.
pub struct Precompute {
    tokenizer: RegexTokenizer,
    token_map: HashMap<usize, HashSet<usize>>, // Maps LLM tokens to grammar tokens
}

impl Precompute {
    /// Creates a new `Precompute` instance with the given tokenizer and token map.
    pub fn new(tokenizer: RegexTokenizer, token_map: HashMap<usize, HashSet<usize>>) -> Self {
        Precompute { tokenizer, token_map }
    }

    /// Precomputes the valid token sets for each possible next string segment.
    pub fn precompute(&mut self, text: &[u8]) -> HashMap<usize, HashSet<usize>> {
        let mut valid_tokens = HashMap::new();

        for (pos, token_ids) in self.tokenizer.execute(text) {
            for token_id in token_ids {
                if let Some(llm_tokens) = self.token_map.get(&token_id) {
                    valid_tokens
                        .entry(pos)
                        .or_insert_with(HashSet::new)
                        .extend(llm_tokens.iter().cloned());
                }
            }
        }

        valid_tokens
    }
}

/// Tests for the precompute module.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::finite_automata::{eat_u8, seq};

    #[test]
    fn test_precompute() {
        // Define some simple regexes for testing.
        let regexes = vec![
            eat_u8(b'a').build(), // Token 0: 'a'
            eat_u8(b'b').build(), // Token 1: 'b'
            seq![eat_u8(b'a'), eat_u8(b'b')].build(), // Token 2: 'ab'
        ];

        // Create a tokenizer with these regexes.
        let mut tokenizer = RegexTokenizer::new(regexes);

        // Define a token map that maps grammar tokens to LLM tokens.
        let mut token_map = HashMap::new();
        token_map.insert(0, HashSet::from([100, 101])); // 'a' -> LLM tokens 100, 101
        token_map.insert(1, HashSet::from([102])); // 'b' -> LLM token 102
        token_map.insert(2, HashSet::from([103])); // 'ab' -> LLM token 103

        // Create a precompute instance.
        let mut precompute = Precompute::new(tokenizer, token_map);

        // Precompute valid tokens for the string "ab".
        let valid_tokens = precompute.precompute(b"ab");

        // Check that the valid tokens are correct.
        assert_eq!(valid_tokens.get(&1), Some(&HashSet::from([100, 101])));
        assert_eq!(valid_tokens.get(&2), Some(&HashSet::from([102, 103])));
    }
}