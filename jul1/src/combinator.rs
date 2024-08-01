use std::fmt::Display;
use std::ops::AddAssign;
use crate::{CacheContext, CacheContextParser, Cached, CachedParser, CacheFirst, CacheFirstContext, CacheFirstContextParser, CacheFirstParser, CheckRightData, CheckRightDataParser, Choice, ChoiceParser, Deferred, EatByteStringChoice, EatByteStringChoiceParser, EatString, EatStringParser, EatU8, EatU8Parser, Eps, EpsParser, Fail, FailParser, ForbidFollows, ForbidFollowsCheckNot, ForbidFollowsClear, ForwardRef, FrameStackOp, FrameStackOpParser, IndentCombinator, IndentCombinatorParser, Lookahead, MutateRightData, MutateRightDataParser, ParseResults, Repeat1, Repeat1Parser, RightData, Seq, SeqParser, Symbol, SymbolParser, Tagged, TaggedParser, WithNewFrame, WithNewFrameParser};
use crate::stats::Stats;

macro_rules! define_enum {
    ($name:ident, $($variants:ident),*) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                $variants($variants),
            )*
        }
    };
}

#[macro_export]
macro_rules! match_enum {
    ($expr:expr, $enum:ident, $inner:ident => $arm:expr, $($variant:ident),*) => {
        match $expr {
            $(
                $enum::$variant($inner) => $arm,
            )*
        }
    };
}

define_enum!(
    Combinator,
    Seq,
    Choice,
    EatU8,
    EatString,
    Eps,
    Fail,
    CacheContext,
    Cached,
    CacheFirstContext,
    CacheFirst,
    IndentCombinator,
    FrameStackOp,
    MutateRightData,
    Repeat1,
    Symbol,
    Tagged,
    ForwardRef,
    WithNewFrame,
    ForbidFollows,
    ForbidFollowsClear,
    ForbidFollowsCheckNot,
    EatByteStringChoice,
    CheckRightData,
    Deferred,
    Lookahead
);

define_enum!(
    Parser,
    SeqParser,
    ChoiceParser,
    EatU8Parser,
    EatStringParser,
    EpsParser,
    FailParser,
    CacheContextParser,
    CachedParser,
    CacheFirstParser,
    CacheFirstContextParser,
    IndentCombinatorParser,
    FrameStackOpParser,
    MutateRightDataParser,
    Repeat1Parser,
    SymbolParser,
    TaggedParser,
    WithNewFrameParser,
    EatByteStringChoiceParser,
    CheckRightDataParser
);

macro_rules! match_combinator {
    ($expr:expr, $inner:ident => $arm:expr) => {
        $crate::match_enum!($expr, Combinator, $inner => $arm,
            Seq,
            Choice,
            EatU8,
            EatString,
            Eps,
            Fail,
            CacheContext,
            Cached,
            CacheFirstContext,
            CacheFirst,
            IndentCombinator,
            FrameStackOp,
            MutateRightData,
            Repeat1,
            Symbol,
            Tagged,
            ForwardRef,
            WithNewFrame,
            ForbidFollows,
            ForbidFollowsClear,
            ForbidFollowsCheckNot,
            EatByteStringChoice,
            CheckRightData,
            Deferred,
            Lookahead
        )
    };
}

#[macro_export]
macro_rules! match_parser {
    ($expr:expr, $inner:ident => $arm:expr) => {
        $crate::match_enum!($expr, Parser, $inner => $arm,
            SeqParser,
            ChoiceParser,
            EatU8Parser,
            EatStringParser,
            EpsParser,
            FailParser,
            CacheContextParser,
            CachedParser,
            CacheFirstParser,
            CacheFirstContextParser,
            IndentCombinatorParser,
            FrameStackOpParser,
            MutateRightDataParser,
            Repeat1Parser,
            SymbolParser,
            TaggedParser,
            WithNewFrameParser,
            EatByteStringChoiceParser,
            CheckRightDataParser
        )
    };
}

pub trait CombinatorTrait {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        self.parser_with_steps(right_data, &[])
    }
    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults);
    // fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
    //     let (mut parser, mut parse_results0) = self.parser(right_data);
    //     let parse_results1 = parser.steps(bytes);
    //     parse_results0.combine(parse_results1);
    //     (parser, parse_results0)
    // }
}

pub trait ParserTrait {
    fn step(&mut self, c: u8) -> ParseResults;
    fn steps(&mut self, bytes: &[u8]) -> ParseResults;
}

impl CombinatorTrait for Combinator {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        match_combinator!(self, inner => inner.parser(right_data))
    }

    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        match_combinator!(self, inner => inner.parser_with_steps(right_data, bytes))
    }
}

impl ParserTrait for Parser {
    fn step(&mut self, c: u8) -> ParseResults {
        match_parser!(self, inner => inner.step(c))
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        match_parser!(self, inner => inner.steps(bytes))
    }
}

impl Combinator {
    pub fn type_name(&self) -> String {
        match_combinator!(self, inner => std::any::type_name_of_val(&inner)).to_string()
    }
}
