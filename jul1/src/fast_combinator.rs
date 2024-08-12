use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::{Combinator, combinator, EatByteStringChoice, EatU8, eps, U8Set};
use crate::trie::{FinishReason, TrieNode};
use std::collections::HashMap;
use crate::tokenizer::charset::CharSet;
use crate::tokenizer::finite_automata::{ExprGroups, Expr, prec, DFAState, opt, ExprGroup, DFA, RegexState, Regex, QuantifierType};

pub fn seq_fast(parsers: Vec<Expr>) -> Expr {
    Expr::Seq(parsers)
}

pub fn choice_fast(parsers: Vec<Expr>) -> Expr {
    Expr::Choice(parsers)
}

pub fn opt_fast(parser: Expr) -> Expr {
    Expr::Choice(vec![parser, Expr::Seq(vec![])])
}

pub fn repeat1_fast(parser: Expr) -> Expr {
    Expr::Quantifier(Box::new(parser), QuantifierType::OneOrMore)
}

impl From<U8Set> for CharSet {
    fn from(u8set: U8Set) -> Self {
        let mut charset = CharSet::new();
        for byte in u8set.iter() {
            charset.insert(byte as char);
        }
        charset
    }
}

pub fn eat_char_fast(c: char) -> Expr {
    Expr::CharClass(U8Set::from_char(c).into())
}

pub fn eat_byte_fast(byte: u8) -> Expr {
    Expr::CharClass(U8Set::from_byte(byte).into())
}

pub fn eat_char_negation_fast(c: char) -> Expr {
    Expr::CharClass(U8Set::from_char(c).complement().into())
}

pub fn eat_char_choice_fast(chars: &str) -> Expr {
    Expr::CharClass(U8Set::from_chars(chars).into())
}

pub fn eat_char_negation_choice_fast(chars: &str) -> Expr {
    Expr::CharClass(U8Set::from_chars(chars).complement().into())
}

pub fn eat_byte_range_fast(start: u8, end: u8) -> Expr {
    Expr::CharClass(U8Set::from_byte_range(start..=end).into())
}

pub fn eat_bytestring_choice_fast(bytestrings: Vec<Vec<u8>>) -> Expr {
    let mut children = vec![];
    for bytes in bytestrings {
        children.push(eat_bytestring_fast(bytes));
    }
    choice_fast(children)
}

pub fn eat_string_choice_fast(strings: &[&str]) -> Expr {
    eat_bytestring_choice_fast(strings.into_iter().map(|s| s.as_bytes().to_vec()).collect())
}

pub fn eat_string_fast(s: &str) -> Expr {
    let mut children = vec![];
    for c in s.bytes() {
        children.push(eat_byte_fast(c));
    }
    seq_fast(children)
}

pub fn eat_bytestring_fast(bytes: Vec<u8>) -> Expr {
    let mut children = vec![];
    for byte in bytes {
        children.push(eat_byte_fast(byte));
    }
    seq_fast(children)
}

pub fn repeat0_fast(parser: Expr) -> Expr {
    opt_fast(repeat1_fast(parser))
}

pub fn seprep1_fast(a: Expr, b: Expr) -> Expr {
    seq_fast(vec![a.clone(), repeat0_fast(seq_fast(vec![b, a]))])
}

pub fn seprep0_fast(a: Expr, b: Expr) -> Expr {
    opt_fast(seprep1_fast(a, b))
}

pub fn repeatn_fast(n: usize, parser: Expr) -> Expr {
    if n == 0 {
        return seq_fast(vec![]);
    }
    let mut parsers = Vec::new();
    for _ in 0..n {
        parsers.push(parser.clone());
    }
    seq_fast(parsers)
}

#[macro_export]
macro_rules! seq_fast {
    ($($x:expr),* $(,)?) => {
        $crate::seq_fast(vec![$($x),*])
    };
}

#[macro_export]
macro_rules! choice_fast {
    ($($x:expr),* $(,)?) => {
        $crate::choice_fast(vec![$($x),*])
    };
}
