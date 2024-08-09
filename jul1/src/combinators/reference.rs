use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Weak;
use crate::*;

#[derive(Clone, Debug)]
pub struct WeakRef {
    pub inner: Weak<RefCell<Option<Combinator>>>,
}

#[derive(Clone, Debug)]
pub struct StrongRef {
    pub inner: Rc<RefCell<Option<Combinator>>>,
}

impl PartialEq for WeakRef {
    fn eq(&self, other: &Self) -> bool {
        self.inner.ptr_eq(&other.inner)
    }
}

impl Eq for WeakRef {}

impl Hash for WeakRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(&self.inner, state);
    }
}

impl PartialEq for StrongRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for StrongRef {}

impl Hash for StrongRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(&self.inner, state);
    }
}

impl CombinatorTrait for WeakRef {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        self.inner.upgrade().unwrap().borrow().as_ref().unwrap().parse(right_data, bytes)
    }
}

impl CombinatorTrait for StrongRef {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        self.inner.borrow().as_ref().unwrap().parse(right_data, bytes)
    }
}

pub fn strong_ref() -> StrongRef {
    StrongRef {
        inner: Rc::new(RefCell::new(None))
    }
}

impl StrongRef {
    pub fn set(&self, inner: Combinator) {
        *self.inner.borrow_mut() = Some(inner);
    }

    pub fn downgrade(&self) -> WeakRef {
        WeakRef {
            inner: Rc::downgrade(&self.inner)
        }
    }
}

impl WeakRef {
    pub fn upgrade(&self) -> Option<StrongRef> {
        self.inner.upgrade().map(|inner| StrongRef {
            inner
        })
    }
}

impl From<WeakRef> for Combinator {
    fn from(weak_ref: WeakRef) -> Self {
        Combinator::WeakRef(weak_ref)
    }
}

impl From<StrongRef> for Combinator {
    fn from(strong_ref: StrongRef) -> Self {
        Combinator::StrongRef(strong_ref)
    }
}

impl From<&StrongRef> for Combinator {
    fn from(strong_ref: &StrongRef) -> Self {
        Combinator::StrongRef(strong_ref.clone())
    }
}