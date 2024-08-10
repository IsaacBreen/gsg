use crate::tokenizer::finite_automata::{Expr, Regex, RegexState};
use crate::tokenizer::tokenizer_trait::{Token, Tokenizer};

impl Tokenizer for RegexState {
    fn tokenize(&mut self, mut input: &str) -> (Vec<Token>, usize, usize) {
        fn tokenize(regex_state: &mut RegexState, input: &str) -> (Option<Token>, usize) {
            let find_result = regex_state.regex.find(input);
            if let Some(success) = find_result.inner {
                let token = Token {
                    id: success.group_id as u8, // Assuming group_id fits into u8
                    width: success.position,
                };
                (Some(token), success.position)
            } else {
                (None, input.len()) // No token matched, read to the end of the input
            }
        }
        let mut tokens = Vec::new();
        let mut read = 0;
        loop {
            let (token, read_now) = tokenize(self, input);
            if let Some(token) = token {
                tokens.push(token);
                read += token.width;
                input = &input[token.width..];
            } else {
                break;
            }
        }
        (tokens, read, input.len())
    }

    fn potential_token_ids(&self, input: &str) -> Vec<u8> {
        self.get_possible_group_ids().into_iter().map(|id| id as u8).collect()
    }
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;

    #[test]
    fn test_single_token() {
        let expr: Expr = 'a'.into();
        let mut regex = expr.build();
        let input = "abc";
        let (tokens, width, read) = regex.init().tokenize(input);
        assert_eq!(tokens.get(0), Some(Token { id: 0, width: 1 }).as_ref());
        assert_eq!(read, 1);
    }

    #[test]
    fn test_no_token() {
        let expr: Expr = 'a'.into();
        let mut regex = expr.build();
        let input = "xyz";
        let (tokens, width, read) = regex.init().tokenize(input);
        assert_eq!(tokens.get(0), None);
        assert_eq!(read, input.len());
    }
}