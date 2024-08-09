use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, Symbol};
use crate::parse_state::RightData;
#[derive(Debug, Clone)]
pub struct ForwardRef<'a> {
    pub(crate) a: Rc<RefCell<Option<Rc<Combinator<'a>>>>>,
}

impl Hash for ForwardRef<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.a).hash(state);
    }
}

impl PartialEq for ForwardRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.a, &other.a)
    }
}

impl Eq for ForwardRef<'_> {}

impl<'a> CombinatorTrait<'a> for ForwardRef<'a> {
    fn parse(&'a self, right_data: RightData, bytes: &[u8]) -> (Parser<'a>, ParseResults) {
        // self.a.try_borrow().expect("ForwardRef.parse: a is borrowed").as_ref().unwrap().parse(right_data, bytes)
        todo!()
    }
}

impl CombinatorTrait<'_> for RefCell<Option<Rc<Combinator<'_>>>> {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        // self.borrow().as_ref().unwrap().parse(right_data, bytes)
        todo!()
    }
}

impl ForwardRef<'_> {
    pub fn set(&mut self, a: impl Into<Combinator<'static>>) -> Symbol<'static> {
        // let a: Rc<Combinator> = a.into().into();
        // *self.a.borrow_mut() = Some(a.clone());
        // Symbol { value: a }
        todo!()
    }
}


pub fn forward_ref<'a>() -> ForwardRef<'a> {
    ForwardRef { a: Rc::new(RefCell::new(None)) }
}

impl From<&ForwardRef<'_>> for Combinator<'_> {
    fn from(value: &ForwardRef) -> Self {
        // Combinator::ForwardRef(value.clone())
        todo!()
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