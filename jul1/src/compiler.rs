use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use crate::*;

#[derive(Clone)]
enum Ref {
    Strong(StrongRef),
    Weak(WeakRef),
}

impl From<Ref> for DeferredInner {
    fn from(value: Ref) -> Self {
        match value {
            Ref::Strong(strong) => DeferredInner::CompiledStrong(strong),
            Ref::Weak(weak) => DeferredInner::CompiledWeak(weak),
        }
    }
}

pub trait Compile {
    fn compile(self) -> Self;
}

impl<T: CombinatorTrait> Compile for T {
    fn compile(mut self) -> Self {
        let mut deferred_cache: HashMap<usize, Ref> = HashMap::new();
        fn compile_inner(combinator: &dyn CombinatorTrait, deferred_cache: &mut HashMap<usize, Ref>) {
            if let Some(deferred) = combinator.as_any().downcast_ref::<Deferred>() {
                let new_inner: DeferredInner = match deferred.inner.get() {
                    Some(DeferredInner::CompiledStrong(strong)) => {
                        DeferredInner::CompiledStrong(strong.clone())
                    }
                    Some(DeferredInner::CompiledWeak(weak)) => {
                        DeferredInner::CompiledWeak(weak.clone())
                    }
                    None => {
                        let strong = strong_ref();
                        let weak = strong.downgrade();
                        deferred_cache.insert(deferred.deferred_fn.get_addr(), Ref::Weak(weak.clone()));
                        let mut evaluated: Combinator = deferred.deferred_fn.evaluate_deferred_fn_to_combinator();
                        compile_inner(&mut evaluated, deferred_cache);
                        strong.set(evaluated);
                        deferred_cache.insert(deferred.deferred_fn.get_addr(), Ref::Strong(strong.clone()));
                        DeferredInner::CompiledStrong(strong.clone())
                    }
                };
                deferred.inner.set(new_inner).ok().expect("Cannot set value more than once");
            } else {
                combinator.apply(&mut |combinator| {
                    compile_inner(combinator, deferred_cache);
                });
            }
        }
        compile_inner(&mut self, &mut deferred_cache);
        self
    }
}
