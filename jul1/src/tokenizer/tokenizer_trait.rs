/// Represents a token with a unique identifier and the width of the matched string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub id: u8,
    pub width: usize,
}

/// Trait for tokenizing strings.
pub trait Tokenizer {
    /// Tokenizes the string from the given start position and returns a vector of tokens,
    /// the total width of all matched tokens, and the maximum read position.
    ///
    /// `input`: the input string to tokenize.
    /// `start_pos`: the position in the input string from which to start tokenizing.
    ///
    /// Returns a tuple of:
    /// - Vec<Token>: Vector of all matched tokens.
    /// - usize: Total width of all matched tokens.
    /// - usize: Maximum read position in the input string.
    fn tokenize(&mut self, input: &str) -> (Vec<Token>, usize, usize);

    /// Returns a set of token IDs that could potentially be valid for the next token.
    /// This can be used for lookahead or predictive parsing features.
    ///
    /// Returns a `Vec<u8>` of token IDs.
    fn potential_token_ids(&self, input: &str) -> Vec<u8>;
}