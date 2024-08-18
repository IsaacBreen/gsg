// src/combinators/deferred.rs
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use crate::*;
use once_cell::unsync::OnceCell;
use crate::compiler::DeferredCompiler;

#[derive(Clone, Debug)]
pub struct Deferred<T: CombinatorTrait + 'static> {
    inner: OnceCell<DeferredInner<T>>,
    deferred_fn: Rc<dyn DeferredFnTrait<T>>,
}

// Made non-public
#[derive(Clone, Copy)]
struct DeferredFn<T: CombinatorTrait + 'static, F: Fn() -> T>(pub F, pub usize);

impl<T: CombinatorTrait + 'static, F: Fn() -> T> Debug for DeferredFn<T, F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeferredFn").finish_non_exhaustive()
    }
}

impl<T: CombinatorTrait + 'static, F: Fn() -> T> PartialEq for DeferredFn<T, F> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&self.0, &other.0)
    }
}

impl<T: CombinatorTrait + 'static, F: Fn() -> T> Eq for DeferredFn<T, F> {}

impl<T: CombinatorTrait + 'static, F: Fn() -> T> Hash for DeferredFn<T, F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(&self.0, state);
    }
}

// Trait for evaluating the deferred function
pub trait DeferredFnTrait<T: CombinatorTrait + 'static>: Debug {
    fn evaluate_to_combinator(&self) -> T;
    fn get_addr(&self) -> usize;
}

impl<T: CombinatorTrait + 'static, F: Fn() -> T> DeferredFnTrait<T> for DeferredFn<T, F> {
    fn evaluate_to_combinator(&self) -> T {
        (self.0)()
    }
    fn get_addr(&self) -> usize {
        self.1
    }
}

// Made non-public
#[derive(Clone, Debug)]
pub enum DeferredInner<T> {
    CompiledStrong(StrongRef<T>),
    CompiledWeak(WeakRef<T>),
}

impl<T: CombinatorTrait> PartialEq for Deferred<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.get().eq(&other.inner.get())
    }
}

impl<T: CombinatorTrait> Eq for Deferred<T> {}

impl<T: CombinatorTrait> Hash for Deferred<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.get().hash(state)
    }
}

impl<T> Hash for DeferredInner<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            DeferredInner::CompiledStrong(f) => std::ptr::hash(f, state),
            DeferredInner::CompiledWeak(f) => std::ptr::hash(f, state),
        }
    }
}

impl<T> PartialEq for DeferredInner<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DeferredInner::CompiledStrong(f1), DeferredInner::CompiledStrong(f2)) => f1 == f2,
            (DeferredInner::CompiledWeak(f1), DeferredInner::CompiledWeak(f2)) => f1 == f2,
            _ => false,
        }
    }
}

impl<T> Eq for DeferredInner<T> {}

impl<T: CombinatorTrait + 'static> CombinatorTrait for Deferred<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        match self.inner.get() {
            Some(DeferredInner::CompiledStrong(inner)) => {
                f(inner);
            }
            Some(DeferredInner::CompiledWeak(inner)) => {
                f(inner);
            }
            None => {
                panic!("Deferred combinator should not be used directly. Use Deferred() function instead.");
            }
        }
    }

    fn parse<'a>(&'a self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'a>, ParseResults) {
        match self.inner.get() {
            Some(DeferredInner::CompiledStrong(combinator)) => combinator.parse(right_data, bytes),
            Some(DeferredInner::CompiledWeak(combinator)) => combinator.parse(right_data, bytes),
            None => {
                panic!("Deferred combinator should not be used directly. Use Deferred() function instead.");
            }
        }
    }
}

// Public function for creating a Deferred combinator
pub fn deferred<T: CombinatorTrait + 'static>(f: fn() -> T) -> Deferred<T> {
    let addr = f as *const () as usize;
    Deferred {
        inner: OnceCell::new(),
        deferred_fn: Rc::new(DeferredFn(f, addr)),
    }
}

impl<T: CombinatorTrait + 'static> DeferredCompiler for Deferred<T> {
    fn get_deferred_addr(&self) -> usize {
        self.deferred_fn.get_addr()
    }

    fn evaluate_to_combinator(&self) -> Combinator {
        Box::new(self.deferred_fn.evaluate_to_combinator())
    }
}

impl<T: CombinatorTrait + 'static> Deferred<T> {
    // Helper function to check if the combinator is compiled
    pub(crate) fn is_compiled(&self) -> bool {
        self.inner.get().is_some()
    }

    // Helper function to set the inner combinator
    pub(crate) fn set_inner(&self, inner: DeferredInner<T>) {
        self.inner.set(inner).ok().expect("Cannot set inner value more than once");
    }
}