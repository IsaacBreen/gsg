use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, RightData, Stats};

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
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let (inner, parse_results) = self.value.parser(right_data);
        (Parser::Symbol(SymbolParser { inner: Box::new(inner), symbol_value: self.value.clone() }), parse_results)
    }
}

impl ParserTrait for SymbolParser {
    fn step(&mut self, c: u8) -> ParseResults {
        self.inner.step(c)
    }

    fn collect_stats(&self, stats: &mut Stats) {
        todo!()
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Parser> + 'a> {
        todo!()
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut Parser> + 'a> {
        todo!()
    }
}

pub fn symbol(value: Combinator) -> Combinator {
    Combinator::Symbol(Symbol { value: Rc::new(value) })
}
