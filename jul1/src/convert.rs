use crate::{deferred, Combinator, CombinatorTrait, Deferred, StrongRef, Symbol, WeakRef};

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

impl<T: CombinatorTrait + 'static> IntoCombinator for &Symbol<T> {
    type Output = Symbol<T>;
    fn into_combinator(self) -> Self::Output {
        self.clone()
    }
}

impl IntoCombinator for &StrongRef {
    type Output = StrongRef;
    fn into_combinator(self) -> Self::Output {
        self.clone()
    }
}

pub trait IntoDyn {
    fn into_dyn(self) -> Box<dyn CombinatorTrait>;
}

impl<T: CombinatorTrait + 'static> IntoDyn for T {
    fn into_dyn(self) -> Box<dyn CombinatorTrait> {
        Box::new(self)
    }
}