// src/combinator.rs
use std::collections::HashMap;
use std::any::Any;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;
use crate::{CacheContext, CacheContextParser, Cached, CachedParser, CheckRightData, Choice, ChoiceParser, Deferred, EatByteStringChoice, EatByteStringChoiceParser, EatString, EatStringParser, EatU8, EatU8Parser, Eps, EpsParser, Fail, FailParser, ForbidFollows, ForbidFollowsCheckNot, ForbidFollowsClear, IndentCombinator, IndentCombinatorParser, Lookahead, MutateRightData, ExcludeBytestrings, ExcludeBytestringsParser, ParseResults, Repeat1, Repeat1Parser, ParseResultTrait, Seq, SeqParser, Symbol, Tagged, TaggedParser, U8Set, ProfiledParser, Profiled, Opt, WeakRef, StrongRef, BruteForceParser, BruteForce, Continuation, ContinuationParser, FastCombinatorWrapper, profile, FastParserWrapper, Seq2, Choice2, OwningParser, RightData, UnambiguousParseError, UnambiguousParseResults};
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

// Removed Parser enum
pub trait CombinatorTrait: BaseCombinatorTrait + std::fmt::Debug {
    type Parser<'a>: ParserTrait where Self: 'a;
    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults);
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        self.old_parse(right_data, bytes)
        // let (mut parser, mut parse_results) = self.old_parse(right_data, &[]);
        // if !parse_results.done() {
        //     let new_parse_results = parser.parse(bytes);
        //     parse_results.combine_seq(new_parse_results);
        // }
        // (parser, parse_results)
    }
    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults;
}

pub trait DynCombinatorTrait: BaseCombinatorTrait + std::fmt::Debug {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait>, ParseResults);
}

pub trait BaseCombinatorTrait {
    fn as_any(&self) -> &dyn std::any::Any;
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {}
    fn compile(mut self) -> Self
    where
        Self: Sized
    {
        self.compile_inner();
        self
    }
    fn compile_inner(&self) {
        self.apply_to_children(&mut |combinator| combinator.compile_inner());
    }
}

pub fn dumb_one_shot_parse<T: CombinatorTrait>(combinator: &T, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
    let (parser, parse_results) = combinator.old_parse(right_data, bytes);
    UnambiguousParseResults::from(parse_results)
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
    type Parser<'a> = T::Parser<'a> where Self: 'a;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        (**self).one_shot_parse(right_data, bytes)
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        (**self).old_parse(right_data, bytes)
    }

    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        (**self).parse(right_data, bytes)
    }
}

impl<T: CombinatorTrait + ?Sized> BaseCombinatorTrait for Box<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        (**self).as_any()
    }
    fn type_name(&self) -> &'static str {
        (**self).type_name()
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        (**self).apply_to_children(f);
    }
    fn compile_inner(&self) {
        (**self).compile_inner();
    }
}

// Removed ParserTrait implementation for Parser enum

pub trait CombinatorTraitExt: CombinatorTrait {
    fn parser(&self, right_data: RightData) -> (Self::Parser<'_>, ParseResults) {
        self.old_parse(right_data, &[])
    }
}

pub trait ParserTraitExt: ParserTrait {
    fn step(&mut self, c: u8) -> ParseResults {
        self.parse(&[c])
    }
}

impl<T: CombinatorTrait> CombinatorTraitExt for T {}
impl<T: ParserTrait> ParserTraitExt for T {}