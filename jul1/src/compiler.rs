use std::collections::HashSet;
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

impl Combinator {
    pub fn compile(mut self) -> Combinator {
        let mut deferred_cache: HashMap<DeferredFn, Ref> = HashMap::new();
        fn compile_inner(combinator: &dyn CombinatorTrait, deferred_cache: &mut HashMap<DeferredFn, Ref>) {
            if let Some(Deferred { inner }) = combinator.as_any().downcast_ref::<Deferred>() {
                let new_inner: DeferredInner = match inner.borrow().deref() {
                    DeferredInner::Uncompiled(f) => {
                        if let Some(cached) = deferred_cache.get(f) {
                            cached.clone().into()
                        } else {
                            let strong = strong_ref();
                            let weak = strong.downgrade();
                            deferred_cache.insert(*f, Ref::Weak(weak.clone()));
                            let mut evaluated: Combinator = (f.0)();
                            compile_inner(&mut evaluated, deferred_cache);
                            strong.set(evaluated);
                            deferred_cache.insert(*f, Ref::Strong(strong.clone()));
                            DeferredInner::CompiledStrong(strong.clone())
                        }
                    }
                    DeferredInner::CompiledStrong(strong) => {
                        DeferredInner::CompiledStrong(strong.clone())
                    }
                    DeferredInner::CompiledWeak(weak) => {
                        DeferredInner::CompiledWeak(weak.clone())
                    }
                };
                *inner.borrow_mut() = new_inner;
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