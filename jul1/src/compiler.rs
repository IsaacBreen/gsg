use std::collections::HashMap;
use crate::*;

enum Ref<T> {
    Strong(StrongRef<T>),
    Weak(WeakRef<T>),
}

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        match self {
            Ref::Strong(strong) => Ref::Strong(strong.clone()),
            Ref::Weak(weak) => Ref::Weak(weak.clone()),
        }
    }
}

impl<T> From<Ref<T>> for DeferredInner<T> {
    fn from(value: Ref<T>) -> Self {
        match value {
            Ref::Strong(strong) => DeferredInner::CompiledStrong(strong),
            Ref::Weak(weak) => DeferredInner::CompiledWeak(weak),
        }
    }
}

pub trait Compile {
    fn compile(self) -> Self;
}

// Trait for interacting with Deferred combinators
pub trait DeferredCompiler {
    fn get_deferred_addr(&self) -> usize;
    fn evaluate_to_combinator(&self) -> Combinator;
    fn is_compiled(&self) -> bool;
    fn set_inner(&self, inner: DeferredInner<Combinator>);
}

// Intermediate trait for downcasting to DeferredCompiler
pub trait AsDeferredCompiler {
    fn as_deferred_compiler(&self) -> Option<&dyn DeferredCompiler>;
}

impl<T: CombinatorTrait + ?Sized> AsDeferredCompiler for T {
    fn as_deferred_compiler(&self) -> Option<&dyn DeferredCompiler> {
        self.as_any().downcast_ref::<dyn DeferredCompiler>()
    }
}

impl<T: CombinatorTrait> Compile for T {
    fn compile(mut self) -> Self {
        let mut deferred_cache: HashMap<usize, Ref<Combinator>> = HashMap::new();
        fn compile_inner(combinator: &dyn CombinatorTrait, deferred_cache: &mut HashMap<usize, Ref<Combinator>>) {
            // Use a dynamic check for the Deferred trait
            if let Some(deferred) = combinator.as_deferred_compiler() {
                if deferred.is_compiled() {
                    return;
                }

                let addr = deferred.get_deferred_addr();

                let new_inner: DeferredInner<Combinator> = if let Some(cached) = deferred_cache.get(&addr) {
                    cached.clone().into()
                } else {
                    let strong = strong_ref();
                    let weak = strong.downgrade();
                    deferred_cache.insert(addr, Ref::Weak(weak.clone()));

                    let mut evaluated: Combinator = deferred.evaluate_to_combinator();
                    compile_inner(&mut evaluated, deferred_cache);

                    strong.set(evaluated);
                    deferred_cache.insert(addr, Ref::Strong(strong.clone()));
                    DeferredInner::CompiledStrong(strong.clone())
                };

                deferred.set_inner(new_inner);
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
