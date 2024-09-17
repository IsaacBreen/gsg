use std::collections::{HashMap, HashSet};

/// Trait for a tokenizer that can be implemented by users for custom tokens.
pub trait Tokenizer {
    /// Creates a new instance of the tokenizer.
    fn new() -> Self where Self: Sized;

    /// Executes the tokenizer on the given text and returns a map of positions to possible token IDs.
    /// The positions are relative to the start of the string passed on this call.
    fn execute(&mut self, text: &[u8]) -> HashMap<usize, Vec<usize>>;

    /// Returns a vector of possible next token IDs that could be matched.
    fn possible_next_tokens(&self) -> Vec<usize>;

    /// Returns true if the tokenizer is done (i.e., no more tokens can be matched).
    fn done(&self) -> bool {
        self.possible_next_tokens().is_empty()
    }

    /// Executes the tokenizer repeatedly up to the end of the string, returning a vector of pairs of
    /// (token ID sequence, final state). These represent every possible sequence of tokens that could
    /// be obtained and every possible final state.
    fn execute_all(mut self, text: &[u8]) -> Vec<(Vec<usize>, Self)>
    where
        Self: Sized + Clone,
    {
        let mut results = Vec::new();
        let mut current_state = self.clone();
        let mut token_sequence = Vec::new();

        for (pos, token_ids) in current_state.execute(text) {
            for token_id in token_ids {
                token_sequence.push(token_id);
                current_state = self.clone();
                results.push((token_sequence.clone(), current_state.clone()));
            }
        }

        results
    }
}

/// Precomputes the possible next token sequences for a given string and a set of tokenizers.
/// This function takes a tokenizer, a possible next string, and a list of other tokenizers.
/// It returns a vector of vectors of token sequences that could be matched by the tokenizers.
pub fn precompute_next_tokens(
    tokenizer: &mut dyn Tokenizer,
    next_string: &[u8],
    other_tokenizers: &[&dyn Tokenizer],
) -> Vec<Vec<usize>> {
    let mut possible_sequences = Vec::new();

    // Execute the current tokenizer on the next string.
    let current_results = tokenizer.execute(next_string);

    // Collect all possible token sequences from the current tokenizer.
    for (pos, token_ids) in current_results {
        for token_id in token_ids {
            possible_sequences.push(vec![token_id]);
        }
    }

    // Now check the other tokenizers to see if they can match the next string.
    for other_tokenizer in other_tokenizers {
        let mut other_tokenizer_clone = other_tokenizer.clone_box();
        let other_results = other_tokenizer_clone.execute(next_string);

        for (pos, token_ids) in other_results {
            for token_id in token_ids {
                possible_sequences.push(vec![token_id]);
            }
        }
    }

    possible_sequences
}

/// Trait for cloning a boxed tokenizer.
pub trait CloneBox {
    fn clone_box(&self) -> Box<dyn Tokenizer>;
}

impl<T> CloneBox for T
where
    T: 'static + Tokenizer + Clone,
{
    fn clone_box(&self) -> Box<dyn Tokenizer> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Tokenizer> {
    fn clone(&self) -> Box<dyn Tokenizer> {
        self.clone_box()
    }
}

/// Example implementation of a simple tokenizer for identifiers.
#[derive(Clone)]
pub struct IdentifierTokenizer {
    state: usize,
}

impl Tokenizer for IdentifierTokenizer {
    fn new() -> Self {
        IdentifierTokenizer { state: 0 }
    }

    fn execute(&mut self, text: &[u8]) -> HashMap<usize, Vec<usize>> {
        let mut results = HashMap::new();

        for (i, &byte) in text.iter().enumerate() {
            if byte.is_ascii_alphabetic() || byte == b'_' {
                results.entry(i).or_insert_with(Vec::new).push(1); // Token ID 1 for IDENTIFIER
            } else {
                break;
            }
        }

        results
    }

    fn possible_next_tokens(&self) -> Vec<usize> {
        vec![1] // IDENTIFIER token
    }
}

/// Example implementation of a simple tokenizer for numbers.
#[derive(Clone)]
pub struct NumberTokenizer {
    state: usize,
}

impl Tokenizer for NumberTokenizer {
    fn new() -> Self {
        NumberTokenizer { state: 0 }
    }

    fn execute(&mut self, text: &[u8]) -> HashMap<usize, Vec<usize>> {
        let mut results = HashMap::new();

        for (i, &byte) in text.iter().enumerate() {
            if byte.is_ascii_digit() {
                results.entry(i).or_insert_with(Vec::new).push(2); // Token ID 2 for NUMBER
            } else {
                break;
            }
        }

        results
    }

    fn possible_next_tokens(&self) -> Vec<usize> {
        vec![2] // NUMBER token
    }
}

/// Example implementation of a simple tokenizer for commas.
#[derive(Clone)]
pub struct CommaTokenizer {
    state: usize,
}

impl Tokenizer for CommaTokenizer {
    fn new() -> Self {
        CommaTokenizer { state: 0 }
    }

    fn execute(&mut self, text: &[u8]) -> HashMap<usize, Vec<usize>> {
        let mut results = HashMap::new();

        if text.starts_with(b",") {
            results.entry(0).or_insert_with(Vec::new).push(3); // Token ID 3 for COMMA
        }

        results
    }

    fn possible_next_tokens(&self) -> Vec<usize> {
        vec![3] // COMMA token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_tokenizer() {
        let mut tokenizer = IdentifierTokenizer::new();
        let result = tokenizer.execute(b"hello");
        assert_eq!(result.get(&0), Some(&vec![1])); // IDENTIFIER token
    }

    #[test]
    fn test_number_tokenizer() {
        let mut tokenizer = NumberTokenizer::new();
        let result = tokenizer.execute(b"123");
        assert_eq!(result.get(&0), Some(&vec![2])); // NUMBER token
    }

    #[test]
    fn test_comma_tokenizer() {
        let mut tokenizer = CommaTokenizer::new();
        let result = tokenizer.execute(b",");
        assert_eq!(result.get(&0), Some(&vec![3])); // COMMA token
    }

    #[test]
    fn test_precompute_next_tokens() {
        let mut identifier_tokenizer = IdentifierTokenizer::new();
        let mut number_tokenizer = NumberTokenizer::new();
        let mut comma_tokenizer = CommaTokenizer::new();

        let other_tokenizers: Vec<&dyn Tokenizer> = vec![&number_tokenizer, &comma_tokenizer];

        let result = precompute_next_tokens(&mut identifier_tokenizer, b"hello", &other_tokenizers);
        assert_eq!(result, vec![vec![1]]); // IDENTIFIER token

        let result = precompute_next_tokens(&mut number_tokenizer, b"123", &other_tokenizers);
        assert_eq!(result, vec![vec![2]]); // NUMBER token

        let result = precompute_next_tokens(&mut comma_tokenizer, b",", &other_tokenizers);
        assert_eq!(result, vec![vec![3]]); // COMMA token
    }
}