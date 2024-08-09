use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, RightData, U8Set};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol<'a> {
    pub value: Rc<Combinator<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolParser<'a> {
    pub inner: Box<Parser<'a>>,
    pub symbol_value: Rc<Combinator<'a>>,
}

impl CombinatorTrait<'_> for Symbol<'_> {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (inner, parse_results) = self.value.parse(right_data, bytes);
        (Parser::SymbolParser(SymbolParser { inner: Box::new(inner), symbol_value: self.value.clone() }), parse_results)
    }
}

impl ParserTrait for SymbolParser<'_> {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.inner.parse(bytes)
    }
}

pub fn symbol<'a>(value: impl Into<Combinator<'a>>) -> Symbol<'a> {
    Symbol { value: Rc::new(value.into()) }
}

impl From<&Symbol<'_>> for Combinator<'_> {
    fn from(value: &Symbol) -> Self {
        Combinator::Symbol(value.clone())
    }
}
