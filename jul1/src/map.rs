use crate::{CacheContext, Cached, CheckRightData, Choice, Combinator, Deferred, EatByteStringChoice, EatString, EatU8, Eps, ExcludeBytestrings, Fail, ForbidFollows, ForbidFollowsCheckNot, ForbidFollowsClear, ForwardRef, IndentCombinator, Lookahead, LookaheadContext, MutateRightData, Opt, Profiled, Repeat1, Seq, Symbol, tag, Tagged};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

impl Combinator {
    pub fn apply_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Combinator),
    {
        match self {
            Combinator::Seq(Seq { children }) => {
                for child in Rc::make_mut(children).iter_mut() {
                    f(child);
                }
            }
            Combinator::Choice(Choice { children, greedy }) => {
                for child in children.iter_mut() {
                    f(Rc::make_mut(child));
                }
            }
            Combinator::EatU8(EatU8 { u8set }) => {}
            Combinator::EatString(EatString { string }) => {}
            Combinator::Eps(Eps {}) => {}
            Combinator::Fail(Fail {}) => {}
            Combinator::CacheContext(CacheContext { inner }) => {
                f(inner);
            }
            Combinator::Cached(Cached { inner }) => {
                f(Rc::make_mut(inner));
            }
            Combinator::IndentCombinator(_) => {}
            Combinator::MutateRightData(MutateRightData { run }) => {}
            Combinator::Repeat1(Repeat1 { a, greedy }) => {
                f(Rc::make_mut(a));
            }
            Combinator::Symbol(Symbol { value }) => {
                f(Rc::make_mut(value));
            }
            Combinator::Tagged(Tagged { inner, tag }) => {
                f(inner);
            }
            Combinator::ForwardRef(ForwardRef { a }) => {
                f(Rc::make_mut(RefCell::borrow_mut(a).as_mut().unwrap()));
            }
            Combinator::ForbidFollows(ForbidFollows { match_ids }) => {}
            Combinator::ForbidFollowsClear(ForbidFollowsClear {}) => {}
            Combinator::ForbidFollowsCheckNot(ForbidFollowsCheckNot { match_id }) => {}
            Combinator::EatByteStringChoice(EatByteStringChoice { root }) => {}
            Combinator::CheckRightData(CheckRightData { run }) => {}
            Combinator::Deferred(Deferred { f }) => {}
            Combinator::Lookahead(Lookahead { combinator, positive, persist_with_partial_lookahead }) => {
                f(combinator.borrow_mut());
            }
            Combinator::ExcludeBytestrings(ExcludeBytestrings { inner, bytestrings_to_exclude }) => {
                f(inner);
            }
            Combinator::LookaheadContext(LookaheadContext { inner, persist_with_partial_lookahead }) => {
                f(inner);
            }
            Combinator::Profiled(Profiled { inner, tag }) => {
                f(inner);
            }
            Combinator::Opt(Opt { inner, greedy }) => {
                f(inner);
            }
        }
    }
}