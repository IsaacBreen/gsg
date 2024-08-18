// src/combinators/deferred.rs
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::*;
use once_cell::unsync::OnceCell;

thread_local! {
    static DEFERRED_CACHE: RefCell<HashMap<usize, Box<dyn std::any::Any>>> = RefCell::new(HashMap::new());
}

#[derive(Clone, Debug)]
pub struct Deferred<T: CombinatorTrait + Clone + 'static> {
    deferred_fn: Rc<dyn DeferredFnTrait<T>>,
    inner: OnceCell<T>, // Added inner field
}

// Made non-public
#[derive(Clone, Copy)]
struct DeferredFn<T: CombinatorTrait + Clone + 'static, F: Fn() -> T>(pub F, pub usize);

impl<T: CombinatorTrait + Clone + 'static, F: Fn() -> T> Debug for DeferredFn<T, F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeferredFn").finish_non_exhaustive()
    }
}

impl<T: CombinatorTrait + Clone + 'static, F: Fn() -> T> PartialEq for DeferredFn<T, F> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&self.0, &other.0)
    }
}

impl<T: CombinatorTrait + Clone + 'static, F: Fn() -> T> Eq for DeferredFn<T, F> {}

impl<T: CombinatorTrait + Clone + 'static, F: Fn() -> T> Hash for DeferredFn<T, F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(&self.0, state);
    }
}

// Trait for evaluating the deferred function
pub trait DeferredFnTrait<T: CombinatorTrait + Clone + 'static>: Debug {
    fn evaluate_to_combinator(&self) -> T;
    fn get_addr(&self) -> usize;
}

impl<T: CombinatorTrait + Clone + 'static, F: Fn() -> T> DeferredFnTrait<T> for DeferredFn<T, F> {
    fn evaluate_to_combinator(&self) -> T {
        DEFERRED_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if let Some(value) = cache.get(&self.1) {
                value.downcast_ref::<T>().unwrap().clone()
            } else {
                let value = (self.0)();
                cache.insert(self.1, Box::new(value.clone()));
                value
            }
        })
    }
    fn get_addr(&self) -> usize {
        self.1
    }
}

impl<T: CombinatorTrait + Clone> PartialEq for Deferred<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.deferred_fn, &other.deferred_fn)
    }
}

impl<T: CombinatorTrait + Clone> Eq for Deferred<T> {}

impl<T: CombinatorTrait + Clone> Hash for Deferred<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(&self.deferred_fn, state);
    }
}

impl<T: CombinatorTrait + Clone + 'static> CombinatorTrait for Deferred<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(self.inner.get_or_init(|| self.deferred_fn.evaluate_to_combinator()))
    }

    fn apply_mut(&mut self, f: &mut dyn FnMut(&mut dyn CombinatorTrait)) {
        f(self.inner.get_mut().unwrap())
    }

    fn parse<'a>(&'a self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'a>, ParseResults) {
        let combinator = self.inner.get_or_init(|| self.deferred_fn.evaluate_to_combinator());
        combinator.parse(right_data, bytes)
    }

    fn compile_mut(&mut self) {
        // Force evaluation and compilation of the inner combinator
        let _ = self.inner.get_or_init(|| {
            let mut combinator = self.deferred_fn.evaluate_to_combinator();
            combinator.compile_mut();
            combinator
        });
    }
}

// Public function for creating a Deferred combinator
pub fn deferred<T: CombinatorTrait + 'static>(f: fn() -> T) -> Deferred<StrongRef<T>> {
    let addr = f as *const () as usize;
    let f = move || StrongRef::new(f());
    Deferred {
        deferred_fn: Rc::new(DeferredFn(f, addr)),
        inner: OnceCell::new(), // Initialize inner
    }
}