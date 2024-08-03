use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use crate::*;

#[derive(Clone)]
pub struct Deferred {
    pub(crate) f: Rc<dyn Fn() -> Combinator>,
}

impl Hash for Deferred {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.f.as_ref() as *const dyn Fn() -> Combinator, state);
    }
}

impl PartialEq for Deferred {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.f, &other.f)
    }
}

impl Eq for Deferred {}

impl Debug for Deferred {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Deferred")
    }
}

impl CombinatorTrait for Deferred {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let a = self.f.as_ref()();
        a.parse(right_data, bytes)
    }
}

pub fn deferred(f: impl Fn() -> Combinator + 'static) -> Combinator {
    Deferred { f: Rc::new(f) }.into()
}

impl From<&'static dyn Fn() -> Combinator> for Combinator {
    fn from(value: &'static dyn Fn() -> Combinator) -> Self {
        deferred(value)
    }
}

impl From<Deferred> for Combinator {
    fn from(value: Deferred) -> Self {
        Combinator::Deferred(value)
    }
}