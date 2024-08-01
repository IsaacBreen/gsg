use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, Symbol};
use crate::parse_state::RightData;
#[derive(Debug, Clone)]
pub struct ForwardRef {
    a: Rc<RefCell<Option<Rc<Combinator>>>>,
}

impl Hash for ForwardRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.a).hash(state);
    }
}

impl PartialEq for ForwardRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.a, &other.a)
    }
}

impl Eq for ForwardRef {}

impl CombinatorTrait for ForwardRef {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        self.a.borrow().as_ref().unwrap().parser(right_data)
    }
}

impl CombinatorTrait for RefCell<Option<Rc<Combinator>>> {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        self.borrow().as_ref().unwrap().parser(right_data)
    }
}

impl ForwardRef {
    pub fn set(&mut self, a: impl Into<Combinator>) -> Symbol {
        let a: Rc<Combinator> = a.into().into();
        *self.a.borrow_mut() = Some(a.clone());
        Symbol { value: a }
    }
}


pub fn forward_ref() -> ForwardRef {
    ForwardRef { a: Rc::new(RefCell::new(None)) }
}

impl From<&ForwardRef> for Combinator {
    fn from(value: &ForwardRef) -> Self {
        Combinator::ForwardRef(value.clone())
    }
}

#[macro_export]
macro_rules! forward_decls {
    ($($name:ident),* $(,)?) => {
        $(
            let mut $name = forward_ref();
        )*
    };
}