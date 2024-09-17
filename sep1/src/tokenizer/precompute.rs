// src/tokenizer/precompute.rs
use std::collections::HashMap;

use crate::tokenizer::finite_automata::{Expr, ExprGroups, Match, Regex, RegexState};
use crate::tokenizer::u8set::U8Set;

pub trait TokenTrait {
    fn id(&self) -> usize;
    fn regex(&self) -> &Regex;

    fn matches(&self, text: &[u8]) -> Option<Match> {
        self.regex().find(text)
    }

    fn fully_matches(&self, text: &[u8]) -> Option<bool> {
        self.regex().fully_matches(text)
    }
}

pub fn precompute_possible_next_tokens(
    current_token: &impl TokenTrait,
    next_string: &[u8],
    other_tokens: &[&dyn TokenTrait],
) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();

    // 1. Execute the current token's regex on the next string to see what matches we get.
    let mut current_regex_state = current_token.regex().init();
    current_regex_state.execute(next_string);

    // If the current token's regex doesn't even partially match the next string, then no other tokens can match.
    if current_regex_state.failed() {
        return result;
    }

    // 2. If the current token fully matches, then we can try all other tokens.
    if current_regex_state.fully_matches_here() {
        result.push(Vec::new()); // Empty sequence is valid if the current token fully matches
        for token in other_tokens {
            if let Some(m) = token.matches(next_string) {
                result.push(vec![token.id()]);
            }
        }
    } else if current_regex_state.could_fully_match() {
        // The current token could fully match the next string but hasn't yet.
        // We don't need to try any other tokens.
        result.push(Vec::new());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestToken {
        id: usize,
        regex: Regex,
    }

    impl TokenTrait for TestToken {
        fn id(&self) -> usize {
            self.id
        }

        fn regex(&self) -> &Regex {
            &self.regex
        }
    }

    #[test]
    fn test_identifier_followed_by_comma_identifier() {
        let identifier_regex = Expr::build(Expr::Seq(vec![
            Expr::U8Class(U8Set::from_chars("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_")),
            Expr::Quantifier(
                Box::new(Expr::U8Class(U8Set::from_chars(
                    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789",
                ))),
                crate::tokenizer::finite_automata::QuantifierType::ZeroOrMore,
            ),
        ]));

        let comma_regex = Expr::build(Expr::U8Seq(vec![b',']));

        let identifier_token = TestToken { id: 0, regex: identifier_regex };
        let comma_token = TestToken { id: 1, regex: comma_regex };

        let other_tokens = vec![&identifier_token, &comma_token];

        let possible_tokens = precompute_possible_next_tokens(
            &identifier_token,
            b"ello, world",
            &other_tokens,
        );

        assert_eq!(possible_tokens, vec![vec![1, 0]]);
    }

    #[test]
    fn test_identifier_followed_by_comma_match() {
        let identifier_regex = Expr::build(Expr::Seq(vec![
            Expr::U8Class(U8Set::from_chars("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_")),
            Expr::Quantifier(
                Box::new(Expr::U8Class(U8Set::from_chars(
                    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789",
                ))),
                crate::tokenizer::finite_automata::QuantifierType::ZeroOrMore,
            ),
        ]));

        let comma_regex = Expr::build(Expr::U8Seq(vec![b',']));
        let match_regex = Expr::build(Expr::U8Seq(b"match"));

        let identifier_token = TestToken { id: 0, regex: identifier_regex };
        let comma_token = TestToken { id: 1, regex: comma_regex };
        let match_token = TestToken { id: 2, regex: match_regex };

        let other_tokens = vec![&identifier_token, &comma_token, &match_token];

        let possible_tokens = precompute_possible_next_tokens(
            &identifier_token,
            b"ello, match",
            &other_tokens,
        );

        // This should return two possible sequences: [COMMA, IDENTIFIER] and [COMMA, MATCH_KEYWORD]
        assert_eq!(possible_tokens.len(), 2);

        // Due to hashmaps being unordered, we can't guarantee the order of the sequences.
        // However, we can check that the two possible sequences are present.
        assert!(possible_tokens.contains(&vec![1, 0]));
        assert!(possible_tokens.contains(&vec![1, 2]));
    }

    #[test]
    fn test_number_followed_by_identifier() {
        let number_regex = Expr::build(Expr::Seq(vec![
            Expr::U8Class(U8Set::from_chars("0123456789")),
            Expr::Quantifier(
                Box::new(Expr::U8Class(U8Set::from_chars("0123456789"))),
                crate::tokenizer::finite_automata::QuantifierType::ZeroOrMore,
            ),
        ]));

        let identifier_regex = Expr::build(Expr::Seq(vec![
            Expr::U8Class(U8Set::from_chars("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_")),
            Expr::Quantifier(
                Box::new(Expr::U8Class(U8Set::from_chars(
                    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789",
                ))),
                crate::tokenizer::finite_automata::QuantifierType::ZeroOrMore,
            ),
        ]));

        let number_token = TestToken { id: 0, regex: number_regex };
        let identifier_token = TestToken { id: 1, regex: identifier_regex };

        let other_tokens = vec![&number_token, &identifier_token];

        let possible_tokens = precompute_possible_next_tokens(
            &number_token,
            b"ello, world",
            &other_tokens,
        );

        assert_eq!(possible_tokens, Vec::new());
    }

    #[test]
    fn test_number_followed_by_nothing() {
        let number_regex = Expr::build(Expr::Seq(vec![
            Expr::U8Class(U8Set::from_chars("0123456789")),
            Expr::Quantifier(
                Box::new(Expr::U8Class(U8Set::from_chars("0123456789"))),
                crate::tokenizer::finite_automata::QuantifierType::ZeroOrMore,
            ),
        ]));

        let number_token = TestToken { id: 0, regex: number_regex };

        let other_tokens = vec![&number_token];

        let possible_tokens =
            precompute_possible_next_tokens(&number_token, b"ello", &other_tokens);

        assert_eq!(possible_tokens, vec![vec![]]);
    }
}