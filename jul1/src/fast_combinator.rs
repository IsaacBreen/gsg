use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::{Combinator, combinator, EatByteStringChoice, EatU8, eps, U8Set};
use crate::trie::{FinishReason, TrieNode};
use std::collections::HashMap;
use crate::FastCombinatorResult::Failure;
use crate::tokenizer::finite_automata::{ExprGroups, Expr, prec, DFAState, opt, ExprGroup, DFA, RegexState, Regex};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FastCombinator {
    Seq(Rc<Vec<FastCombinator>>),
    Choice(Rc<Vec<FastCombinator>>),
    Opt(Box<FastCombinator>),
    Repeat1(Rc<FastCombinator>),
    Eps,
    EatU8(U8Set),
    EatByteStringChoiceFast(Rc<TrieNode>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FastParser {
    // SeqParser(Rc<Vec<FastCombinator>>, usize),
    // ChoiceParser(Rc<Vec<FastCombinator>>, usize),
    // Repeat1Parser(Rc<FastCombinator>),
    // EatByteStringChoiceFastParser(Rc<TrieNode>, Rc<TrieNode>),
    // EatU8Parser(U8Set),
    DFA(DFAState),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FastCombinatorResult {
    Success(usize),
    Failure,
    Incomplete(FastParser, Vec<usize>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FastParserResult {
    pub(crate) finish_positions: Vec<usize>,
    pub(crate) done: bool,
}

impl FastCombinator {
    pub fn slow(&self) -> Combinator {
        match self {
            FastCombinator::Seq(children) => {
                let mut all_children: crate::VecX<Combinator> = crate::vecx![];
                for child in children.iter() {
                    let child_slow = child.slow();
                    match child_slow {
                        Combinator::Seq(crate::Seq { children, .. }) => {
                            all_children.extend(children.iter().cloned());
                        }
                        _ => all_children.push(child_slow),
                    }
                }
                crate::_seq(all_children)
            }
            FastCombinator::Choice(children) => {
                let mut all_children: crate::VecX<Combinator> = crate::vecx![];
                for child in children.iter() {
                    let child_slow = child.slow();
                    match child_slow {
                        Combinator::Choice(crate::Choice { children, .. }) => {
                            all_children.extend(children.iter().cloned());
                        }
                        _ => all_children.push(child_slow),
                    }
                }
                crate::_choice(all_children)
            }
            FastCombinator::Opt(parser) => crate::opt(parser.slow()).into(),
            FastCombinator::Repeat1(parser) => crate::repeat1(parser.slow()).into(),
            FastCombinator::Eps => crate::eps().into(),
            FastCombinator::EatU8(u8set) => crate::EatU8 { u8set: *u8set }.into(),
            FastCombinator::EatByteStringChoiceFast(root) => {
                crate::EatByteStringChoice { root: Rc::clone(root) }.into()
            }
        }
    }

    fn to_dfa_expr(&self) -> Expr {
        match self {
            FastCombinator::Seq(children) => {
                let mut exprs = Vec::new();
                for child in children.iter() {
                    exprs.push(child.to_dfa_expr());
                }
                crate::tokenizer::finite_automata::_seq(exprs)
            }
            FastCombinator::Choice(children) => {
                let mut exprs = Vec::new();
                for child in children.iter() {
                    exprs.push(child.to_dfa_expr());
                }
                crate::tokenizer::finite_automata::_choice(exprs)
            }
            FastCombinator::Opt(parser) => {
                let expr = parser.to_dfa_expr();
                crate::tokenizer::finite_automata::rep1(expr)
            }
            FastCombinator::Repeat1(parser) => {
                let expr = parser.to_dfa_expr();
                crate::tokenizer::finite_automata::rep1(expr)
            }
            FastCombinator::Eps => {
                crate::tokenizer::finite_automata::eps()
            }
            FastCombinator::EatU8(u8set) => {
                let mut choices = Vec::new();
                for byte in u8set.iter() {
                    choices.push(crate::tokenizer::finite_automata::char(byte));
                }
                crate::tokenizer::finite_automata::_choice(choices)
            }
            FastCombinator::EatByteStringChoiceFast(root) => {
                let mut choices = Vec::new();
                for bytes in root.to_vec() {
                    choices.push(crate::tokenizer::finite_automata::_seq(bytes.into_iter().map(crate::tokenizer::finite_automata::char).collect()));
                }
                crate::tokenizer::finite_automata::_choice(choices)
            }
        }
    }

    pub fn to_dfa(&self) -> Regex {
        let expr = self.to_dfa_expr();
        expr.build()
    }
}

pub trait FastCombinatorTrait {
    fn parse(&self, bytes: &[u8]) -> FastCombinatorResult;
}

pub trait FastParserTrait {
    fn parse(&mut self, bytes: &[u8]) -> FastParserResult;
    fn get_u8set(&self) -> U8Set;
}

impl FastCombinatorTrait for FastCombinator {
    fn parse(&self, bytes: &[u8]) -> FastCombinatorResult {
        todo!()
    }
}

impl FastParserTrait for FastParser {
    fn parse(&mut self, bytes: &[u8]) -> FastParserResult {
        todo!()
    }

    fn get_u8set(&self) -> U8Set {
        todo!()
    }
}

pub fn seq_fast(parsers: Vec<FastCombinator>) -> FastCombinator {
    FastCombinator::Seq(Rc::new(parsers))
}

pub fn choice_fast(parsers: Vec<FastCombinator>) -> FastCombinator {
    FastCombinator::Choice(Rc::new(parsers))
}

pub fn opt_fast(parser: FastCombinator) -> FastCombinator {
    FastCombinator::Opt(Box::new(parser))
}

pub fn repeat1_fast(parser: FastCombinator) -> FastCombinator {
    FastCombinator::Repeat1(Rc::new(parser))
}

pub fn eat_char_fast(c: char) -> FastCombinator {
    FastCombinator::EatU8(U8Set::from_char(c))
}

pub fn eat_byte_fast(byte: u8) -> FastCombinator {
    FastCombinator::EatU8(U8Set::from_byte(byte))
}

pub fn eat_char_negation_fast(c: char) -> FastCombinator {
    FastCombinator::EatU8(U8Set::from_char(c).complement())
}

pub fn eat_char_choice_fast(chars: &str) -> FastCombinator {
    FastCombinator::EatU8(U8Set::from_chars(chars))
}

pub fn eat_char_negation_choice_fast(chars: &str) -> FastCombinator {
    FastCombinator::EatU8(U8Set::from_chars(chars).complement())
}

pub fn eat_byte_range_fast(start: u8, end: u8) -> FastCombinator {
    FastCombinator::EatU8(U8Set::from_byte_range(start..=end))
}

pub fn eat_bytestring_choice_fast(bytestrings: Vec<Vec<u8>>) -> FastCombinator {
    FastCombinator::EatByteStringChoiceFast(Rc::new(bytestrings.into()))
}

pub fn eat_string_choice_fast(strings: &[&str]) -> FastCombinator {
    eat_bytestring_choice_fast(strings.into_iter().map(|s| s.as_bytes().to_vec()).collect())
}

pub fn eat_string_fast(s: &str) -> FastCombinator {
    let mut children = vec![];
    for c in s.bytes() {
        children.push(eat_byte_fast(c));
    }
    seq_fast(children)
}

pub fn repeat0_fast(parser: FastCombinator) -> FastCombinator {
    opt_fast(repeat1_fast(parser))
}

pub fn seprep1_fast(a: FastCombinator, b: FastCombinator) -> FastCombinator {
    seq_fast(vec![a.clone(), repeat0_fast(seq_fast(vec![b, a]))])
}

pub fn seprep0_fast(a: FastCombinator, b: FastCombinator) -> FastCombinator {
    opt_fast(seprep1_fast(a, b))
}

pub fn repeatn_fast(n: usize, parser: FastCombinator) -> FastCombinator {
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
