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
pub enum Combinator<'a> {
    Seq(Seq<'a>),
    Choice(Choice<'a>),
    EatU8(EatU8),
    EatString(EatString),
    Eps(Eps),
    Fail(Fail),
    CacheContext(CacheContext<'a>),
    Cached(Cached<'a>),
    IndentCombinator(IndentCombinator),
    MutateRightData(MutateRightData),
    Repeat1(Repeat1<'a>),
    Symbol(Symbol<'a>),
    Tagged(Tagged<'a>),
    ForwardRef(ForwardRef<'a>),
    ForwardRef2(ForwardRef2<'static>),
    ForbidFollows(ForbidFollows),
    ForbidFollowsClear(ForbidFollowsClear),
    ForbidFollowsCheckNot(ForbidFollowsCheckNot),
    EatByteStringChoice(EatByteStringChoice),
    CheckRightData(CheckRightData),
    Deferred(Deferred),
    Lookahead(Lookahead<'a>),
    ExcludeBytestrings(ExcludeBytestrings<'a>),
    LookaheadContext(LookaheadContext<'a>),
    Profiled(Profiled<'a>),
    Opt(Opt<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Parser<'a> {
    SeqParser(SeqParser<'a>),
    ChoiceParser(ChoiceParser<'a>),
    EatU8Parser(EatU8Parser),
    EatStringParser(EatStringParser),
    EpsParser(EpsParser),
    FailParser(FailParser),
    CacheContextParser(CacheContextParser<'a>),
    CachedParser(CachedParser<'a>),
    IndentCombinatorParser(IndentCombinatorParser<'a>),
    Repeat1Parser(Repeat1Parser<'a>),
    SymbolParser(SymbolParser<'a>),
    TaggedParser(TaggedParser<'a>),
    EatByteStringChoiceParser(EatByteStringChoiceParser),
    ExcludeBytestringsParser(ExcludeBytestringsParser<'a>),
    LookaheadContextParser(LookaheadContextParser<'a>),
    ProfiledParser(ProfiledParser<'a>),
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

pub trait CombinatorTrait<'a> {
    fn parse(&'a self, right_data: RightData, bytes: &[u8]) -> (Parser<'a>, ParseResults);
}

pub trait ParserTrait {
    fn get_u8set(&self) -> U8Set;
    fn parse(&mut self, bytes: &[u8]) -> ParseResults;
}

impl<'a> CombinatorTrait<'a> for Combinator<'a> {
    fn parse(&'a self, right_data: RightData, bytes: &[u8]) -> (Parser<'a>, ParseResults) {
        let (parser, parse_results) = match_combinator!(self, inner => inner.parse(right_data, bytes));
        // if !parse_results.done() && bytes.len() > 100 {
            // println!("Combinator {:?} did not consume all input. Positions: {:?}, bytes.len(): {}", self, parse_results.right_data_vec.iter().map(|x| x.position).collect::<Vec<_>>(), bytes.len());
        // }
        (parser, parse_results)
    }
}

impl<'a> ParserTrait for Parser<'a> {
    fn get_u8set(&self) -> U8Set {
        match_parser!(self, inner => inner.get_u8set())
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        match_parser!(self, inner => inner.parse(bytes))
    }
}

impl Combinator<'_> {
    pub fn type_name(&self) -> String {
        match_combinator!(self, inner => std::any::type_name_of_val(&inner)).to_string()
    }
}

pub trait CombinatorTraitExt<'a>: CombinatorTrait<'a> {
    fn parser(&'a self, right_data: RightData) -> (Parser<'a>, ParseResults) {
        self.parse(right_data, &[])
    }
}

pub trait ParserTraitExt<'a>: ParserTrait {
    fn step(&'a mut self, c: u8) -> ParseResults {
        self.parse(&[c])
    }
}

impl<'a, T: CombinatorTrait<'a>> CombinatorTraitExt<'a> for T {}
impl<'a, T: ParserTrait> ParserTraitExt<'a> for T {}