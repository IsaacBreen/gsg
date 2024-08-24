use std::any::Any;
use crate::*;

#[derive(Debug)]
pub struct DynCombinator<C: CombinatorTrait> {
    combinator: C,
}

impl<C: CombinatorTrait> BaseCombinatorTrait for DynCombinator<C> where for<'a> C: 'a {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<C: CombinatorTrait> DynCombinatorTrait for DynCombinator<C> where for<'a> C: 'a {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<C: CombinatorTrait> CombinatorTrait for DynCombinator<C> where for<'a> C: 'a {
    type Parser<'a> = Box<dyn ParserTrait + 'a> where Self: 'a;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.combinator.one_shot_parse(right_data, bytes)
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let (parser, parse_results) = self.combinator.old_parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }
}

pub fn dyn_combinator<C: CombinatorTrait>(combinator: C) -> Box<dyn DynCombinatorTrait> where for<'a> C: 'a {
    Box::new(DynCombinator { combinator })
}