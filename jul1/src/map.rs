use crate::{CacheContext, Cached, CheckRightData, Choice, Combinator, Deferred, EatByteStringChoice, EatString, EatU8, Eps, ExcludeBytestrings, Fail, ForbidFollows, ForbidFollowsCheckNot, ForbidFollowsClear, ForwardRef, IndentCombinator, Lookahead, LookaheadContext, MutateRightData, Opt, Profiled, Repeat1, Seq, Symbol, tag, Tagged};

impl Combinator {
    pub fn map<F>(&self, f: F) -> Combinator
    where
        F: Fn(&Combinator) -> Combinator,
    {
        match self {
            Combinator::Seq(Seq { children }) => {
                let mapped_children = children.iter().map(|c| f(&c.as_ref().into())).collect();
                Seq { children: mapped_children }.into()
            }
            Combinator::Choice(Choice { children, greedy }) => {
                let mapped_children = children.iter().map(|c| f(&c.as_ref().into())).collect();
                Choice { children: mapped_children, greedy: *greedy }.into()
            }
            Combinator::EatU8(EatU8 { u8set }) => EatU8 { u8set: u8set.clone() }.into(),
            Combinator::EatString(EatString { string }) => EatString { string: string.clone() }.into(),
            Combinator::Eps(Eps {}) => Eps.into(),
            Combinator::Fail(Fail {}) => Fail.into(),
            Combinator::CacheContext(CacheContext { inner }) => CacheContext { inner: Box::new(f(inner)) }.into(),
            Combinator::Cached(Cached { inner }) => Cached { inner: Rc::new(f(inner)) }.into(),
            Combinator::IndentCombinator(IndentCombinator {}) => IndentCombinator.into(),
            Combinator::MutateRightData(MutateRightData { run }) => MutateRightData { run: run.clone() }.into(),
            Combinator::Repeat1(Repeat1 { a, greedy }) => Repeat1 { a: Rc::new(f(a)), greedy: *greedy }.into(),
            Combinator::Symbol(Symbol { value }) => Symbol { value: Rc::new(f(value)) }.into(),
            Combinator::Tagged(Tagged { inner, tag }) => Tagged { inner: Box::new(f(inner)), tag: tag.clone() }.into(),
            Combinator::ForwardRef(ForwardRef { a }) => ForwardRef { a: a.clone() }.into(),
            Combinator::ForbidFollows(ForbidFollows { match_ids }) => ForbidFollows { match_ids: match_ids.clone() }.into(),
            Combinator::ForbidFollowsClear(ForbidFollowsClear {}) => ForbidFollowsClear.into(),
            Combinator::ForbidFollowsCheckNot(ForbidFollowsCheckNot { match_id }) => ForbidFollowsCheckNot { match_id: *match_id }.into(),
            Combinator::EatByteStringChoice(EatByteStringChoice { root }) => EatByteStringChoice { root: root.clone() }.into(),
            Combinator::CheckRightData(CheckRightData { run }) => CheckRightData { run: run.clone() }.into(),
            Combinator::Deferred(Deferred { f }) => Deferred { f: *f }.into(),
            Combinator::Lookahead(Lookahead { combinator, positive, persist_with_partial_lookahead }) => Lookahead { combinator: Box::new(f(combinator)), positive: *positive, persist_with_partial_lookahead: *persist_with_partial_lookahead }.into(),
            Combinator::ExcludeBytestrings(ExcludeBytestrings { inner, bytestrings_to_exclude }) => ExcludeBytestrings { inner: Box::new(f(inner)), bytestrings_to_exclude: bytestrings_to_exclude.clone() }.into(),
            Combinator::LookaheadContext(LookaheadContext { inner, persist_with_partial_lookahead }) => LookaheadContext { inner: Box::new(f(inner)), persist_with_partial_lookahead: *persist_with_partial_lookahead }.into(),
            Combinator::Profiled(Profiled { inner, tag }) => Profiled { inner: Box::new(f(inner)), tag: tag.clone() }.into(),
            Combinator::Opt(Opt { inner, greedy }) => Opt { inner: Box::new(f(inner)), greedy: *greedy }.into(),
        }
    }

    pub fn map_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut Combinator),
    {
        match self {
            Combinator::Seq(Seq { children }) => {
                for c in children {
                    f(&mut c.as_ref().into());
                }
            }
            Combinator::Choice(Choice { children, .. }) => {
                for c in children {
                    f(&mut c.as_ref().into());
                }
            }
            Combinator::EatU8(EatU8 { .. }) => {}
            Combinator::EatString(EatString { .. }) => {}
            Combinator::Eps(Eps {}) => {}
            Combinator::Fail(Fail {}) => {}
            Combinator::CacheContext(CacheContext { inner }) => f(inner),
            Combinator::Cached(Cached { inner }) => f(&mut inner.as_ref().into()),
            Combinator::IndentCombinator(IndentCombinator {}) => {}
            Combinator::MutateRightData(MutateRightData { .. }) => {}
            Combinator::Repeat1(Repeat1 { a, .. }) => f(&mut a.as_ref().into()),
            Combinator::Symbol(Symbol { value }) => f(&mut value.as_ref().into()),
            Combinator::Tagged(Tagged { inner, .. }) => f(inner),
            Combinator::ForwardRef(ForwardRef { .. }) => {}
            Combinator::ForbidFollows(ForbidFollows { .. }) => {}
            Combinator::ForbidFollowsClear(ForbidFollowsClear {}) => {}
            Combinator::ForbidFollowsCheckNot(ForbidFollowsCheckNot { .. }) => {}
            Combinator::EatByteStringChoice(EatByteStringChoice { .. }) => {}
            Combinator::CheckRightData(CheckRightData { .. }) => {}
            Combinator::Deferred(Deferred { .. }) => {}
            Combinator::Lookahead(Lookahead { combinator, .. }) => f(combinator),
            Combinator::ExcludeBytestrings(ExcludeBytestrings { inner, .. }) => f(inner),
            Combinator::LookaheadContext(LookaheadContext { inner, .. }) => f(inner),
            Combinator::Profiled(Profiled { inner, .. }) => f(inner),
            Combinator::Opt(Opt { inner, .. }) => f(inner),
        }
    }
}

