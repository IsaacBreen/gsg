use crate::{deferred, Combinator, CombinatorTrait, Deferred, Symbol};

pub trait IntoCombinator {
    type Output: CombinatorTrait;
    fn into_combinator(self) -> Self::Output;
}

impl<T: CombinatorTrait> IntoCombinator for T {
    type Output = T;
    fn into_combinator(self) -> Self::Output {
        self
    }
}

// impl<F> IntoCombinator for &F
// where
//     F: Fn() -> Combinator
// {
//     type Output = Deferred;
//
//     fn into_combinator(self) -> Self::Output {
//         deferred(&self)
//     }
// }

impl IntoCombinator for &Symbol {
    type Output = Symbol;
    fn into_combinator(self) -> Self::Output {
        self.clone()
    }
}

pub trait IntoDyn {
    fn into_dyn(self) -> Box<dyn CombinatorTrait>;
}

impl<T: IntoCombinator> IntoDyn for T {
    fn into_dyn(self) -> Box<dyn CombinatorTrait> {
        Box::new(self.into_combinator())
    }
}