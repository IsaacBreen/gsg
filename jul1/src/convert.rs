use crate::{deferred, fast_combinator, Combinator, CombinatorTrait, Deferred, FastCombinatorWrapper, StrongRef, Symbol, WeakRef};
use crate::tokenizer::finite_automata::Expr;

pub trait IntoCombinator {
    type Output: CombinatorTrait;
    fn into_combinator(self) -> Self::Output;
}

impl<T: CombinatorTrait> IntoCombinator for T {
    type Output = Self;
    fn into_combinator(self) -> Self::Output {
        self
    }
}

// impl<T: IntoCombinator, F: Fn() -> T> IntoCombinator for F {
//     type Output = T::Output;
//     fn into_combinator(self) -> Self::Output {
//         self().into_combinator()
//     }
// }

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

impl IntoCombinator for Expr {
    type Output = FastCombinatorWrapper;
    fn into_combinator(self) -> Self::Output {
        fast_combinator(self)
    }
}

pub trait IntoDyn<'a> {
    fn into_dyn(self) -> impl CombinatorTrait + 'a;
}

impl<'a, T: CombinatorTrait + 'a> IntoDyn<'a> for T {
    fn into_dyn(self) -> impl CombinatorTrait + 'a {
        self
    }
}