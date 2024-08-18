use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use crate::*;
use once_cell::unsync::OnceCell;

macro_rules! profile {
    ($name:expr, $e:expr) => {
        $e
    };
}

thread_local! {
    static COMBINATOR_CACHE: RefCell<HashMap<usize, Rc<Combinator>>> = RefCell::new(HashMap::new());
}

#[derive(Clone, Debug)]
pub struct Deferred {
    pub(crate) inner: OnceCell<DeferredInner>,
    pub(crate) deferred_fn: Rc<dyn EvaluateDeferredFnToBoxedDynCombinator>,
}

#[derive(Clone, Copy)]
pub struct DeferredFn<T: CombinatorTrait + 'static, F: Fn() -> T>(pub F, pub usize);

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

pub trait EvaluateDeferredFnToBoxedDynCombinator: Debug {
    fn evaluate_deferred_fn_to_combinator(&self) -> Combinator;
    fn get_addr(&self) -> usize;
}

impl<T: CombinatorTrait + 'static, F: Fn() -> T> EvaluateDeferredFnToBoxedDynCombinator for DeferredFn<T, F> {
    fn evaluate_deferred_fn_to_combinator(&self) -> Combinator {
        Box::new(self.0())
    }
    fn get_addr(&self) -> usize {
        self.1
    }
}

#[derive(Clone, Debug)]
pub enum DeferredInner {
    CompiledStrong(StrongRef),
    CompiledWeak(WeakRef),
}

impl PartialEq for Deferred {
    fn eq(&self, other: &Self) -> bool {
        self.inner.get().eq(&other.inner.get())
    }
}

impl Eq for Deferred {}

impl Hash for Deferred {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.get().hash(state)
    }
}

impl Hash for DeferredInner {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            DeferredInner::CompiledStrong(f) => std::ptr::hash(f, state),
            DeferredInner::CompiledWeak(f) => std::ptr::hash(f, state),
        }
    }
}

impl PartialEq for DeferredInner {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DeferredInner::CompiledStrong(f1), DeferredInner::CompiledStrong(f2)) => f1 == f2,
            (DeferredInner::CompiledWeak(f1), DeferredInner::CompiledWeak(f2)) => f1 == f2,
            _ => false,
        }
    }
}

impl Eq for DeferredInner {}

impl CombinatorTrait for Deferred {
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

pub fn deferred<T: CombinatorTrait + 'static>(f: fn() -> T) -> Deferred {
    let addr = f as *const () as usize;
    Deferred {
        inner: OnceCell::new(),
        deferred_fn: Rc::new(DeferredFn(f, addr)),
    }
}

pub fn deferred2(f: fn() -> Choice2<Seq2<EatU8, Deferred>, EatU8>) -> Deferred {
    let addr = f as *const () as usize;
    Deferred {
        inner: OnceCell::new(),
        deferred_fn: Rc::new(DeferredFn(f, addr)),
    }
}
