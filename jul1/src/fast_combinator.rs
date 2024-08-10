use crate::U8Set;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FastParserResult {
    Success(usize),
    Incomplete,
    Failure,
}

pub trait FastParserTrait {
    fn parse(&mut self, bytes: &[u8]) -> FastParserResult;
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
    fn parse(&mut self, bytes: &[u8]) -> FastParserResult {
        match self.a.parse(bytes) {
            FastParserResult::Success(a_len) => {
                match self.b.parse(&bytes[a_len..]) {
                    FastParserResult::Success(b_len) => FastParserResult::Success(a_len + b_len),
                    FastParserResult::Incomplete => FastParserResult::Incomplete,
                    FastParserResult::Failure => FastParserResult::Failure,
                }
            }
            FastParserResult::Incomplete => FastParserResult::Incomplete,
            FastParserResult::Failure => FastParserResult::Failure,
        }
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
    fn parse(&mut self, bytes: &[u8]) -> FastParserResult {
        match self.a.parse(bytes) {
            FastParserResult::Success(len) => FastParserResult::Success(len),
            FastParserResult::Incomplete => match self.b.parse(bytes) {
                FastParserResult::Success(len) => FastParserResult::Success(len),
                FastParserResult::Incomplete => FastParserResult::Incomplete,
                FastParserResult::Failure => FastParserResult::Incomplete,
            },
            FastParserResult::Failure => self.b.parse(bytes),
        }
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
    fn parse(&mut self, bytes: &[u8]) -> FastParserResult {
        match self.a.parse(bytes) {
            FastParserResult::Success(len) => FastParserResult::Success(len),
            FastParserResult::Incomplete => FastParserResult::Success(0),
            FastParserResult::Failure => FastParserResult::Success(0),
        }
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
    fn parse(&mut self, bytes: &[u8]) -> FastParserResult {
        let mut total_len = 0;
        loop {
            match self.a.parse(&bytes[total_len..]) {
                FastParserResult::Success(len) => {
                    if len == 0 {
                        break;
                    }
                    total_len += len;
                }
                FastParserResult::Incomplete => return FastParserResult::Incomplete,
                FastParserResult::Failure => {
                    if total_len == 0 {
                        return FastParserResult::Failure;
                    } else {
                        break;
                    }
                }
            }
        }
        FastParserResult::Success(total_len)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatU8Parser {
    pub(crate) u8set: U8Set,
}

impl FastParserTrait for EatU8Parser {
    fn parse(&mut self, bytes: &[u8]) -> FastParserResult {
        if bytes.is_empty() {
            return FastParserResult::Incomplete;
        }
        if self.u8set.contains(bytes[0]) {
            FastParserResult::Success(1)
        } else {
            FastParserResult::Failure
        }
    }
}

#[macro_export]
macro_rules! seq_fast {
    ($a:expr) => { $a };
    ($a:expr, $($b:expr),+) => {
        $crate::fast_combinator::Seq {
            a: $a,
            b: $crate::seq_fast!($($b),+),
        }
    };
}

#[macro_export]
macro_rules! choice_fast {
    ($a:expr) => { $a };
    ($a:expr, $($b:expr),+) => {
        $crate::fast_combinator::Choice {
            a: $a,
            b: $crate::choice_fast!($($b),+),
        }
    };
}

pub fn opt_fast<A: FastParserTrait>(a: A) -> Opt<A> {
    Opt {
        a,
    }
}

pub fn repeat1_fast<A: FastParserTrait>(a: A) -> Repeat1<A> {
    Repeat1 {
        a,
    }
}

pub fn eat_char_fast(c: char) -> EatU8Parser {
    EatU8Parser {
        u8set: U8Set::from_char(c),
    }
}

// Derived combinators

pub fn repeat0_fast<A: FastParserTrait>(a: A) -> Opt<Repeat1<A>> {
    opt_fast(repeat1_fast(a))
}

pub fn seprep1_fast<A: FastParserTrait, B: FastParserTrait>(a: A, b: B) -> Seq<A, Repeat0<Seq<B, A>>> {
    seq_fast!(a, repeat0_fast(seq_fast!(b, a)))
}

pub fn seprep0_fast<A: FastParserTrait, B: FastParserTrait>(a: A, b: B) -> Opt<Seq<A, Repeat0<Seq<B, A>>>> {
    opt_fast(seprep1_fast(a, b))
}