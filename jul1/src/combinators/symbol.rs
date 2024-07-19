use std::rc::Rc;
use crate::{CombinatorTrait, IntoCombinator, ParserTrait, RightData, Stats, UpData};

pub struct Symbol<T> {
    pub value: Rc<T>,
}

pub struct SymbolParser<T> where T: CombinatorTrait {
    pub inner: T::Parser,
    pub symbol_value: Rc<T>,
}

impl<T> Clone for Symbol<T> {
    fn clone(&self) -> Self {
        Symbol { value: self.value.clone() }
    }
}

impl<T> CombinatorTrait for Symbol<T> where T: CombinatorTrait {
    type Parser = SymbolParser<T>;
    fn parser(&self, right_data: crate::parse_state::RightData) -> (Self::Parser, Vec<crate::parse_state::RightData>, Vec<crate::parse_state::UpData>) {
        let (inner, right_data, up_data) = self.value.parser(right_data);
        (SymbolParser { inner, symbol_value: self.value.clone() }, right_data, up_data)
    }
}

impl<T> ParserTrait for SymbolParser<T> where T: CombinatorTrait
{
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>) {
        self.inner.step(c)
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a dyn ParserTrait> + 'a> {
        Box::new(std::iter::once(&self.inner as &dyn ParserTrait))
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut dyn ParserTrait> + 'a> {
        Box::new(std::iter::once(&mut self.inner as &mut dyn ParserTrait))
    }
}

impl<T> IntoCombinator for &Symbol<T> where T: CombinatorTrait {
    type Output = Symbol<T>;
    fn into_combinator(self) -> Self::Output {
        self.clone()
    }
}

pub fn symbol<T>(value: T) -> Symbol<T>
where
    T: CombinatorTrait,
{
    Symbol { value: Rc::new(value) }
}