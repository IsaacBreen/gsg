use crate::*;
use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

impl Combinator {
    pub fn apply_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Combinator),
    {
        match self {
            Combinator::Seq(Seq { children }) | Combinator::Choice(Choice { children, .. }) => {
                for child in Rc::make_mut(children).iter_mut() {
                    f(child);
                }
            }
            Combinator::Cached(Cached { inner }) | Combinator::Repeat1(Repeat1 { a: inner, .. }) | Combinator::Symbol(Symbol { value: inner }) => {
                f(Rc::make_mut(inner));
            }
            Combinator::CacheContext(CacheContext { inner }) | Combinator::Tagged(Tagged { inner, .. }) | Combinator::Lookahead(Lookahead { combinator: inner, .. }) | Combinator::ExcludeBytestrings(ExcludeBytestrings { inner, .. }) | Combinator::LookaheadContext(LookaheadContext { inner, .. }) | Combinator::Profiled(Profiled { inner, .. }) | Combinator::Opt(Opt { inner, .. }) => {
                f(inner);
            }
            Combinator::ForwardRef(ForwardRef { a }) => {
                f(Rc::make_mut(RefCell::borrow_mut(a).as_mut().unwrap()));
            }
            Combinator::EatU8(_) => {}
            Combinator::EatString(_) => {}
            Combinator::Eps(_) => {}
            Combinator::Fail(_) => {}
            Combinator::IndentCombinator(_) => {}
            Combinator::MutateRightData(_) => {}
            Combinator::ForbidFollows(_) => {}
            Combinator::ForbidFollowsClear(_) => {}
            Combinator::ForbidFollowsCheckNot(_) => {}
            Combinator::EatByteStringChoice(_) => {}
            Combinator::CheckRightData(_) => {}
            Combinator::Deferred(_) => {}
        }
    }

    pub fn apply_recursive_preorder_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Combinator),
    {
        f(self);
        self.apply_mut(f);
    }

    pub fn apply_recursive_postorder_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Combinator),
    {
        self.apply_mut(&mut f);
        f(self);
    }

    pub fn map<F>(mut self, f: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        self.apply_mut(|combinator| *combinator = f(combinator.clone()));
        self
    }

    pub fn map_recursive_preorder<F>(mut self, f: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        self.apply_recursive_preorder_mut(|combinator| *combinator = f(combinator.clone()));
        self
    }

    pub fn map_recursive_postorder<F>(mut self, f: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        self.apply_recursive_postorder_mut(|combinator| *combinator = f(combinator.clone()));
        self
    }
}