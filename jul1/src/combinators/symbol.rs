use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, RightData, U8Set};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub value: Rc<Combinator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolParser {
    pub inner: Box<Parser>,
    pub symbol_value: Rc<Combinator>,
}

impl CombinatorTrait for Symbol {
    fn parse(&self, right_data: Box<RightData>, bytes: &[u8]) -> (Parser, ParseResults) {
        let (inner, parse_results) = self.value.parse(right_data, bytes);
        (Parser::SymbolParser(SymbolParser { inner: Box::new(inner), symbol_value: self.value.clone() }), parse_results)
    }
}

impl ParserTrait for SymbolParser {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.inner.parse(bytes)
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
