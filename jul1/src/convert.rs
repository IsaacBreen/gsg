// src/convert.rs
use crate::{deferred, fast_combinator, CombinatorTrait, Deferred, DynCombinatorTrait, FastCombinatorWrapper, ParserTrait, StrongRef, Symbol, WeakRef};
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

impl<T: CombinatorTrait> IntoCombinator for &Symbol<T> {
    type Output = Symbol<T>;
    fn into_combinator(self) -> Self::Output {
        self.clone()
    }
}

impl<T: CombinatorTrait> IntoCombinator for &StrongRef<T> {
    type Output = StrongRef<T>;
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
    fn into_dyn(self) -> Box<dyn DynCombinatorTrait + 'a>;
}

impl<'a, T: CombinatorTrait + 'a> IntoDyn<'a> for T {
    fn into_dyn(self) -> Box<dyn DynCombinatorTrait + 'a> {
        Box::new(self)
    }
}