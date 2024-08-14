use crate::{Combinator, combinator, EatByteStringChoice, EatU8, eps, U8Set};
use crate::tokenizer::finite_automata::{ExprGroups, Expr, prec, DFAState, opt, ExprGroup, DFA, RegexState, Regex, QuantifierType};

pub fn seq_fast<'a>(parsers: &'a [Expr<'a>]) -> Expr<'a> {
    Expr::Seq(parsers)
}

pub fn choice_fast<'a>(parsers: &'a [Expr<'a>]) -> Expr<'a> {
    Expr::Choice(parsers)
}

pub fn opt_fast(parser: Expr) -> Expr<'static> {
    Expr::Choice(&[parser, Expr::Seq(&[])])
}

pub fn repeat1_fast(parser: Expr) -> Expr<'static> {
    Expr::Quantifier(&parser, QuantifierType::OneOrMore)
}

pub fn eat_u8_fast(byte: u8) -> Expr<'static> {
    Expr::U8Class(U8Set::from_byte(byte))
}

pub fn eat_u8_negation_fast(byte: u8) -> Expr<'static> {
    Expr::U8Class(U8Set::from_byte(byte).complement())
}

pub fn eat_u8_choice_fast(bytes: &[u8]) -> Expr<'static> {
    Expr::U8Class(U8Set::from_bytes(bytes))
}

pub fn eat_u8_negation_choice_fast(bytes: &[u8]) -> Expr<'static> {
    Expr::U8Class(U8Set::from_bytes(bytes).complement())
}

pub fn eat_u8_range_fast(start: u8, end: u8) -> Expr<'static> {
    Expr::U8Class(U8Set::from_byte_range(start..=end))
}

pub fn eat_char_fast(c: char) -> Expr<'static> {
    Expr::U8(c as u8)
}

pub fn eat_char_negation_fast(c: char) -> Expr<'static> {
    Expr::U8Class(U8Set::from_char(c).complement())
}

pub fn eat_char_choice_fast(s: &str) -> Expr<'static> {
    Expr::U8Class(U8Set::from_bytes(s.as_bytes()))
}

pub fn eat_char_negation_choice_fast(s: &str) -> Expr<'static> {
    Expr::U8Class(U8Set::from_bytes(s.as_bytes()).complement())
}

pub fn eat_string_fast(s: &str) -> Expr<'static> {
    Expr::U8Slice(s.as_bytes())
}

pub fn eat_byte_range_fast(start: u8, end: u8) -> Expr<'static> {
    Expr::U8Class(U8Set::from_byte_range(start..=end))
}

pub fn eat_bytestring_choice_fast(bytestrings: Vec<Vec<u8>>) -> Expr<'static> {
    let mut children = vec![];
    for bytes in bytestrings {
        if bytes.len() > 1 {
            // TODO: This is a hack to speed things up.
            continue;
        }
        children.push(eat_bytestring_fast(bytes));
    }
    choice_fast(&children)
}

pub fn eat_bytestring_fast(bytes: Vec<u8>) -> Expr<'static> {
    Expr::U8Slice(bytes.as_slice())
}

pub fn eat_string_choice_fast(strings: &[&str]) -> Expr<'static> {
    let mut children = vec![];
    for s in strings {
        children.push(eat_string_fast(s));
    }
    choice_fast(&children)
}

pub fn repeat0_fast(parser: Expr) -> Expr<'static> {
    opt_fast(repeat1_fast(parser))
}

pub fn seprep1_fast(a: Expr, b: Expr) -> Expr<'static> {
    seq_fast(&[a, repeat0_fast(seq_fast(&[b, a]))])
}

pub fn seprep0_fast(a: Expr, b: Expr) -> Expr<'static> {
    opt_fast(seprep1_fast(a, b))
}

pub fn repeatn_fast(n: usize, parser: Expr) -> Expr<'static> {
    if n == 0 {
        return seq_fast(&[]);
    }
    let mut parsers = Vec::new();
    for _ in 0..n {
        parsers.push(parser.clone());
    }
    seq_fast(&parsers)
}

#[macro_export]
macro_rules! seq_fast {
    ($($x:expr),* $(,)?) => {
        $crate::seq_fast(&[$($x),*])
    };
}

#[macro_export]
macro_rules! choice_fast {
    ($($x:expr),* $(,)?) => {
        $crate::choice_fast(&[$($x),*])
    };
}
