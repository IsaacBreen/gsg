use crate::{Combinator, U8Set};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FastParserResult {
    Success(usize),
    Failure,
    Incomplete,
}

pub trait FastParserTrait {
    fn parse(&self, bytes: &[u8]) -> FastParserResult;
    fn slow(&self) -> Combinator;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seq<A: FastParserTrait, B: FastParserTrait> {
    pub(crate) a: A,
    pub(crate) b: B,
}

impl<A, B> FastParserTrait for Seq<A, B>
where
    A: FastParserTrait,
    B: FastParserTrait,
{
    fn parse(&self, bytes: &[u8]) -> FastParserResult {
        match self.a.parse(bytes) {
            FastParserResult::Success(a_len) => {
                match self.b.parse(&bytes[a_len..]) {
                    FastParserResult::Success(b_len) => FastParserResult::Success(a_len + b_len),
                    FastParserResult::Failure => FastParserResult::Failure,
                    FastParserResult::Incomplete => FastParserResult::Incomplete,
                }
            }
            FastParserResult::Failure => FastParserResult::Failure,
            FastParserResult::Incomplete => FastParserResult::Incomplete,
        }
    }

    fn slow(&self) -> Combinator {
        let mut all_children: crate::VecX<Combinator> = crate::vecx![];
        let a_slow = self.a.slow();
        let b_slow = self.b.slow();
        match a_slow {
            Combinator::Seq(crate::Seq { children, .. }) => {
                all_children.extend(children.iter().cloned());
            }
            _ => all_children.push(a_slow),
        }
        match b_slow {
            Combinator::Seq(crate::Seq { children, .. }) => {
                all_children.extend(children.iter().cloned());
            }
            _ => all_children.push(b_slow),
        }
        crate::_seq(all_children)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Choice<A: FastParserTrait, B: FastParserTrait> {
    pub(crate) a: A,
    pub(crate) b: B,
}

impl<A, B> FastParserTrait for Choice<A, B>
where
    A: FastParserTrait,
    B: FastParserTrait,
{
    fn parse(&self, bytes: &[u8]) -> FastParserResult {
        match self.a.parse(bytes) {
            FastParserResult::Success(len) => FastParserResult::Success(len),
            FastParserResult::Failure => match self.b.parse(bytes) {
                FastParserResult::Success(len) => FastParserResult::Success(len),
                FastParserResult::Failure => FastParserResult::Failure,
                FastParserResult::Incomplete => FastParserResult::Incomplete,
            },
            FastParserResult::Incomplete => FastParserResult::Incomplete,
        }
    }

    fn slow(&self) -> Combinator {
        let mut all_children: crate::VecX<Combinator> = crate::vecx![];
        let a_slow = self.a.slow();
        let b_slow = self.b.slow();
        match a_slow {
            Combinator::Choice(crate::Choice { children, .. }) => {
                all_children.extend(children.iter().cloned());
            }
            _ => all_children.push(a_slow),
        }
        match b_slow {
            Combinator::Choice(crate::Choice { children, .. }) => {
                all_children.extend(children.iter().cloned());
            }
            _ => all_children.push(b_slow),
        }
        crate::_choice(all_children)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Opt<A: FastParserTrait> {
    pub(crate) a: A,
}

impl<A> FastParserTrait for Opt<A>
where
    A: FastParserTrait,
{
    fn parse(&self, bytes: &[u8]) -> FastParserResult {
        match self.a.parse(bytes) {
            FastParserResult::Success(len) => FastParserResult::Success(len),
            FastParserResult::Failure => FastParserResult::Success(0),
            FastParserResult::Incomplete => FastParserResult::Incomplete,
        }
    }

    fn slow(&self) -> Combinator {
        crate::opt(self.a.slow()).into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Repeat1<A: FastParserTrait> {
    pub(crate) a: A,
}

impl<A> FastParserTrait for Repeat1<A>
where
    A: FastParserTrait,
{
    fn parse(&self, bytes: &[u8]) -> FastParserResult {
        let mut total_len = 0;
        loop {
            match self.a.parse(&bytes[total_len..]) {
                FastParserResult::Success(len) => {
                    if len == 0 {
                        break;
                    }
                    total_len += len;
                }
                FastParserResult::Failure => {
                    if total_len == 0 {
                        return FastParserResult::Failure;
                    } else {
                        break;
                    }
                }
                FastParserResult::Incomplete => return FastParserResult::Incomplete,
            }
        }
        FastParserResult::Success(total_len)
    }

    fn slow(&self) -> Combinator {
        crate::repeat1(self.a.slow()).into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Eps;

impl FastParserTrait for Eps {
    fn parse(&self, bytes: &[u8]) -> FastParserResult {
        FastParserResult::Success(0)
    }

    fn slow(&self) -> Combinator {
        crate::eps().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatU8Parser {
    pub(crate) u8set: U8Set,
}

impl FastParserTrait for EatU8Parser {
    fn parse(&self, bytes: &[u8]) -> FastParserResult {
        if bytes.is_empty() {
            return FastParserResult::Incomplete;
        }
        if self.u8set.contains(bytes[0]) {
            FastParserResult::Success(1)
        } else {
            FastParserResult::Failure
        }
    }

    fn slow(&self) -> Combinator {
        crate::EatU8 { u8set: self.u8set }.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatByteStringChoiceFast {
    pub(crate) root: crate::TrieNode,
}

impl FastParserTrait for EatByteStringChoiceFast {
    fn parse(&self, bytes: &[u8]) -> FastParserResult {
        let mut current_node = &self.root;
        let mut bytes_consumed = 0;

        for &byte in bytes {
            if current_node.valid_bytes.contains(byte) {
                let child_index = current_node.valid_bytes.bitset.count_bits_before(byte) as usize;
                if child_index < current_node.children.len() {
                    current_node = &current_node.children[child_index];
                    bytes_consumed += 1;
                    if current_node.is_end {
                        return FastParserResult::Success(bytes_consumed);
                    }
                } else {
                    return FastParserResult::Failure;
                }
            } else {
                return FastParserResult::Failure;
            }
        }

        if bytes_consumed > 0 && current_node.is_end {
            FastParserResult::Success(bytes_consumed)
        } else {
            FastParserResult::Incomplete
        }
    }

    fn slow(&self) -> Combinator {
        crate::EatByteStringChoice { root: std::rc::Rc::new(self.root.clone()) }.into()
    }
}

pub fn seq_fast<A: FastParserTrait, B: FastParserTrait>(a: A, b: B) -> impl FastParserTrait {
    Seq { a, b }
}

pub fn choice_fast<A: FastParserTrait, B: FastParserTrait>(a: A, b: B) -> impl FastParserTrait {
    Choice { a, b }
}

#[macro_export]
macro_rules! seq_fast {
    // Implement for up to eight arguments. Keep it balanced.
    ($x0:expr $(,)?) => { $x0 };
    ($x0:expr, $x1:expr $(,)?) => { $crate::fast_combinator::seq_fast($x0, $x1) };
    ($x0:expr, $x1:expr, $x2:expr $(,)?) => { $crate::seq_fast($x0, $crate::seq_fast!($x1, $x2)) };
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr $(,)?) => { $crate::seq_fast($crate::seq_fast!($x0, $x1), $crate::seq_fast!($x2, $x3)) };
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $x4:expr $(,)?) => { $crate::seq_fast($crate::seq_fast!($x0, $x1), $crate::seq_fast!($x2, $x3, $x4)) };
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr $(,)?) => { $crate::seq_fast($crate::seq_fast!($x0, $x1, $x2), $crate::seq_fast!($x3, $x4, $x5)) };
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr, $x6:expr $(,)?) => { $crate::seq_fast($crate::seq_fast!($x0, $x1, $x2), $crate::seq_fast!($x3, $x4, $x5, $x6)) };
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr, $x6:expr, $x7:expr $(,)?) => { $crate::seq_fast($crate::seq_fast!($x0, $x1, $x2, $x3), $crate::seq_fast!($x4, $x5, $x6, $x7)) };
    // And all the rest
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr, $x6:expr, $x7:expr, $($rest:expr),+) => {
        $crate::seq_fast($crate::seq_fast!($x0, $x1, $x2, $x3), $crate::seq_fast!($x4, $x5, $x6, $x7, $($rest),+))
    };
}

#[macro_export]
macro_rules! choice_fast {
    // Implement for up to eight arguments. Keep it balanced.
    ($x0:expr $(,)?) => { $crate::fast_combinator::choice_fast($x0) };
    ($x0:expr, $x1:expr $(,)?) => { $crate::fast_combinator::choice_fast($x0, $x1) };
    ($x0:expr, $x1:expr, $x2:expr $(,)?) => { $crate::fast_combinator::choice_fast($x0, $crate::choice_fast!($x1, $x2)) };
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr $(,)?) => { $crate::fast_combinator::choice_fast($crate::choice_fast!($x0, $x1), $crate::choice_fast!($x2, $x3)) };
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $x4:expr $(,)?) => { $crate::fast_combinator::choice_fast($crate::choice_fast!($x0, $x1), $crate::choice_fast!($x2, $x3, $x4)) };
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr $(,)?) => { $crate::fast_combinator::choice_fast($crate::choice_fast!($x0, $x1, $x2), $crate::choice_fast!($x3, $x4, $x5)) };
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr, $x6:expr $(,)?) => { $crate::fast_combinator::choice_fast($crate::choice_fast!($x0, $x1, $x2), $crate::choice_fast!($x3, $x4, $x5, $x6)) };
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr, $x6:expr, $x7:expr $(,)?) => { $crate::fast_combinator::choice_fast($crate::choice_fast!($x0, $x1, $x2, $x3), $crate::choice_fast!($x4, $x5, $x6, $x7)) };
    // And all the rest
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr, $x6:expr, $x7:expr, $($rest:expr),+) => {
        $crate::fast_combinator::choice_fast($crate::fast_combinator::choice_fast($x0, $x1, $x2, $x3), $crate::fast_combinator::choice_fast($x4, $x5, $x6, $x7, $($rest),+))
    };
}

pub fn opt_fast<A: FastParserTrait>(a: A) -> impl FastParserTrait {
    Opt { a }
}

pub fn repeat1_fast<A: FastParserTrait>(a: A) -> Repeat1<A> {
    Repeat1 { a }
}

pub fn eat_char_fast(c: char) -> EatU8Parser {
    EatU8Parser { u8set: U8Set::from_char(c) }
}

pub fn eat_bytestring_choice_fast(bytestrings: Vec<Vec<u8>>) -> EatByteStringChoiceFast {
    let mut build_root = crate::BuildTrieNode::new();
    for bytestring in bytestrings {
        build_root.insert(&bytestring);
    }
    let root = build_root.to_optimized_trie_node();
    EatByteStringChoiceFast { root }
}

// Derived combinators

pub fn repeat0_fast<A: FastParserTrait>(a: A) -> impl FastParserTrait {
    opt_fast(repeat1_fast(a))
}

pub fn seprep1_fast<A: FastParserTrait + Clone, B: FastParserTrait>(a: A, b: B) -> impl FastParserTrait {
    seq_fast!(a.clone(), repeat0_fast(seq_fast!(b, a)))
}

pub fn seprep0_fast<A: FastParserTrait + Clone, B: FastParserTrait>(a: A, b: B) -> impl FastParserTrait {
    opt_fast(seprep1_fast(a, b))
}