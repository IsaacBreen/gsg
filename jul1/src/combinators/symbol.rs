use std::any::Any;
use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, RightData, U8Set};
#[derive(Debug, Clone)]
pub struct Symbol {
    pub value: Rc<Combinator>,
}

impl CombinatorTrait for Symbol {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(self.value.as_ref());
    }

    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (parser, parse_results) = self.value.parse(right_data, bytes);
        (parser, parse_results)
    }
}

pub fn symbol(value: impl CombinatorTrait) -> Symbol {
    Symbol { value: Rc::new(Box::new(value)) }
}

// impl From<&Symbol> for Symbol {
//     fn from(value: &Symbol) -> Self {
//         Combinator::Symbol(value.clone())
//     }
// }
