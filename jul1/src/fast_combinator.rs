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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatU8Parser {
    pub(crate) u8set: U8Set,
}

impl FastParserTrait for EatU8Parser {
    fn parse(&mut self, bytes: &[u8]) -> FastParserResult {
        todo!()
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