use std::collections::HashSet;
use crate::*;

impl Combinator<'_> {
    pub fn compile(mut self) -> Combinator<'static> {
        let mut deferred_cache: HashMap<Deferred, ForwardRef> = HashMap::new();
        let mut forward_refs: HashSet<ForwardRef> = HashSet::new();
        self.apply_recursive_preorder_mut(&mut |combinator| {
            match combinator {
                // Construct the deferred combinator
                Combinator::Deferred(inner) => {
                    if let Some(forward_ref) = deferred_cache.get(&inner) {
                        *combinator = forward_ref.into();
                    } else {
                        let mut forward_ref = forward_ref();
                        deferred_cache.insert(inner.clone(), forward_ref.clone());
                        forward_refs.insert(forward_ref.clone());
                        let evaluated = (inner.f)();
                        forward_ref.set(evaluated.clone());
                        *combinator = evaluated;
                    }
                    true
                }
                Combinator::ForwardRef(forward_ref) => {
                    if forward_refs.contains(forward_ref) {
                        true
                    } else {
                        // forward_refs.insert(forward_ref.clone());
                        false
                    }
                }
                _ => true
            }
        });
        self
    }
}