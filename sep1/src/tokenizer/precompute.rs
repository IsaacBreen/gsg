// src/tokenizer/token_trait.rs
use crate::tokenizer::finite_automata::{Expr, FinalStateReport, Regex, RegexState};

pub trait TokenTrait {
    fn id(&self) -> usize;
    fn expr(&self) -> Expr;
    fn regex(&self) -> &Regex;
    fn regex_state(&mut self) -> &mut RegexState;
    fn reset_regex_state(&mut self);

    // Checks whether the token is considered "finished" or complete
    fn is_done(&self) -> bool;

    // Finds all possible matches within the given text slice.
    fn find_matches(&mut self, text: &[u8]) -> Vec<FinalStateReport>;
}

pub fn possible_next_tokens(
    token: &mut impl TokenTrait,
    next_string: &[u8],
    other_tokens: &[&mut dyn TokenTrait],
) -> Vec<Vec<usize>> {
    let mut possible_token_sequences: Vec<Vec<usize>> = Vec::new();
    let mut current_token_sequence: Vec<usize> = Vec::new();

    // Start with the current token's state
    let mut active_tokenizers: Vec<&mut dyn TokenTrait> = vec![token];

    let mut offset = 0;
    while offset < next_string.len() {
        let mut new_tokenizers = Vec::new();

        for tokenizer in &mut active_tokenizers {
            let matches = tokenizer.find_matches(&next_string[offset..]);

            if matches.is_empty() {
                // If no matches were found for this tokenizer, it's failed for this path.
                continue;
            }

            // Each match represents a potential branch in the tokenization process.
            for m in matches {
                let mut tokenizer_clone = tokenizer;
                tokenizer_clone.regex_state().position = m.position;

                if tokenizer_clone.is_done() {
                    // If the tokenizer is done, add its ID to the current token sequence
                    current_token_sequence.push(tokenizer_clone.id());

                    // Consider starting new tokenizers from the current position
                    for other_token in other_tokens.iter_mut() {
                        other_token.reset_regex_state();
                        new_tokenizers.push(*other_token);
                    }
                } else {
                    // If the tokenizer is not done, it remains active
                    new_tokenizers.push(tokenizer_clone);
                }
            }
        }

        if new_tokenizers.is_empty() && !current_token_sequence.is_empty() {
            // If no new tokenizers were activated, but we have a valid token sequence, add it to the results
            possible_token_sequences.push(current_token_sequence);
            current_token_sequence = Vec::new();
        }

        // Update the active tokenizers for the next iteration
        active_tokenizers = new_tokenizers;

        // Move to the next character in the input string
        offset += 1;
    }

    // If we reached the end of the string with a valid token sequence, add it to the results
    if !current_token_sequence.is_empty() {
        possible_token_sequences.push(current_token_sequence);
    }

    possible_token_sequences
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::finite_automata::{Expr, groups, prec, rep, seq, ExprGroups};
    use crate::tokenizer::token_trait::{possible_next_tokens, TokenTrait};
    use crate::tokenizer::Regex;
    use crate::tokenizer::finite_automata::RegexState;

    struct TestToken {
        id: usize,
        regex: Regex,
        regex_state: RegexState<'static>,
    }

    impl TestToken {
        fn new(id: usize, expr: Expr) -> Self {
            let regex = expr.build();
            let regex_state = regex.init();
            TestToken { id, regex, regex_state }
        }
    }

    impl TokenTrait for TestToken {
        fn id(&self) -> usize {
            self.id
        }

        fn expr(&self) -> Expr {
            todo!()
        }

        fn regex(&self) -> &Regex {
            &self.regex
        }

        fn regex_state(&mut self) -> &mut RegexState {
            &mut self.regex_state
        }

        fn reset_regex_state(&mut self) {
            self.regex_state = self.regex.init();
        }

        fn is_done(&self) -> bool {
            self.regex_state.done()
        }

        fn find_matches(&mut self, text: &[u8]) -> Vec<FinalStateReport> {
            let mut matches = Vec::new();
            let mut local_position = 0;
            let dfa = &self.regex_state.regex.dfa;

            while local_position < text.len() {
                let state_data = &dfa.states[self.regex_state.current_state];
                let next_u8 = text[local_position];

                if let Some(&next_state) = state_data.transitions.get(next_u8) {
                    self.regex_state.current_state = next_state;
                    local_position += 1;

                    if let Some(finalizer) = dfa.states[self.regex_state.current_state].finalizer {
                        matches.push(FinalStateReport {
                            position: self.regex_state.position + local_position,
                            inner: None,
                        });
                    }
                } else {
                    break;
                }
            }

            self.regex_state.position += text.len();

            matches
        }
    }

    #[test]
    fn test_possible_next_tokens_simple() {
        let mut identifier_token = TestToken::new(0, rep(choice_fast![eat_u8_fast(b'a'), eat_u8_fast(b'b')]));
        let mut comma_token = TestToken::new(1, eat_u8_fast(b','));

        let other_tokens: Vec<&mut dyn TokenTrait> = vec![&mut comma_token];

        let next_string = b"ello, world";
        let possible_tokens = possible_next_tokens(&mut identifier_token, next_string, &other_tokens);

        assert_eq!(possible_tokens, vec![vec![1, 0]]);
    }

    #[test]
    fn test_possible_next_tokens_no_match() {
        let mut number_token = TestToken::new(0, rep(choice_fast![eat_u8_fast(b'0'), eat_u8_fast(b'1'), eat_u8_fast(b'2'), eat_u8_fast(b'3'), eat_u8_fast(b'4'), eat_u8_fast(b'5'), eat_u8_fast(b'6'), eat_u8_fast(b'7'), eat_u8_fast(b'8'), eat_u8_fast(b'9')]));
        let mut identifier_token = TestToken::new(1, rep(choice_fast![eat_u8_fast(b'a'), eat_u8_fast(b'b')]));

        let other_tokens: Vec<&mut dyn TokenTrait> = vec![&mut identifier_token];

        let next_string = b"ello, world";
        let possible_tokens = possible_next_tokens(&mut number_token, next_string, &other_tokens);

        assert_eq!(possible_tokens, Vec::<Vec<usize>>::new());
    }

    #[test]
    fn test_possible_next_tokens_no_new_tokens() {
        let mut identifier_token = TestToken::new(0, rep(choice_fast![eat_u8_fast(b'a'), eat_u8_fast(b'b')]));
        let mut comma_token = TestToken::new(1, eat_u8_fast(b','));

        let other_tokens: Vec<&mut dyn TokenTrait> = vec![&mut comma_token];

        let next_string = b"ello";
        let possible_tokens = possible_next_tokens(&mut identifier_token, next_string, &other_tokens);

        assert_eq!(possible_tokens, vec![vec![]]);
    }

    #[test]
    fn test_possible_next_tokens_multiple_matches() {
        let mut identifier_token = TestToken::new(0, rep(choice_fast![eat_u8_fast(b'a'), eat_u8_fast(b'b')]));
        let mut comma_token = TestToken::new(1, eat_u8_fast(b','));
        let mut match_keyword_token = TestToken::new(2, eat_string_fast("match"));

        let other_tokens: Vec<&mut dyn TokenTrait> = vec![&mut comma_token, &mut match_keyword_token];

        let next_string = b"ello, mat";
        let possible_tokens = possible_next_tokens(&mut identifier_token, next_string, &other_tokens);

        // Sort the possible token sequences for easier comparison
        let mut sorted_possible_tokens = possible_tokens.clone();
        sorted_possible_tokens.sort();

        let mut expected_tokens = vec![vec![1, 0], vec![1, 2]];
        expected_tokens.sort();

        assert_eq!(sorted_possible_tokens, expected_tokens);
    }
}