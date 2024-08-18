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
    pub(crate) inner: OnceCell<StrongRef>,
    pub(crate) f: Rc<dyn EvaluateDeferredFnToBoxedDynCombinator>,
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
        // dbg!(self.1)
        self.1
    }
}

impl PartialEq for Deferred {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&self.f, &other.f)
    }
}

impl Eq for Deferred {}

impl Hash for Deferred {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(&self.f, state)
    }
}

impl CombinatorTrait for Deferred {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        let strong_ref = self.inner.get_or_init(|| {
            let combinator = self.f.evaluate_deferred_fn_to_combinator();
            let strong_ref = strong_ref();
            strong_ref.set(combinator);
            strong_ref
        });
        f(strong_ref);
    }

    fn parse<'a>(&'a self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'a>, ParseResults) {
        let strong_ref = self.inner.get_or_init(|| {
            let combinator = self.f.evaluate_deferred_fn_to_combinator();
            let strong_ref = strong_ref();
            strong_ref.set(combinator);
            strong_ref
        });
        strong_ref.parse(right_data, bytes)
    }
}

pub fn deferred<T: CombinatorTrait + 'static>(f: fn() -> T) -> Deferred {
    let addr = f as *const () as usize;
    Deferred {
        inner: OnceCell::new(),
        f: Rc::new(DeferredFn(f, addr))
    }
}

pub fn deferred2(f: fn() -> Choice2<Seq2<EatU8, Deferred>, EatU8>) -> Deferred {
    let addr = std::ptr::addr_of!(f) as usize;
    let addr = f as *const () as usize;
    dbg!(addr);
    dbg!(f as *const ());
    Deferred {
        inner: OnceCell::new(),
        f: Rc::new(DeferredFn(f, addr))
    }
}