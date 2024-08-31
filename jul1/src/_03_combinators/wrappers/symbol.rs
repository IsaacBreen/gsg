
// src/_03_combinators/wrappers/symbol.rs
use crate::{DownData, RightData, RightDataGetters};
use crate::{BaseCombinatorTrait, DynCombinatorTrait, UnambiguousParseResults};
use std::rc::Rc;

use crate::{CombinatorTrait, IntoCombinator, ParseResultTrait, ParseResults, ParserTrait};
#[derive(Debug)]
pub struct Symbol<T> {
    pub value: Rc<T>,
}

impl<T> Clone for Symbol<T> {
    fn clone(&self) -> Self {
        Symbol { value: self.value.clone() }
    }
}

impl<T: CombinatorTrait> DynCombinatorTrait for Symbol<T> {
    fn parse_dyn(&self, down_data: DownData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(down_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(down_data, bytes)
    }
}

impl<T: CombinatorTrait> CombinatorTrait for Symbol<T> {
    type Parser<'a> = T::Parser<'a> where Self: 'a;
    type Output = T::Output;
    type PartialOutput = T::PartialOutput;

    fn one_shot_parse(&self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        self.value.one_shot_parse(down_data, bytes)
    }

    fn old_parse(&self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let (parser, parse_results) = self.value.parse(down_data, bytes);
        (parser, parse_results)
    }
}

impl<T: CombinatorTrait> BaseCombinatorTrait for Symbol<T> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        f(self.value.as_ref());
    }
}

pub fn symbol<T: IntoCombinator>(value: T) -> Symbol<T::Output> {
    Symbol { value: Rc::new(value.into_combinator()) }
}