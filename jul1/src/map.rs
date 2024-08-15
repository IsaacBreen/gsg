use crate::*;
use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

impl Combinator {
    pub fn apply(&self, f: &mut impl FnMut(&Combinator)) {
        match self {
            Combinator::Seq(Seq { children, .. }) | Combinator::Choice(Choice { children, .. }) => {
                for child in children.iter() {
                    f(child);
                }
            }
            Combinator::Cached(Cached { inner }) | Combinator::Repeat1(Repeat1 { a: inner, .. }) | Combinator::Symbol(Symbol { value: inner }) => {
                f(inner);
            }
            Combinator::CacheContext(CacheContext { inner }) | Combinator::Tagged(Tagged { inner, .. }) | Combinator::Lookahead(Lookahead { combinator: inner, .. }) | Combinator::ExcludeBytestrings(ExcludeBytestrings { inner, .. }) | Combinator::Profiled(Profiled { inner, .. }) | Combinator::Opt(Opt { inner, .. }) => {
                f(inner);
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
            Combinator::WeakRef(inner) => todo!(),
            Combinator::StrongRef(inner) => todo!(),
            Combinator::BruteForce(_) => {},
            Combinator::Continuation(_) => {},
            Combinator::Fast(_) => {}
            Combinator::Repeat0(Opt { inner: Repeat1 { a: inner, .. }, .. }) => {
                f(inner);
            }
            Combinator::SepRep1(_) => todo!(),
            Combinator::Dyn(inner) => {
                f(inner);
            }
            Combinator::DynRc(_) => {},
        }
    }
}