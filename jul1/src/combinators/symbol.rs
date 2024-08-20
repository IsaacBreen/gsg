use crate::{ApplyToChildren, UnambiguousParseResults};
use crate::RightData;
use std::any::Any;
use std::rc::Rc;

use crate::{CombinatorTrait, Parser, ParseResults, ParserTrait, ParseResultTrait, U8Set, IntoCombinator};
#[derive(Debug)]
pub struct Symbol<T> {
    pub value: Rc<T>,
}

impl<T> Clone for Symbol<T> {
    fn clone(&self) -> Self {
        Symbol { value: self.value.clone() }
    }
}

impl<T: CombinatorTrait + 'static> CombinatorTrait for Symbol<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.value.one_shot_parse(right_data, bytes)
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (parser, parse_results) = self.value.parse(right_data, bytes);
        (parser, parse_results)
    }
}

impl<T: CombinatorTrait + 'static> ApplyToChildren for Symbol<T> {
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(self.value.as_ref());
    }
}

pub fn symbol<T: IntoCombinator>(value: T) -> Symbol<T::Output> {
    Symbol { value: Rc::new(value.into_combinator()) }
}

// impl From<&Symbol> for Symbol {
//     fn from(value: &Symbol) -> Self {
//         Combinator::Symbol(value.clone())
//     }
// }
