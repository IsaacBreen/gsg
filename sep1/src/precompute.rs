// src/precompute.rs
use crate::finite_automata::{Expr, Regex, RegexState};
use std::collections::{HashMap, HashSet};
use crate::u8set::U8Set;

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
    fn execute_all(&mut self, text: &[u8]) -> Vec<(Vec<usize>, Self)>
    where
        Self: Clone,
    {
        // Implement using recursion. For each end position, start a new instance of the tokenizer and execute it on the remaining text.
        // Return all possible token sequences and the final state they lead to.
        if text.is_empty() {
            return vec![(vec![], self.clone())];
        }

        let mut results = vec![];
        let mut tokenizer_copy = self.clone();
        let matches = tokenizer_copy.execute(text);

        for (pos, token_ids) in matches {
            let remaining_text = &text[pos..];
            for (mut next_token_ids, next_state) in next_state.execute_all(remaining_text) {
                next_token_ids.extend(token_ids.iter().cloned());
                results.push((next_token_ids, next_state));
            }
        }

        if results.is_empty() {
            // No matches found at any position, so the current state is the final state.
            results.push((vec![], self.clone()));
        }

        results
    }
}

/// A struct that holds a set of regex-based tokenizers for each grammar token.
pub struct RegexTokenizer<'a> {
    regexes: &'a [Regex],
    states: Vec<RegexState<'a>>,
}

impl<'a> RegexTokenizer<'a> {
    /// Creates a new `RegexTokenizer` from a set of regexes, one for each grammar token.
    pub fn from_regexes(regexes: &'a [Regex]) -> Self {
        RegexTokenizer {
            regexes,
            states: regexes.iter().map(|regex| regex.init()).collect(),
        }
    }
}

impl<'a> Tokenizer for RegexTokenizer<'a> {
    fn initial_state(&self) -> Self {
        RegexTokenizer {
            regexes: self.regexes,
            states: self.regexes.iter().map(|regex| regex.init()).collect(),
        }
    }

    fn states(&self) -> Vec<Self> {
        // Generate all possible combinations of states for each regex.
        let mut state_combinations = vec![vec![]];
        for regex in self.regexes {
            let mut new_state_combinations = vec![];
            for state_combination in state_combinations {
                for state in regex.states() {
                    let mut new_combination = state_combination.clone();
                    new_combination.push(state);
                    new_state_combinations.push(new_combination);
                }
            }
            state_combinations = new_state_combinations;
        }

        state_combinations
            .into_iter()
            .map(|states| RegexTokenizer {
                regexes: self.regexes,
                states,
            })
            .collect()
    }

    fn execute(&mut self, text: &[u8]) -> HashMap<usize, Vec<usize>> {
        let mut matches = HashMap::new();
        for (i, state) in self.states.iter_mut().enumerate() {
            state.execute(text);
            if let Some(m) = state.prev_match() {
                matches.entry(m.position).or_insert_with(Vec::new).push(i);
            }
        }
        matches
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

impl<'a> Regex {
    fn states(&self) -> Vec<RegexState<'a>> {
        // Generate all reachable states from the initial state.
        let mut states = HashSet::new();
        let mut queue = vec![self.init()];
        while let Some(state) = queue.pop() {
            if states.insert(state.clone()) {
                let u8set = state.get_u8set();
                for u8 in u8set.iter() {
                    let mut next_state = state.clone();
                    next_state.execute(&[u8]);
                    queue.push(next_state);
                }
            }
        }
        states.into_iter().collect()
    }
}

fn precompute<T: Tokenizer>(tokenizer: T, llm_tokens: Vec<Vec<u8>>) -> HashMap<T, Vec<(Vec<usize>, T)>>
where
    T: Clone + Eq + std::hash::Hash,
{
    let mut result = HashMap::new();
    for llm_token in llm_tokens {
        for mut state in tokenizer.states() {
            let initial_state = state.clone();
            let token_sequences = state.execute_all(&llm_token);
            result.entry(initial_state).or_insert_with(Vec::new).extend(token_sequences);
        }
    }
    result
}

