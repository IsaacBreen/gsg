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
    static COMBINATOR_CACHE: RefCell<HashMap<usize, Rc<Combinator>>> = RefCell::new(HashMap::new());
}

#[derive(Clone)]
pub struct Deferred {
    pub(crate) inner: RefCell<DeferredInner>,
}

#[derive(Clone, Copy)]
pub struct DeferredFn<T: CombinatorTrait + 'static, F: Fn() -> T>(pub F, pub usize);

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

pub trait EvaluateDeferredFnToBoxedDynCombinator {
    fn evaluate_deferred_fn_to_combinator(&self) -> Combinator;
    fn get_addr(&self) -> usize;
}

impl<T: CombinatorTrait + 'static, F: Fn() -> T> EvaluateDeferredFnToBoxedDynCombinator for DeferredFn<T, F> {
    fn evaluate_deferred_fn_to_combinator(&self) -> Combinator {
        Box::new(self.0())
    }
    fn get_addr(&self) -> usize {
        // dbg!(self.1)
        self.1
    }
}

#[derive(Clone)]
pub enum DeferredInner {
    Uncompiled(Rc<dyn EvaluateDeferredFnToBoxedDynCombinator>),
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

    fn parse<'a, 'b>(&'b self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'a>, ParseResults) where Self: 'a, 'b: 'a {
        match self.inner.borrow().clone() {
            DeferredInner::Uncompiled(f) => {
                panic!("DeferredInner combinator should not be used directly. Use DeferredInner() function instead.");
                // let combinator = profile!("DeferredInner cache check", {
                //         COMBINATOR_CACHE.with(|cache| {
                //         let mut cache = cache.borrow_mut();
                //         cache.entry(f.clone())
                //             .or_insert_with(|| profile!("DeferredInner init", Rc::new((f.0)())))
                //             .clone()
                //     })
                // });
                // combinator.parse(right_data, bytes)
            }
            DeferredInner::CompiledStrong(combinator) => combinator.parse(right_data, bytes),
            DeferredInner::CompiledWeak(combinator) => combinator.parse(right_data, bytes),
        }
    }
}

// pub fn deferred<T: CombinatorTrait + 'static>(f: impl Fn() -> T + 'static) -> Deferred {
//     // dbg!(f as *const ());
//     // let addr = f as *const () as usize;
//     let addr = std::ptr::addr_of!(f) as usize;
//     dbg!(addr);
//     Deferred { inner: RefCell::new(DeferredInner::Uncompiled(Rc::new(DeferredFn(f, addr)))) }
// }

pub fn deferred<T: CombinatorTrait + 'static>(f: fn() -> T) -> Deferred {
    let addr = f as *const () as usize;
    // dbg!(std::ptr::addr_of!(f) as usize);
    // dbg!(f as *const () as usize);
    Deferred { inner: RefCell::new(DeferredInner::Uncompiled(Rc::new(DeferredFn(f, addr)))) }
}

pub fn deferred2(f: fn() -> Choice2<Seq2<EatU8, Deferred>, EatU8>) -> Deferred {
    let addr = std::ptr::addr_of!(f) as usize;
    let addr = f as *const () as usize;
    dbg!(addr);
    dbg!(f as *const ());
    Deferred { inner: RefCell::new(DeferredInner::Uncompiled(Rc::new(DeferredFn(f, addr)))) }
}
