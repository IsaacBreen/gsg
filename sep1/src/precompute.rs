use crate::finite_automata::{Expr, Regex, RegexState};
use std::collections::{HashMap, HashSet};

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
pub struct RegexTokenizer<'a> {
    states: Vec<RegexState<'a>>,
}

impl<'a> RegexTokenizer<'a> {
    /// Creates a new `RegexTokenizer` from a set of regexes, one for each grammar token.
    pub fn from_regexes(regexes: &'a [Regex]) -> Self {
        RegexTokenizer { states: regexes.iter().map(|regex| regex.init()).collect() }
    }
}

impl Tokenizer for RegexTokenizer<'_> {
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

fn precompute<T: Tokenizer>(tokenizer: T, llm_tokens: Vec<Vec<u8>>) -> HashMap<T, Vec<(Vec<usize>, T)>> {
    todo!()
}

/// Tests for the precompute module.
#[cfg(test)]
mod tests {
    // todo
}