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
            if let Some(Deferred { inner }) = combinator.as_any().downcast_ref::<Deferred>() {
                let new_inner: DeferredInner = match inner.borrow().deref() {
                    DeferredInner::Uncompiled(f) => {
                        fn get_addr(f: &Rc<dyn EvaluateDeferredFnToBoxedDynCombinator>) -> usize {
                            let thin_ptr = f.as_ref() as *const dyn EvaluateDeferredFnToBoxedDynCombinator;
                            let addr = std::ptr::addr_of!(thin_ptr) as usize;
                            println!("addr: {addr:?}");
                            addr
                        }
                        if let Some(cached) = deferred_cache.get(&get_addr(f)) {
                            cached.clone().into()
                        } else {
                            let strong = strong_ref();
                            let weak = strong.downgrade();
                            deferred_cache.insert(get_addr(f), Ref::Weak(weak.clone()));
                            let mut evaluated: Combinator = f.evaluate_deferred_fn_to_combinator();
                            compile_inner(&mut evaluated, deferred_cache);
                            strong.set(evaluated);
                            deferred_cache.insert(get_addr(f), Ref::Strong(strong.clone()));
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