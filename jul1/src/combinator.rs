use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;
use crate::{CacheContext, CacheContextParser, Cached, CachedParser, CheckRightData, Choice, ChoiceParser, Deferred, EatByteStringChoice, EatByteStringChoiceParser, EatString, EatStringParser, EatU8, EatU8Parser, Eps, EpsParser, Fail, FailParser, ForbidFollows, ForbidFollowsCheckNot, ForbidFollowsClear, ForwardRef, IndentCombinator, IndentCombinatorParser, Lookahead, MutateRightData, ExcludeBytestrings, ExcludeBytestringsParser, ParseResults, Repeat1, Repeat1Parser, RightData, Seq, SeqParser, Symbol, SymbolParser, Tagged, TaggedParser, U8Set, LookaheadContext, LookaheadContextParser, ProfiledParser, Profiled, Opt, ForwardRef2};
use crate::stats::Stats;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Combinator {
    Seq(Seq),
    Choice(Choice),
    EatU8(EatU8),
    EatString(EatString),
    Eps(Eps),
    Fail(Fail),
    CacheContext(CacheContext),
    Cached(Cached),
    IndentCombinator(IndentCombinator),
    MutateRightData(MutateRightData),
    Repeat1(Repeat1),
    Symbol(Symbol),
    Tagged(Tagged),
    ForwardRef(ForwardRef),
    ForwardRef2(ForwardRef2),
    ForbidFollows(ForbidFollows),
    ForbidFollowsClear(ForbidFollowsClear),
    ForbidFollowsCheckNot(ForbidFollowsCheckNot),
    EatByteStringChoice(EatByteStringChoice),
    CheckRightData(CheckRightData),
    Deferred(Deferred),
    Lookahead(Lookahead),
    ExcludeBytestrings(ExcludeBytestrings),
    LookaheadContext(LookaheadContext),
    Profiled(Profiled),
    Opt(Opt),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Parser {
    SeqParser(SeqParser),
    ChoiceParser(ChoiceParser),
    EatU8Parser(EatU8Parser),
    EatStringParser(EatStringParser),
    EpsParser(EpsParser),
    FailParser(FailParser),
    CacheContextParser(CacheContextParser),
    CachedParser(CachedParser),
    IndentCombinatorParser(IndentCombinatorParser),
    Repeat1Parser(Repeat1Parser),
    SymbolParser(SymbolParser),
    TaggedParser(TaggedParser),
    EatByteStringChoiceParser(EatByteStringChoiceParser),
    ExcludeBytestringsParser(ExcludeBytestringsParser),
    LookaheadContextParser(LookaheadContextParser),
    ProfiledParser(ProfiledParser),
}

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
            IndentCombinator,
            MutateRightData,
            Repeat1,
            Symbol,
            Tagged,
            ForwardRef,
            ForwardRef2,
            ForbidFollows,
            ForbidFollowsClear,
            ForbidFollowsCheckNot,
            EatByteStringChoice,
            CheckRightData,
            Deferred,
            Lookahead,
            ExcludeBytestrings,
            LookaheadContext,
            Profiled,
            Opt
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
            EatByteStringChoiceParser,
            EpsParser,
            FailParser,
            CacheContextParser,
            CachedParser,
            IndentCombinatorParser,
            Repeat1Parser,
            SymbolParser,
            TaggedParser,
            ExcludeBytestringsParser,
            LookaheadContextParser,
            ProfiledParser
        )
    };
}

pub trait CombinatorTrait {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults);
}

pub trait ParserTrait {
    fn get_u8set(&self) -> U8Set;
    fn parse(&mut self, bytes: &[u8]) -> ParseResults;
}

impl CombinatorTrait for Combinator {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (parser, parse_results) = match_combinator!(self, inner => inner.parse(right_data, bytes));
        // if !parse_results.done() && bytes.len() > 100 {
            // println!("Combinator {:?} did not consume all input. Positions: {:?}, bytes.len(): {}", self, parse_results.right_data_vec.iter().map(|x| x.position).collect::<Vec<_>>(), bytes.len());
        // }
        (parser, parse_results)
    }
}

impl ParserTrait for Parser {
    fn get_u8set(&self) -> U8Set {
        match_parser!(self, inner => inner.get_u8set())
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        match_parser!(self, inner => inner.parse(bytes))
    }
}

impl Combinator {
    pub fn type_name(&self) -> String {
        match_combinator!(self, inner => std::any::type_name_of_val(&inner)).to_string()
    }
}

pub trait CombinatorTraitExt: CombinatorTrait {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        self.parse(right_data, &[])
    }
}

pub trait ParserTraitExt: ParserTrait {
    fn step(&mut self, c: u8) -> ParseResults {
        self.parse(&[c])
    }
}

impl<T: CombinatorTrait> CombinatorTraitExt for T {}
impl<T: ParserTrait> ParserTraitExt for T {}