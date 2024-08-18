// src/compiler.rs
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use crate::*;
use std::any::Any;

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
}

impl<T: CombinatorTrait> Compile for T {
    fn compile(mut self) -> Self {
        let mut deferred_cache: HashMap<usize, Ref<Combinator>> = HashMap::new();
        fn compile_inner(combinator: &dyn CombinatorTrait, deferred_cache: &mut HashMap<usize, Ref<Combinator>>) {
            // Use a dynamic check for the Deferred trait
            if let Some(deferred) = combinator.as_any().downcast_ref::<dyn DeferredCompiler>() {
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

// Add this trait implementation to Deferred<T>
impl<T: CombinatorTrait + 'static> DeferredCompiler for Deferred<T> {
    fn get_deferred_addr(&self) -> usize {
        self.deferred_fn.get_addr()
    }

    fn evaluate_to_combinator(&self) -> Combinator {
        Box::new(self.deferred_fn.evaluate_to_combinator())
    }
}

// Add a method for checking if the Deferred is compiled
impl<T: CombinatorTrait + 'static> Deferred<T> {
    fn is_compiled(&self) -> bool {
        self.inner.get().is_some()
    }

    fn set_inner(&self, inner: DeferredInner<Combinator>) {
        // This is safe because we know the inner type is Combinator due to the
        // implementation of DeferredCompiler for Deferred<T>
        let inner = unsafe { std::mem::transmute::<DeferredInner<Combinator>, DeferredInner<T>>(inner) };
        self.inner.set(inner).ok().expect("Cannot set inner value more than once");
    }
}