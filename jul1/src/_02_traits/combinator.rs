
// src/_02_traits/combinator.rs
use crate::{ParseResultTrait, ParseResults, RightData, U8Set, UnambiguousParseResults, UpData, OneShotUpData, DownData};
use std::fmt::Display;

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
pub trait CombinatorTrait: BaseCombinatorTrait + DynCombinatorTrait + std::fmt::Debug {
    type Parser<'a>: ParserTrait where Self: 'a;
    type Output;
    type PartialOutput;

    fn old_parse<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'a>, ParseResults);
    fn parse<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'a>, ParseResults) {
        self.old_parse(down_data, bytes)
    }
    fn one_shot_parse(&self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults;
}

pub trait DynCombinatorTrait: BaseCombinatorTrait + std::fmt::Debug {
    fn parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> (Box<dyn ParserTrait + 'a>, ParseResults);
    fn one_shot_parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults;
}

pub trait BaseCombinatorTrait {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static;
    fn type_name(&self) -> &str {
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

pub fn dumb_one_shot_parse<T: CombinatorTrait>(combinator: &T, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
    let (parser, parse_results) = combinator.old_parse(down_data, bytes);
    UnambiguousParseResults::from(parse_results)
}

pub trait ParserTrait: std::fmt::Debug {
    fn get_u8set(&self) -> U8Set;
    fn parse(&mut self, down_data: DownData, bytes: &[u8]) -> ParseResults;
    fn autoparse(&mut self, down_data: DownData, max_length: usize) -> (Vec<u8>, ParseResults) {
        let mut prefix = Vec::new();
        let mut parse_results = ParseResults::empty_finished();
        while prefix.len() < max_length {
            let u8set = self.get_u8set();
            if u8set.len() == 1 {
                let c = u8set.iter().next().unwrap();
                let new_parse_results = self.parse(down_data.clone(), &[c]);
                parse_results.combine_seq(new_parse_results);
                prefix.push(c);
            } else {
                break;
            }
        }
        (prefix, parse_results)
    }
}

impl<T: DynCombinatorTrait + ?Sized> DynCombinatorTrait for Box<T> {
    fn parse_dyn(&self, down_data: DownData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        (**self).parse_dyn(down_data, bytes)
    }

    fn one_shot_parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        (**self).one_shot_parse_dyn(down_data, bytes)
    }
}

impl<T: CombinatorTrait + ?Sized> CombinatorTrait for Box<T> {
    type Parser<'a> = T::Parser<'a> where Self: 'a;
    type Output = T::Output;
    type PartialOutput = T::PartialOutput;

    fn one_shot_parse(&self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        (**self).one_shot_parse(down_data, bytes)
    }

    fn old_parse(&self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        (**self).old_parse(down_data, bytes)
    }

    fn parse(&self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        (**self).parse(down_data, bytes)
    }

}

impl<T: BaseCombinatorTrait + ?Sized> BaseCombinatorTrait for Box<T> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        (**self).as_any()
    }
    fn type_name(&self) -> &str {
        (**self).type_name()
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        (**self).apply_to_children(f);
    }
    fn compile_inner(&self) {
        (**self).compile_inner();
    }
}

impl<'a> ParserTrait for Box<dyn ParserTrait + 'a> {
    fn get_u8set(&self) -> U8Set {
        (**self).get_u8set()
    }

    fn parse(&mut self, down_data: DownData, bytes: &[u8]) -> ParseResults {
        (**self).parse(down_data, bytes)
    }
}

impl<'b> CombinatorTrait for Box<dyn DynCombinatorTrait + 'b> {
    type Parser<'a> = Box<dyn ParserTrait + 'a> where Self: 'a;
    type Output = Box<dyn std::any::Any>;
    type PartialOutput = Box<dyn std::any::Any>;

    fn one_shot_parse(&self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        (**self).one_shot_parse_dyn(down_data, bytes)
    }

    fn old_parse(&self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        (**self).parse_dyn(down_data, bytes)
    }
}

// Removed ParserTrait implementation for Parser enum

pub trait CombinatorTraitExt: CombinatorTrait {
    fn parser(&self, down_data: DownData) -> (Self::Parser<'_>, ParseResults) {
        self.old_parse(down_data, &[])
    }
}

pub trait ParserTraitExt: ParserTrait {
    fn step(&mut self, down_data: DownData, c: u8) -> ParseResults {
        self.parse(down_data, &[c])
    }
}

impl<T: CombinatorTrait> CombinatorTraitExt for T {}
impl<T: ParserTrait> ParserTraitExt for T {}