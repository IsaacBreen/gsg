use std::any::Any;
use crate::*;

#[derive(Debug)]
pub struct DynCombinator<C: CombinatorTrait> {
    combinator: C,
}

impl<C: CombinatorTrait> BaseCombinatorTrait for DynCombinator<C> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<C: CombinatorTrait> DynCombinatorTrait for DynCombinator<C> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait>, ParseResults) {
        todo!()
    }
}

impl<C: CombinatorTrait> CombinatorTrait for DynCombinator<C> {
    type Parser<'a> = Box<dyn ParserTrait> where Self: 'a;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.combinator.one_shot_parse(right_data, bytes)
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        self.combinator.old_parse(right_data, bytes)
    }
}

pub fn dyn_combinator<C: CombinatorTrait>(combinator: C) -> Box<dyn DynCombinatorTrait> {
    Box::new(DynCombinator { combinator })
}