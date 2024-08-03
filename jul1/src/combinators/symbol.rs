use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, RightData};
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
    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (inner, parse_results) = self.value.parser_with_steps(right_data, bytes);
        (Parser::SymbolParser(SymbolParser { inner: Box::new(inner), symbol_value: self.value.clone() }), parse_results)
    }
}

impl ParserTrait for SymbolParser {
    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        self.inner.steps(bytes)
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
