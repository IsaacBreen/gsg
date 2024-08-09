use std::collections::HashSet;
use crate::*;

impl Combinator {
    pub fn compile(mut self) -> Combinator {
        let mut deferred_cache: HashMap<Deferred, Combinator> = HashMap::new();
        fn compile_inner(combinator: &mut Combinator, deferred_cache: &mut HashMap<Deferred, Combinator>) {
            match combinator {
                // Construct the deferred combinator
                Combinator::Deferred(inner) => {
                    if let Some(cached) = deferred_cache.get(&inner) {
                        *combinator = cached.clone();
                    } else {
                        let strong = strong_ref();
                        let weak = strong.downgrade();
                        deferred_cache.insert(inner.clone(), weak.clone().into());
                        let mut evaluated = (inner.f)();
                        compile_inner(&mut evaluated, deferred_cache);
                        deferred_cache.insert(inner.clone(), evaluated.clone());
                        strong.set(evaluated);
                        *combinator = strong.into();
                    }
                }
                _ => {
                    combinator.apply_mut(|combinator| {
                        compile_inner(combinator, deferred_cache);
                    });
                }
            }
        }
        compile_inner(&mut self, &mut deferred_cache);
        self
    }
}