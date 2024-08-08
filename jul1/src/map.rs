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
            _ => {}
        }
    }
}