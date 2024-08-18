use crate::*;
use std::collections::HashMap;
use std::hash::Hash;

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
                let f_ptr = deferred.f.as_ref() as *const _ as *const () as usize;
                if let Some(cached) = deferred_cache.get(&f_ptr) {
                    deferred.inner.set(cached.clone().into()).ok();
                } else {
                    let strong = strong_ref();
                    let weak = strong.downgrade();
                    deferred_cache.insert(f_ptr, Ref::Weak(weak.clone()));
                    let mut evaluated: Combinator = (deferred.f)();
                    compile_inner(&mut evaluated, deferred_cache);
                    strong.set(evaluated);
                    deferred_cache.insert(f_ptr, Ref::Strong(strong.clone()));
                    deferred.inner.set(DeferredInner::CompiledStrong(strong)).ok();
                }
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