use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, RightData, U8Set};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub value: Rc<Combinator>,
}

impl CombinatorTrait for Symbol {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (parser, parse_results) = self.value.parse(right_data, bytes);
        (parser, parse_results)
    }
}

pub fn symbol(value: impl Into<Combinator>) -> Symbol {
    Symbol { value: Rc::new(value.into()) }
}

impl From<&Symbol> for Combinator {
    fn from(value: &Symbol) -> Self {
        Combinator::Symbol(value.clone())
    }
}
