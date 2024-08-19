// src/combinator.rs
use std::collections::HashMap;
use std::any::Any;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;
use crate::{CacheContext, CacheContextParser, Cached, CachedParser, CheckRightData, Choice, ChoiceParser, Deferred, EatByteStringChoice, EatByteStringChoiceParser, EatString, EatStringParser, EatU8, EatU8Parser, Eps, EpsParser, Fail, FailParser, ForbidFollows, ForbidFollowsCheckNot, ForbidFollowsClear, IndentCombinator, IndentCombinatorParser, Lookahead, MutateRightData, ExcludeBytestrings, ExcludeBytestringsParser, ParseResults, Repeat1, Repeat1Parser, RightData, Seq, SeqParser, Symbol, Tagged, TaggedParser, U8Set, ProfiledParser, Profiled, Opt, WeakRef, StrongRef, BruteForceParser, BruteForce, Continuation, ContinuationParser, FastCombinatorWrapper, profile, FastParserWrapper, Seq2, Choice2, OwningParser};
use crate::stats::Stats;
use std::cell::RefCell;

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

pub type Combinator = Box<dyn CombinatorTrait>;

#[derive(Debug)]
pub enum Parser<'a> {
    SeqParser(SeqParser<'a>),
    ChoiceParser(ChoiceParser<'a>),
    EatU8Parser(EatU8Parser),
    EatStringParser(EatStringParser),
    EpsParser(EpsParser),
    FailParser(FailParser),
    CacheContextParser(CacheContextParser<'a>),
    CachedParser(CachedParser),
    IndentCombinatorParser(IndentCombinatorParser<'a>),
    Repeat1Parser(Repeat1Parser<'a>),
    EatByteStringChoiceParser(EatByteStringChoiceParser),
    ExcludeBytestringsParser(ExcludeBytestringsParser<'a>),
    ProfiledParser(ProfiledParser<'a>),
    BruteForceParser(BruteForceParser),
    ContinuationParser(ContinuationParser),
    FastParserWrapper(FastParserWrapper),
    DynParser(Box<dyn ParserTrait + 'a>),
    OwningParser(OwningParser<'a>),
    TaggedParser(TaggedParser<'a>),
}

impl ParserTrait for Box<Parser<'_>> {
    fn get_u8set(&self) -> U8Set {
        let inner = &**self;
        inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let inner = &mut **self;
        inner.parse(bytes)
    }
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
            ForbidFollows,
            ForbidFollowsClear,
            ForbidFollowsCheckNot,
            EatByteStringChoice,
            CheckRightData,
            Deferred,
            Lookahead,
            ExcludeBytestrings,
            Profiled,
            Opt,
            Repeat0,
            SepRep1,
            WeakRef,
            StrongRef,
            BruteForce,
            Continuation,
            Fast,
            Dyn,
            DynRc
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
            ExcludeBytestringsParser,
            ProfiledParser,
            BruteForceParser,
            ContinuationParser,
            FastParserWrapper,
            DynParser,
            OwningParser,
            TaggedParser
        )
    };
}

pub trait CombinatorTrait: std::fmt::Debug {
    fn as_any(&self) -> &dyn std::any::Any;
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {}
    fn apply_mut(&mut self, f: &mut dyn FnMut(&mut dyn CombinatorTrait)) {}
    fn parse<'a>(&'a self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'a>, ParseResults);
    fn compile(mut self) -> Self where Self: Sized {
        self.compile_mut();
        self
    }
    fn compile_mut(&mut self) {
        self.apply_mut(&mut |combinator| combinator.compile_mut());
    }
}

pub trait ParserTrait: std::fmt::Debug {
    fn get_u8set(&self) -> U8Set;
    fn parse(&mut self, bytes: &[u8]) -> ParseResults;
    fn autoparse(&mut self, max_length: usize) -> (Vec<u8>, ParseResults) {
        let mut prefix = Vec::new();
        let mut parse_results = ParseResults::empty_finished();
        while prefix.len() < max_length {
            let u8set = self.get_u8set();
            if u8set.len() == 1 {
                let c = u8set.iter().next().unwrap();
                let new_parse_results = self.parse(&[c]);
                parse_results.combine_seq(new_parse_results);
                prefix.push(c);
            } else {
                break;
            }
        }
        (prefix, parse_results)
    }
}

impl<T: CombinatorTrait + ?Sized> CombinatorTrait for Box<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        (**self).as_any()
    }

    fn type_name(&self) -> &'static str {
        (**self).type_name()
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        (**self).apply(f);
    }

    fn apply_mut(&mut self, f: &mut dyn FnMut(&mut dyn CombinatorTrait)) {
        (**self).apply_mut(f);
    }

    fn parse<'a>(&'a self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'a>, ParseResults) {
        (**self).parse(right_data, bytes)
    }
}

impl ParserTrait for Parser<'_> {
    fn get_u8set(&self) -> U8Set {
        match_parser!(self, inner => inner.get_u8set())
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let parse_results = match_parser!(self, inner => inner.parse(bytes));
        parse_results
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