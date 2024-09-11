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
    fn parse_dyn<'a>(&'a self, right_data: RightData, bytes: &'a [u8]) -> (Box<dyn ParserTrait + 'a>, ParseResults<C::Output>) where C::Output: 'a {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &'a [u8]) -> UnambiguousParseResults<C::Output> where C::Output: 'a {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<C: CombinatorTrait> CombinatorTrait for DynCombinator<C> {
    type Parser<'a> = Box<dyn ParserTrait + 'a> where Self: 'a;
    type Output = C::Output;

    fn one_shot_parse<'a>(&self, right_data: RightData, bytes: &'a [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'a {
        self.combinator.one_shot_parse(right_data, bytes)
    }

    fn parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        let (parser, parse_results) = self.combinator.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }
}

pub fn dyn_combinator<'a, C: CombinatorTrait + 'a>(combinator: C) -> Box<dyn DynCombinatorTrait + 'a> {
    Box::new(DynCombinator { combinator })
}