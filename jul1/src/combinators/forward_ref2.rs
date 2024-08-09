use std::cell::RefCell;
use crate::*;

#[derive(Debug, Clone)]
pub struct ForwardRef2<'a> {
    b: Rc<RefCell<Option<&'a Combinator>>>,
}

impl<'a> ForwardRef2<'a> {
    pub fn set(&mut self, a: impl Into<Combinator>) -> Symbol {
        let a: Rc<Combinator> = a.into().into();
        *self.b.borrow_mut() = Some(&a);
        Symbol { value: a }
    }
}

impl<'a> CombinatorTrait for ForwardRef2<'a> {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        self.b.borrow().as_ref().unwrap().parse(right_data, bytes)
    }
}

impl<'a> From<&'a ForwardRef2<'a>> for Combinator {
    fn from(value: &'a ForwardRef2<'a>) -> Self {
        Combinator::ForwardRef2(value.clone())
    }
}

pub fn forward_ref2<'a>() -> ForwardRef2<'a> {
    ForwardRef2 { b: Rc::new(RefCell::new(None)) }
}
