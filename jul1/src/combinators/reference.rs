use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};
use once_cell::unsync::OnceCell;
use crate::*;

#[derive(Debug, Clone)]
pub struct WeakRef {
    pub inner: Weak<OnceCell<Combinator>>,
}

#[derive(Debug, Clone)]
pub struct StrongRef {
    pub inner: Rc<OnceCell<Combinator>>,
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(self.inner.upgrade().unwrap().get().unwrap());
    }

    fn parse<'a>(&'a self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'a>, ParseResults) {
        self.get().unwrap().parse(right_data, bytes)
    }
}

impl CombinatorTrait for StrongRef {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(self.inner.get().unwrap());
    }

    fn parse<'a>(&'a self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'a>, ParseResults) {
        self.inner
            .get()
            .unwrap()
            .parse(right_data, bytes)
    }
}

pub fn strong_ref() -> StrongRef {
    StrongRef {
        inner: Rc::new(OnceCell::new())
    }
}

impl StrongRef {
    pub fn set(&self, inner: impl CombinatorTrait + 'static) {
        self.inner.set(Box::new(inner)).ok().expect("Cannot set value more than once");
    }

    pub fn downgrade(&self) -> WeakRef {
        WeakRef {
            inner: Rc::downgrade(&self.inner)
        }
    }
}

impl WeakRef {
    pub fn upgrade(&self) -> Option<StrongRef> {
        self.inner.upgrade().map(|inner| StrongRef { inner })
    }

    pub fn get(&self) -> Option<&Combinator> {
        // Upgrade the weak reference to a strong reference
        let strong_ref = self.inner.upgrade()?;

        // Safely access the inner Combinator
        // Note: We use unsafe code here to transmute the lifetime.
        // This is safe because the OnceCell guarantees that the value,
        // once set, will live as long as the Rc/Weak, which is 'static.
        unsafe {
            let combinator: &Combinator = std::mem::transmute(strong_ref.get().unwrap());
            Some(combinator)
        }
    }
}

// impl From<WeakRef> for Combinator {
//     fn from(weak_ref: WeakRef) -> Self {
//         Combinator::WeakRef(weak_ref)
//     }
// }
//
// impl From<StrongRef> for Combinator {
//     fn from(strong_ref: StrongRef) -> Self {
//         Combinator::StrongRef(strong_ref)
//     }
// }
//
// impl From<&StrongRef> for Combinator {
//     fn from(strong_ref: &StrongRef) -> Self {
//         Combinator::StrongRef(strong_ref.clone())
//     }
// }
