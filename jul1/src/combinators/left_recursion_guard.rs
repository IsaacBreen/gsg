use std::any::Any;
use std::fmt::{Debug, Formatter};
use crate::*;

pub struct LeftRecursionGuardData {

}

#[derive(Debug)]
pub struct LeftRecursionGuard<T: CombinatorTrait> {
    inner: T,
}

impl<T: CombinatorTrait + 'static> BaseCombinatorTrait for LeftRecursionGuard<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<T: CombinatorTrait + 'static> DynCombinatorTrait for LeftRecursionGuard<T> {
    fn parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + 'a>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.inner.one_shot_parse(right_data, bytes)
    }
}

impl<T: CombinatorTrait + 'static> CombinatorTrait for LeftRecursionGuard<T> {
    type Parser<'a> = T::Parser<'a> where T: 'a;

    fn old_parse<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'a>, ParseResults) {
        todo!()
    }

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        todo!()
    }
}

pub fn left_recursion_guard<T: CombinatorTrait>(inner: T) -> LeftRecursionGuard<T> {
    LeftRecursionGuard {
        inner
    }
}