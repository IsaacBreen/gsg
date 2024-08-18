use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::OnceCell;
use std::collections::HashMap;
use crate::*;

#[derive(Clone)]
pub struct Deferred {
    pub(crate) inner: OnceCell<DeferredInner>,
    pub(crate) f: Rc<dyn Fn() -> Combinator>,
}

impl Debug for Deferred {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Deferred")
            .field("inner", &self.inner)
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Debug)]
pub enum DeferredInner {
    CompiledStrong(StrongRef),
    CompiledWeak(WeakRef),
}

impl PartialEq for Deferred {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.f, &other.f)
    }
}

impl Eq for Deferred {}

impl Hash for Deferred {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(&*self.f, state);
    }
}

impl CombinatorTrait for Deferred {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        if let Some(inner) = self.inner.get() {
            match inner {
                DeferredInner::CompiledStrong(inner) => f(inner),
                DeferredInner::CompiledWeak(inner) => todo!(),
            }
        }
    }

    fn parse<'a>(&'a self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'a>, ParseResults) {
        let inner = self.inner.get_or_init(|| {
            let combinator = (self.f)();
            let strong = strong_ref();
            strong.set(combinator);
            DeferredInner::CompiledStrong(strong)
        });

        match inner {
            DeferredInner::CompiledStrong(combinator) => combinator.parse(right_data, bytes),
            DeferredInner::CompiledWeak(combinator) => combinator.parse(right_data, bytes),
        }
    }
}

pub fn deferred<T: CombinatorTrait + 'static>(f: fn() -> T) -> Deferred {
    Deferred {
        inner: OnceCell::new(),
        f: Rc::new(move || Box::new(f()) as Combinator),
    }
}

pub fn deferred2(f: fn() -> Choice2<Seq2<EatU8, Deferred>, EatU8>) -> Deferred {
    Deferred {
        inner: OnceCell::new(),
        f: Rc::new(move || Box::new(f()) as Combinator),
    }
}