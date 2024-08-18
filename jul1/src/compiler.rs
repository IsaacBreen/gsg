use std::collections::HashMap;
use crate::*;
use castaway::cast;

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

impl<T: CombinatorTrait + 'static> Compile for T {
    fn compile(mut self) -> Self {
        let mut deferred_cache: HashMap<usize, Ref<Combinator>> = HashMap::new();
        fn compile_inner(combinator: Rc<dyn CombinatorTrait>, deferred_cache: &mut HashMap<usize, Ref<Combinator>>) {
            // Use a dynamic check for the Deferred trait
            if let Ok(deferred) = cast!(combinator.as_any(), Box<dyn DeferredCompiler>) {
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
                    compile_inner(evaluated.into(), deferred_cache);

                    strong.set(evaluated);
                    deferred_cache.insert(addr, Ref::Strong(strong.clone()));
                    DeferredInner::CompiledStrong(strong.clone())
                };

                deferred.set_inner(new_inner);
            } else {
                combinator.apply(&mut |combinator| {
                    compile_inner(Rc::new(combinator.clone()), deferred_cache);
                });
            }
        }
        let mut x = Rc::new(self);
        compile_inner(x.clone(), &mut deferred_cache);
        todo!()
    }
}
