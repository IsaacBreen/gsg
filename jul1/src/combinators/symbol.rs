use std::rc::Rc;
use crate::{CombinatorTrait, IntoCombinator};

pub struct Symbol<T> {
    pub value: Rc<T>,
}

impl<T> Clone for Symbol<T> {
    fn clone(&self) -> Self {
        Symbol { value: self.value.clone() }
    }
}

impl<T> CombinatorTrait for Symbol<T> where T: CombinatorTrait {
    type Parser = T::Parser;
    fn parser(&self, right_data: crate::parse_state::RightData) -> (Self::Parser, Vec<crate::parse_state::RightData>, Vec<crate::parse_state::UpData>) {
        self.value.parser(right_data)
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