use std::collections::{HashMap, HashSet};

/// Trait for a tokenizer that can process a string and return possible token sequences.
pub trait Tokenizer {
    /// Creates a new tokenizer.
    fn new() -> Self;

    /// Executes the tokenizer on the given text and returns a map from position to possible token IDs.
    /// The key is the position in the text, and the value is a vector of possible token IDs at that position.
    fn execute(&mut self, text: &[u8]) -> HashMap<usize, Vec<usize>>;

    /// Returns the possible next tokens that could be matched by the tokenizer.
    fn possible_next_tokens(&self) -> Vec<usize>;

    /// Returns true if the tokenizer has no more possible next tokens.
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

        for (pos, tokens) in current_state.execute(text) {
            for token in tokens {
                token_sequence.push(token);
                current_state = self.clone();
                results.push((token_sequence.clone(), current_state));
            }
        }

        results
    }
}

/// A struct representing the state of the LR parser.
#[derive(Debug, Clone)]
pub struct LRParserState {
    pub state_stack: Vec<usize>, // Stack of states in the LR parser
}

/// A struct representing the precomputed information for each state in the LR parser.
#[derive(Debug, Clone)]
pub struct PrecomputedState {
    pub valid_llm_tokens: HashSet<usize>, // Set of valid LLM tokens for this state
}

/// A struct representing the precomputation system.
pub struct Precompute {
    pub parser_states: HashMap<usize, PrecomputedState>, // Map from LR parser state to precomputed state
}

impl Precompute {
    /// Creates a new precompute system.
    pub fn new() -> Self {
        Precompute {
            parser_states: HashMap::new(),
        }
    }

    /// Precomputes the valid LLM tokens for each state in the LR parser.
    pub fn precompute(&mut self, parser: &LRParserState, tokenizers: Vec<Box<dyn Tokenizer>>) {
        for (state_id, state) in parser.state_stack.iter().enumerate() {
            let mut valid_llm_tokens = HashSet::new();

            // For each tokenizer, execute it and collect the possible next tokens.
            for tokenizer in &tokenizers {
                let possible_tokens = tokenizer.possible_next_tokens();
                for token in possible_tokens {
                    valid_llm_tokens.insert(token);
                }
            }

            // Store the precomputed state.
            self.parser_states.insert(
                *state,
                PrecomputedState {
                    valid_llm_tokens,
                },
            );
        }
    }

    /// Returns the valid LLM tokens for the given parser state.
    pub fn get_valid_llm_tokens(&self, state: usize) -> Option<&HashSet<usize>> {
        self.parser_states.get(&state).map(|s| &s.valid_llm_tokens)
    }
}

/// A struct representing the LLM token mask.
pub struct LLMTokenMask {
    pub mask: HashMap<usize, bool>, // Map from LLM token ID to a flag indicating whether the token is valid
}

impl LLMTokenMask {
    /// Creates a new LLM token mask with all tokens initially set to false.
    pub fn new(num_tokens: usize) -> Self {
        let mut mask = HashMap::new();
        for i in 0..num_tokens {
            mask.insert(i, false);
        }
        LLMTokenMask { mask }
    }

    /// Sets the flag for the given LLM token to true.
    pub fn set_valid(&mut self, token_id: usize) {
        if let Some(flag) = self.mask.get_mut(&token_id) {
            *flag = true;
        }
    }

    /// Returns the mask as a vector of booleans.
    pub fn to_vec(&self) -> Vec<bool> {
        let mut result = vec![false; self.mask.len()];
        for (token_id, &flag) in &self.mask {
            result[*token_id] = flag;
        }
        result
    }
}

/// A function that computes the LLM token mask for a given parser state and tokenizers.
pub fn compute_llm_token_mask(
    parser_state: &LRParserState,
    tokenizers: Vec<Box<dyn Tokenizer>>,
    num_llm_tokens: usize,
) -> LLMTokenMask {
    let mut mask = LLMTokenMask::new(num_llm_tokens);

    // For each tokenizer, execute it and collect the possible next tokens.
    for tokenizer in tokenizers {
        let possible_tokens = tokenizer.possible_next_tokens();
        for token in possible_tokens {
            mask.set_valid(token);
        }
    }

    mask
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockTokenizer {
        possible_tokens: Vec<usize>,
    }

    impl Tokenizer for MockTokenizer {
        fn new() -> Self {
            MockTokenizer {
                possible_tokens: vec![],
            }
        }

        fn execute(&mut self, _text: &[u8]) -> HashMap<usize, Vec<usize>> {
            let mut result = HashMap::new();
            result.insert(0, self.possible_tokens.clone());
            result
        }

        fn possible_next_tokens(&self) -> Vec<usize> {
            self.possible_tokens.clone()
        }
    }

    #[test]
    fn test_precompute() {
        let mut precompute = Precompute::new();
        let parser_state = LRParserState {
            state_stack: vec![0, 1, 2],
        };

        let tokenizer1 = Box::new(MockTokenizer {
            possible_tokens: vec![1, 2, 3],
        });
        let tokenizer2 = Box::new(MockTokenizer {
            possible_tokens: vec![4, 5, 6],
        });

        precompute.precompute(&parser_state, vec![tokenizer1, tokenizer2]);

        let valid_tokens = precompute.get_valid_llm_tokens(1).unwrap();
        assert!(valid_tokens.contains(&1));
        assert!(valid_tokens.contains(&2));
        assert!(valid_tokens.contains(&3));
        assert!(valid_tokens.contains(&4));
        assert!(valid_tokens.contains(&5));
        assert!(valid_tokens.contains(&6));
    }

    #[test]
    fn test_llm_token_mask() {
        let parser_state = LRParserState {
            state_stack: vec![0, 1, 2],
        };

        let tokenizer1 = Box::new(MockTokenizer {
            possible_tokens: vec![1, 2, 3],
        });
        let tokenizer2 = Box::new(MockTokenizer {
            possible_tokens: vec![4, 5, 6],
        });

        let mask = compute_llm_token_mask(&parser_state, vec![tokenizer1, tokenizer2], 10);

        let mask_vec = mask.to_vec();
        assert_eq!(mask_vec[1], true);
        assert_eq!(mask_vec[2], true);
        assert_eq!(mask_vec[3], true);
        assert_eq!(mask_vec[4], true);
        assert_eq!(mask_vec[5], true);
        assert_eq!(mask_vec[6], true);
        assert_eq!(mask_vec[0], false);
        assert_eq!(mask_vec[7], false);
        assert_eq!(mask_vec[8], false);
        assert_eq!(mask_vec[9], false);
    }
}