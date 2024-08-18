use std::collections::HashMap;
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
                    match deferred.inner.get() {
                        Some(_) => {}
                        None => {
                            let addr = deferred.deferred_fn.get_addr();
                            let new_inner: DeferredInner = if let Some(cached) = deferred_cache.get(&addr) {
                                cached.clone().into()
                            } else {
                                let strong = strong_ref();
                                let weak = strong.downgrade();
                                deferred_cache.insert(addr, Ref::Weak(weak.clone()));
                                let mut evaluated: Combinator = deferred.deferred_fn.evaluate_deferred_fn_to_combinator();
                                compile_inner(&mut evaluated, deferred_cache);
                                strong.set(evaluated);
                                deferred_cache.insert(addr, Ref::Strong(strong.clone()));
                                DeferredInner::CompiledStrong(strong.clone())
                            };
                            deferred.inner.set(new_inner).unwrap();
                        }
                    };
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