/// Tests for the precompute module.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::finite_automata::{eat_u8, rep, seq, choice, Expr};
    use crate::seq;

    #[test]
    fn test_regex_tokenizer_execute_all() {
        let regexes = vec![
            eat_u8(b'a').build(),
            seq![eat_u8(b'b'), eat_u8(b'c')].build(),
        ];
        let mut tokenizer = RegexTokenizer::from_regexes(&regexes);

        let results = tokenizer.execute_all(b"abc");
        assert_eq!(results.len(), 2);

        let expected_results = vec![
            (vec![0, 1], RegexTokenizer::from_regexes(&regexes)),
            (vec![0], RegexTokenizer::from_regexes(&regexes)),
        ];

        assert!(results.contains(&(expected_results[0].0.clone(), expected_results[0].1.clone())));
        assert!(results.contains(&(expected_results[1].0.clone(), expected_results[1].1.clone())));
    }

    #[test]
    fn test_regex_tokenizer_execute_all_no_match() {
        let regexes = vec![
            eat_u8(b'a').build(),
            seq![eat_u8(b'b'), eat_u8(b'c')].build(),
        ];
        let mut tokenizer = RegexTokenizer::from_regexes(&regexes);

        let results = tokenizer.execute_all(b"d");
        assert_eq!(results.len(), 1);

        let expected_results = vec![
            (vec![], RegexTokenizer::from_regexes(&regexes)),
        ];

        assert!(results.contains(&(expected_results[0].0.clone(), expected_results[0].1.clone())));
    }

    #[test]
    fn test_regex_tokenizer_execute_all_ambiguous() {
        let regexes = vec![
            eat_u8(b'a').build(),
            rep(eat_u8(b'a')).build(),
        ];
        let mut tokenizer = RegexTokenizer::from_regexes(&regexes);

        let results = tokenizer.execute_all(b"aaa");
        assert_eq!(results.len(), 4);

        let expected_results = vec![
            (vec![0, 0, 0], RegexTokenizer::from_regexes(&regexes)),
            (vec![0, 1], RegexTokenizer::from_regexes(&regexes)),
            (vec![1, 0], RegexTokenizer::from_regexes(&regexes)),
            (vec![1], RegexTokenizer::from_regexes(&regexes)),
        ];

        for expected in expected_results {
            assert!(results.contains(&(expected.0.clone(), expected.1.clone())));
        }
    }

    #[test]
    fn test_regex_tokenizer_possible_next_tokens() {
        let regexes = vec![
            eat_u8(b'a').build(),
            seq![eat_u8(b'b'), eat_u8(b'c')].build(),
        ];
        let mut tokenizer = RegexTokenizer::from_regexes(&regexes);

        assert_eq!(tokenizer.possible_next_tokens(), vec![0, 1]);

        tokenizer.execute(b"a");
        assert_eq!(tokenizer.possible_next_tokens(), vec![0, 1]);

        tokenizer.execute(b"b");
        assert_eq!(tokenizer.possible_next_tokens(), vec![0]);

        tokenizer.execute(b"c");
        assert_eq!(tokenizer.possible_next_tokens(), vec![0, 1]);
    }

    #[test]
    fn test_regex_tokenizer_done() {
        let regexes = vec![
            eat_u8(b'a').build(),
            seq![eat_u8(b'b'), eat_u8(b'c')].build(),
        ];
        let mut tokenizer = RegexTokenizer::from_regexes(&regexes);

        assert!(!tokenizer.done());

        tokenizer.execute(b"abc");
        assert!(!tokenizer.done());

        let regexes = vec![
            eat_u8(b'a').build(),
            eat_u8(b'b').build(),
        ];
        let mut tokenizer = RegexTokenizer::from_regexes(&regexes);

        tokenizer.execute(b"a");
        assert!(!tokenizer.done());

        tokenizer.execute(b"b");
        assert!(!tokenizer.done());

        tokenizer.execute(b"c");
        assert!(tokenizer.done());
    }

    #[test]
    fn test_precompute() {
        let regexes = vec![
            eat_u8(b'a').build(),
            seq![eat_u8(b'b'), eat_u8(b'c')].build(),
        ];
        let tokenizer = RegexTokenizer::from_regexes(&regexes);
        let llm_tokens = vec![b"a".to_vec(), b"bc".to_vec(), b"d".to_vec()];

        let precomputed = precompute(tokenizer.clone(), llm_tokens);

        assert_eq!(precomputed.len(), 4); // 4 possible states

        let initial_state = tokenizer.initial_state();
        let initial_state_results = precomputed.get(&initial_state).unwrap();
        assert_eq!(initial_state_results.len(), 3);

        let expected_results = vec![
            (vec![0], tokenizer.clone()),
            (vec![1], tokenizer.clone()),
            (vec![], tokenizer.clone()),
        ];

        for expected in expected_results {
            assert!(initial_state_results.contains(&(expected.0.clone(), expected.1.clone())));
        }
    }
}