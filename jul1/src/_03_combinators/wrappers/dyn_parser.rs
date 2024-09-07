
// src/_03_combinators/wrappers/dyn_parser.rs
use crate::*;
use std::any::Any;

#[derive(Debug)]
pub struct DynCombinator<C: CombinatorTrait> {
    combinator: C,
}

impl<C: CombinatorTrait> BaseCombinatorTrait for DynCombinator<C> {
    fn as_any(&self) -> &dyn Any where Self: 'static {
        self
    }
}

impl<C: CombinatorTrait> DynCombinatorTrait for DynCombinator<C> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<C: CombinatorTrait> CombinatorTrait for DynCombinator<C> {
    type Parser<'a> = Box<dyn ParserTrait + 'a> where Self: 'a;
    type Output = C::Output;
    type PartialOutput = C::PartialOutput;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.combinator.one_shot_parse(right_data, bytes)
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let (parser, parse_results) = self.combinator.old_parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }
}

pub fn dyn_combinator<'a, C: CombinatorTrait + 'a>(combinator: C) -> Box<dyn DynCombinatorTrait + 'a> {
    Box::new(DynCombinator { combinator })
}