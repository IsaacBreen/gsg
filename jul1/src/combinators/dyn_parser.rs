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

impl<C: CombinatorTrait> CombinatorTrait for DynCombinator<C> {
    type Parser = Box<dyn ParserTrait>;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.combinator.one_shot_parse(right_data, bytes)
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        self.combinator.old_parse(right_data, bytes)
    }
}

pub fn dyn_combinator<C: CombinatorTrait>(combinator: C) -> Box<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>> {
    Box::new(DynCombinator { combinator })
}