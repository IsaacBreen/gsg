
// src/_02_traits/combinator.rs
use crate::{ParseResultTrait, ParseResults, RightData, RightDataGetters, U8Set, UnambiguousParseResults, UpData, OneShotUpData};
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
    type Parser<'a>: ParserTrait<Self::Output> where Self: 'a;
    type Output: OutputTrait;

    fn old_parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b;
    fn parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        self.old_parse(right_data, bytes)
    }
    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b;
}

pub trait DynCombinatorTrait: BaseCombinatorTrait + std::fmt::Debug {
    fn parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Box<dyn ParserTrait<Self::Output> + 'a>, ParseResults<Self::Output>) where Self::Output: 'b;
    fn one_shot_parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b;
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

pub fn dumb_one_shot_parse<T: CombinatorTrait>(combinator: &T, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults<T::Output> {
    let (parser, parse_results) = combinator.old_parse(right_data, bytes);
    UnambiguousParseResults::from(parse_results)
}

pub trait ParserTrait<Output: OutputTrait>: std::fmt::Debug {
    fn get_u8set(&self) -> U8Set;
    fn parse<'b>(&mut self, bytes: &'b [u8]) -> ParseResults<Output> where Output: 'b;
    fn autoparse(&mut self, right_data: RightData, max_length: usize) -> (Vec<u8>, ParseResults<Output>) {
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

impl<T: DynCombinatorTrait + ?Sized> DynCombinatorTrait for Box<T> {
    fn parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Box<dyn ParserTrait<Self::Output> + 'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        (**self).parse_dyn(right_data, bytes)
    }

    fn one_shot_parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b {
        (**self).one_shot_parse_dyn(right_data, bytes)
    }
}

impl<T: CombinatorTrait + ?Sized> CombinatorTrait for Box<T> {
    type Parser<'a> = T::Parser<'a> where Self: 'a;
    type Output = T::Output;

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b {
        (**self).one_shot_parse(right_data, bytes)
    }

    fn old_parse<'a, 'b>(&self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        (**self).old_parse(right_data, bytes)
    }

    fn parse<'a, 'b>(&self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        (**self).parse(right_data, bytes)
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

impl<'a> ParserTrait<Output> for Box<dyn ParserTrait<Output> + 'a> where Output: OutputTrait {
    fn get_u8set(&self) -> U8Set {
        (**self).get_u8set()
    }

    fn parse<'b>(&mut self, bytes: &'b [u8]) -> ParseResults<Output> where Output: 'b {
        (**self).parse(bytes)
    }
}

impl<'b> CombinatorTrait for Box<dyn DynCombinatorTrait + 'b> {
    type Parser<'a> = Box<dyn ParserTrait<Self::Output> + 'a> where Self: 'a;
    type Output = Box<dyn std::any::Any>;

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b {
        (**self).one_shot_parse_dyn(right_data, bytes)
    }

    fn old_parse<'a, 'b>(&self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        (**self).parse_dyn(right_data, bytes)
    }
}

// Removed ParserTrait implementation for Parser enum

pub trait CombinatorTraitExt: CombinatorTrait {
    fn parser(&self, right_data: RightData) -> (Self::Parser<'_>, ParseResults<Self::Output>) {
        self.old_parse(right_data, &[])
    }
}

pub trait ParserTraitExt: ParserTrait<Output> where Output: OutputTrait {
    fn step(&mut self, c: u8) -> ParseResults<Output> {
        self.parse(&[c])
    }
}

impl<T: CombinatorTrait> CombinatorTraitExt for T {}
impl<T: ParserTrait<Output>> ParserTraitExt for T where Output: OutputTrait {}