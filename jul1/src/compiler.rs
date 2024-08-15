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
        let mut deferred_cache: HashMap<Deferred, Ref> = HashMap::new();
        fn compile_inner(combinator: &Combinator, deferred_cache: &mut HashMap<Deferred, Ref>) {
            match combinator {
                // Construct the deferred combinator
                Combinator::Deferred(inner) => {
                    let new_inner: DeferredInner = match inner.inner.borrow().deref() {
                        DeferredInner::Uncompiled(f) => {
                            if let Some(cached) = deferred_cache.get(&inner) {
                                // *inner.inner.borrow_mut().deref_mut() = cached.clone().into();
                                cached.clone().into()
                            } else {
                                let strong = strong_ref();
                                let weak = strong.downgrade();
                                deferred_cache.insert(inner.clone(), Ref::Weak(weak.clone()));
                                // let mut lazy = inner.inner.as_ref();
                                // let mut evaluated: Combinator = (**lazy).clone();
                                let mut evaluated: Combinator = f();
                                compile_inner(&mut evaluated, deferred_cache);
                                strong.set(evaluated);
                                deferred_cache.insert(inner.clone(), Ref::Strong(strong.clone()));
                                DeferredInner::CompiledStrong(strong.clone())
                            }
                        }
                        DeferredInner::CompiledStrong(strong) => {
                            let inner: &Combinator = strong.inner.get().unwrap();
                            compile_inner(inner, deferred_cache);
                            DeferredInner::CompiledStrong(strong.clone())
                        }
                        DeferredInner::CompiledWeak(weak) => {
                            let binding = weak.inner.upgrade().unwrap();
                            let inner: &Combinator = binding.get().unwrap();
                            compile_inner(inner, deferred_cache);
                            DeferredInner::CompiledWeak(weak.clone())
                        }
                    };
                    *inner.inner.borrow_mut() = new_inner;
                }
                _ => {
                    combinator.apply(&mut |combinator| {
                        compile_inner(combinator, deferred_cache);
                    });
                }
            }
        }
        compile_inner(&mut self, &mut deferred_cache);
        self
    }
}