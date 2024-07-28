use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, Stats};
use crate::parse_state::RightData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardRef {
    a: Rc<RefCell<Option<Rc<Combinator>>>>,
}

impl Hash for ForwardRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.borrow().as_ref().unwrap().hash(state);
    }
}

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

pub fn forward_ref() -> ForwardRef {
    ForwardRef { a: Rc::new(RefCell::new(None)) }
}

impl ForwardRef {
    pub fn set(&mut self, a: Combinator) -> Rc<Combinator> {
        let a = Rc::new(a);
        *self.a.borrow_mut() = Some(a.clone());
        a
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