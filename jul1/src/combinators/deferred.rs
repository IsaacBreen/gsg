use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::*;

macro_rules! profile {
    ($name:expr, $e:expr) => {
        $e
    };
}

thread_local! {
    static COMBINATOR_CACHE: RefCell<HashMap<DeferredFn, Rc<Combinator>>> = RefCell::new(HashMap::new());
}

#[derive(Clone)]
pub struct Deferred {
    pub(crate) inner: RefCell<DeferredInner>,
}

#[derive(Clone, Copy)]
pub struct DeferredFn(pub &'static dyn Fn() -> Combinator);

impl PartialEq for DeferredFn {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl Eq for DeferredFn {}

impl Hash for DeferredFn {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.0, state);
    }
}

#[derive(Clone)]
pub enum DeferredInner {
    Uncompiled(DeferredFn),
    CompiledStrong(StrongRef),
    CompiledWeak(WeakRef),
}

impl PartialEq for Deferred {
    fn eq(&self, other: &Self) -> bool {
        self.inner.borrow().eq(&other.inner.borrow())
    }
}

impl Eq for Deferred {}

impl Hash for Deferred {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.borrow().hash(state)
    }
}

impl Hash for DeferredInner {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            DeferredInner::Uncompiled(f) => std::ptr::hash(f, state),
            DeferredInner::CompiledStrong(f) => std::ptr::hash(f, state),
            DeferredInner::CompiledWeak(f) => std::ptr::hash(f, state),
        }
    }
}

impl PartialEq for DeferredInner {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DeferredInner::Uncompiled(f1), DeferredInner::Uncompiled(f2)) => std::ptr::eq(f1, f2),
            (DeferredInner::CompiledStrong(f1), DeferredInner::CompiledStrong(f2)) => f1 == f2,
            (DeferredInner::CompiledWeak(f1), DeferredInner::CompiledWeak(f2)) => f1 == f2,
            _ => false,
        }
    }
}

impl Eq for DeferredInner {}

impl Debug for Deferred {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Deferred").finish_non_exhaustive()
    }
}

impl CombinatorTrait for Deferred {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        match self.inner.borrow().clone() {
            DeferredInner::Uncompiled(f) => {
                // panic!("DeferredInner combinator should not be used directly. Use DeferredInner() function instead.");
                let combinator = profile!("DeferredInner cache check", {
                        COMBINATOR_CACHE.with(|cache| {
                        let mut cache = cache.borrow_mut();
                        cache.entry(f.clone())
                            .or_insert_with(|| profile!("DeferredInner init", Rc::new((f.0)())))
                            .clone()
                    })
                });
                combinator.parse(right_data, bytes)
            }
            DeferredInner::CompiledStrong(combinator) => combinator.parse(right_data, bytes),
            DeferredInner::CompiledWeak(combinator) => combinator.parse(right_data, bytes),
        }
    }
}

pub fn deferred(f: &'static impl Fn() -> Combinator) -> Combinator {
    Deferred { inner: RefCell::new(DeferredInner::Uncompiled(DeferredFn(f))) }.into()
}

impl<T> From<&'static T> for Combinator
where
    T: Fn() -> Combinator
{
    fn from(value: &'static T) -> Self {
        deferred(value).into()
    }
}

impl From<Deferred> for Combinator {
    fn from(value: Deferred) -> Self {
        Combinator::Deferred(value).into()
    }
}
