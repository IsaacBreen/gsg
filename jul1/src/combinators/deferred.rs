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
pub struct Deferred<T: CombinatorTrait, F: Fn() -> T> {
    pub(crate) inner: RefCell<DeferredInner<T, F>>,
}

#[derive(Clone, Copy)]
pub struct DeferredFn<T: CombinatorTrait, F: Fn() -> T>(pub F);

impl<T: CombinatorTrait, F: Fn() -> T> PartialEq for DeferredFn<T, F> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl<T: CombinatorTrait, F: Fn() -> T> Eq for DeferredFn<T, F> {}

impl<T: CombinatorTrait, F: Fn() -> T> Hash for DeferredFn<T, F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.0, state);
    }
}

#[derive(Clone)]
pub enum DeferredInner<T: CombinatorTrait, F: Fn() -> T> {
    Uncompiled(DeferredFn<T, F>),
    CompiledStrong(StrongRef),
    CompiledWeak(WeakRef),
}

impl<T: CombinatorTrait, F: Fn() -> T> PartialEq for Deferred<T, F> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.borrow().eq(&other.inner.borrow())
    }
}

impl<T: CombinatorTrait, F: Fn() -> T> Eq for Deferred<T, F> {}

impl<T: CombinatorTrait, F: Fn() -> T> Hash for Deferred<T, F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.borrow().hash(state)
    }
}

impl<T: CombinatorTrait, F: Fn() -> T> Hash for DeferredInner<T, F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            DeferredInner::Uncompiled(f) => std::ptr::hash(f, state),
            DeferredInner::CompiledStrong(f) => std::ptr::hash(f, state),
            DeferredInner::CompiledWeak(f) => std::ptr::hash(f, state),
        }
    }
}

impl<T: CombinatorTrait, F: Fn() -> T> PartialEq for DeferredInner<T, F> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DeferredInner::Uncompiled(f1), DeferredInner::Uncompiled(f2)) => std::ptr::eq(f1, f2),
            (DeferredInner::CompiledStrong(f1), DeferredInner::CompiledStrong(f2)) => f1 == f2,
            (DeferredInner::CompiledWeak(f1), DeferredInner::CompiledWeak(f2)) => f1 == f2,
            _ => false,
        }
    }
}

impl<T: CombinatorTrait, F: Fn() -> T> Eq for DeferredInner<T, F> {}

impl<T: CombinatorTrait, F: Fn() -> T> Debug for Deferred<T, F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Deferred").finish_non_exhaustive()
    }
}

impl<T: CombinatorTrait, F: Fn() -> T> CombinatorTrait for Deferred<T, F> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        match self.inner.borrow().clone() {
            DeferredInner::Uncompiled(f) => {
                // todo: better error message (this one makes no sense)
                panic!("DeferredInner combinator should not be used directly. Use DeferredInner() function instead.");
            }
            DeferredInner::CompiledStrong(inner) => {
                f(&inner);
            }
            DeferredInner::CompiledWeak(inner) => {
                f(&inner);
            }
        }
    }

    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        match self.inner.borrow().clone() {
            DeferredInner::Uncompiled(f) => {
                panic!("DeferredInner combinator should not be used directly. Use DeferredInner() function instead.");
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

pub fn deferred(f: &'static impl Fn() -> Combinator) -> impl CombinatorTrait {
    Deferred { inner: RefCell::new(DeferredInner::Uncompiled(DeferredFn(f))) }
}

// impl<T> From<&'static T> for Combinator
// where
//     T: Fn()-> Combinator
// {
//     fn from(value: &'static T) -> Self {
//         deferred(value).into()
//     }
// }

// impl From<Deferred> for Combinator {
//     fn from(value: Deferred) -> Self {
//         Combinator::Deferred(value).into()
//     }
// }
