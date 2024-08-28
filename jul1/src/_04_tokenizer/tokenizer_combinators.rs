use crate::tokenizer::finite_automata::{Expr, QuantifierType};
use crate::U8Set;

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

pub fn eat_u8_fast(byte: u8) -> Expr {
    Expr::U8Seq(vec![byte])
}

pub fn eat_u8_negation_fast(byte: u8) -> Expr {
    Expr::U8Class(U8Set::from_byte(byte).complement())
}

pub fn eat_u8_choice_fast(bytes: &[u8]) -> Expr {
    Expr::U8Class(U8Set::from_bytes(bytes))
}

pub fn eat_u8_negation_choice_fast(bytes: &[u8]) -> Expr {
    Expr::U8Class(U8Set::from_bytes(bytes).complement())
}

pub fn eat_u8_range_fast(start: u8, end: u8) -> Expr {
    Expr::U8Class(U8Set::from_byte_range(start..=end))
}

pub fn eat_char_fast(c: char) -> Expr {
    Expr::U8Seq(vec![c as u8])
}

pub fn eat_char_negation_fast(c: char) -> Expr {
    Expr::U8Class(U8Set::from_char(c).complement())
}

pub fn eat_char_choice_fast(s: &str) -> Expr {
    Expr::U8Class(U8Set::from_chars(s))
}

pub fn eat_char_negation_choice_fast(s: &str) -> Expr {
    Expr::U8Class(U8Set::from_chars(s).complement())
}

pub fn eat_string_fast(s: &str) -> Expr {
    Expr::U8Seq(s.bytes().map(|c| c as u8).collect())
}

pub fn eat_byte_range_fast(start: u8, end: u8) -> Expr {
    Expr::U8Class(U8Set::from_byte_range(start..=end))
}

pub fn eat_bytestring_choice_fast(bytestrings: Vec<Vec<u8>>) -> Expr {
    let mut children = vec![];
    println!("eat_bytestring_choice_fast: start");
    for bytes in bytestrings {
        if bytes.len() > 1 {
            // TODO: This is a hack to speed things up.
            continue;
        }
        if bytes.len() > 4 {
            println!("very long bytestring: {:?}", bytes);
        }
        children.push(eat_bytestring_fast(bytes));
    }
    println!("eat_bytestring_choice_fast: done");
    choice_fast(children)
}

pub fn eat_bytestring_fast(bytes: Vec<u8>) -> Expr {
    Expr::U8Seq(bytes)
}

pub fn eat_string_choice_fast(strings: &[&str]) -> Expr {
    let mut children = vec![];
    for s in strings {
        children.push(eat_string_fast(s));
    }
    choice_fast(children)
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
        $crate::_04_tokenizer::seq_fast(vec![$($x),*])
    };
}

#[macro_export]
macro_rules! choice_fast {
    ($($x:expr),* $(,)?) => {
        $crate::_04_tokenizer::choice_fast(vec![$($x),*])
    };
}
