use crate::*;

impl Combinator {
    pub fn compile(mut self) -> Combinator {
        self.map_recursive_preorder(|combinator| {
            match combinator {
                // Construct the deferred combinator
                Combinator::Deferred(Deferred { f }) => f(),
                _ => combinator
            }
        })
    }
}