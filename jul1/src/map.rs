use crate::{CacheContext, Cached, CheckRightData, Choice, Combinator, Deferred, EatByteStringChoice, EatString, EatU8, Eps, ExcludeBytestrings, Fail, ForbidFollows, ForbidFollowsCheckNot, ForbidFollowsClear, ForwardRef, IndentCombinator, Lookahead, LookaheadContext, MutateRightData, Opt, Profiled, Repeat1, Seq, Symbol, tag, Tagged};

impl Combinator {
    pub fn traverse<F>(&self, f: F) -> Combinator
    where
        F: Fn(&Combinator),
    {
        f(self);
        match self {
            Combinator::Seq(Seq { children }) => {
                Combinator::Seq(Seq { children: children.iter().map(|c| c.traverse(&f)).collect() })
            }
            Combinator::Choice(Choice { children, greedy }) => {
                Combinator::Choice(Choice { children: children.iter().map(|c| c.traverse(&f)).collect(), greedy: *greedy })
            }
            Combinator::EatU8(EatU8 { u8set }) => {
                Combinator::EatU8(EatU8 { u8set: *u8set })
            }
            Combinator::EatString(EatString { string }) => {
                Combinator::EatString(EatString { string: string.clone() })
            }
            Combinator::Eps(Eps {}) => {
                Combinator::Eps(Eps {})
            }
            Combinator::Fail(Fail {}) => {
                Combinator::Fail(Fail {})
            }
            Combinator::CacheContext(CacheContext { inner }) => {
                Combinator::CacheContext(CacheContext { inner: Box::new(inner.traverse(&f)) })
            }
            Combinator::Cached(Cached { inner }) => {
                Combinator::Cached(Cached { inner: inner.clone() })
            }
            Combinator::IndentCombinator(IndentCombinator {}) => {
                Combinator::IndentCombinator(IndentCombinator {})
            }
            Combinator::MutateRightData(MutateRightData { run }) => {
                Combinator::MutateRightData(MutateRightData { run: run.clone() })
            }
            Combinator::Repeat1(Repeat1 { a, greedy }) => {
                Combinator::Repeat1(Repeat1 { a: a.traverse(&f), greedy: *greedy })
            }
            Combinator::Symbol(Symbol { value }) => {
                Combinator::Symbol(Symbol { value: value.clone() })
            }
            Combinator::Tagged(Tagged { inner, tag }) => {
                Combinator::Tagged(Tagged { inner: Box::new(inner.traverse(&f)), tag: tag.clone() })
            }
            Combinator::ForwardRef(ForwardRef { a }) => {
                Combinator::ForwardRef(ForwardRef { a: a.clone() })
            }
            Combinator::ForbidFollows(ForbidFollows { match_ids }) => {
                Combinator::ForbidFollows(ForbidFollows { match_ids: match_ids.clone() })
            }
            Combinator::ForbidFollowsClear(ForbidFollowsClear {}) => {
                Combinator::ForbidFollowsClear(ForbidFollowsClear {})
            }
            Combinator::ForbidFollowsCheckNot(ForbidFollowsCheckNot { match_id }) => {
                Combinator::ForbidFollowsCheckNot(ForbidFollowsCheckNot { match_id: *match_id })
            }
            Combinator::EatByteStringChoice(EatByteStringChoice { root }) => {
                Combinator::EatByteStringChoice(EatByteStringChoice { root: root.clone() })
            }
            Combinator::CheckRightData(CheckRightData { run }) => {
                Combinator::CheckRightData(CheckRightData { run: run.clone() })
            }
            Combinator::Deferred(Deferred { f }) => {
                Combinator::Deferred(Deferred { f: *f })
            }
            Combinator::Lookahead(Lookahead { combinator, positive, persist_with_partial_lookahead }) => {
                Combinator::Lookahead(Lookahead { combinator: Box::new(combinator.traverse(&f)), positive: *positive, persist_with_partial_lookahead: *persist_with_partial_lookahead })
            }
            Combinator::ExcludeBytestrings(ExcludeBytestrings { inner, bytestrings_to_exclude }) => {
                Combinator::ExcludeBytestrings(ExcludeBytestrings { inner: Box::new(inner.traverse(&f)), bytestrings_to_exclude: bytestrings_to_exclude.clone() })
            }
            Combinator::LookaheadContext(LookaheadContext { inner, persist_with_partial_lookahead }) => {
                Combinator::LookaheadContext(LookaheadContext { inner: Box::new(inner.traverse(&f)), persist_with_partial_lookahead: *persist_with_partial_lookahead })
            }
            Combinator::Profiled(Profiled { inner, tag }) => {
                Combinator::Profiled(Profiled { inner: Box::new(inner.traverse(&f)), tag: tag.clone() })
            }
            Combinator::Opt(Opt { inner, greedy }) => {
                Combinator::Opt(Opt { inner: Box::new(inner.traverse(&f)), greedy: *greedy })
            }
        }
    }

    pub fn map_mut(&mut self, ...) {
        match self {
            Combinator::Seq(Seq { children }) => { todo!() }
            Combinator::Choice(Choice { children, greedy }) => { todo!() }
            Combinator::EatU8(EatU8 { u8set }) => { todo!() }
            Combinator::EatString(EatString { string }) => { todo!() }
            Combinator::Eps(Eps {}) => { todo!() }
            Combinator::Fail(Fail {}) => { todo!() }
            Combinator::CacheContext(CacheContext { inner }) => { todo!() }
            Combinator::Cached(Cached { inner }) => { todo!() }
            Combinator::IndentCombinator(IndentCombinator {}) => { todo!() }
            Combinator::MutateRightData(MutateRightData { run }) => { todo!() }
            Combinator::Repeat1(Repeat1 { a, greedy }) => { todo!() }
            Combinator::Symbol(Symbol { value }) => { todo!() }
            Combinator::Tagged(Tagged { inner, tag }) => { todo!() }
            Combinator::ForwardRef(ForwardRef { a }) => { todo!() }
            Combinator::ForbidFollows(ForbidFollows { match_ids }) => { todo!() }
            Combinator::ForbidFollowsClear(ForbidFollowsClear {}) => { todo!() }
            Combinator::ForbidFollowsCheckNot(ForbidFollowsCheckNot { match_id }) => { todo!() }
            Combinator::EatByteStringChoice(EatByteStringChoice { root }) => { todo!() }
            Combinator::CheckRightData(CheckRightData { run }) => { todo!() }
            Combinator::Deferred(Deferred { f }) => { todo!() }
            Combinator::Lookahead(Lookahead { combinator, positive, persist_with_partial_lookahead }) => { todo!() }
            Combinator::ExcludeBytestrings(ExcludeBytestrings { inner, bytestrings_to_exclude }) => { todo!() }
            Combinator::LookaheadContext(LookaheadContext { inner, persist_with_partial_lookahead }) => { todo!() }
            Combinator::Profiled(Profiled { inner, tag }) => { todo!() }
            Combinator::Opt(Opt { inner, greedy }) => { todo!() }
        };
    }

    pub fn traverse_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut Combinator),
    {
        f(self);
        match self {
            Combinator::Seq(Seq { children }) => {
                children.iter_mut().for_each(|c| c.traverse_mut(&f));
            }
            Combinator::Choice(Choice { children, greedy }) => {
                children.iter_mut().for_each(|c| c.traverse_mut(&f));
            }
            Combinator::EatU8(EatU8 { u8set }) => {}
            Combinator::EatString(EatString { string }) => {}
            Combinator::Eps(Eps {}) => {}
            Combinator::Fail(Fail {}) => {}
            Combinator::CacheContext(CacheContext { inner }) => {
                inner.traverse_mut(&f);
            }
            Combinator::Cached(Cached { inner }) => {}
            Combinator::IndentCombinator(IndentCombinator {}) => {}
            Combinator::MutateRightData(MutateRightData { run }) => {}
            Combinator::Repeat1(Repeat1 { a, greedy }) => {
                a.traverse_mut(&f);
            }
            Combinator::Symbol(Symbol { value }) => {}
            Combinator::Tagged(Tagged { inner, tag }) => {
                inner.traverse_mut(&f);
            }
            Combinator::ForwardRef(ForwardRef { a }) => {}
            Combinator::ForbidFollows(ForbidFollows { match_ids }) => {}
            Combinator::ForbidFollowsClear(ForbidFollowsClear {}) => {}
            Combinator::ForbidFollowsCheckNot(ForbidFollowsCheckNot { match_id }) => {}
            Combinator::EatByteStringChoice(EatByteStringChoice { root }) => {}
            Combinator::CheckRightData(CheckRightData { run }) => {}
            Combinator::Deferred(Deferred { f }) => {}
            Combinator::Lookahead(Lookahead { combinator, positive, persist_with_partial_lookahead }) => {
                combinator.traverse_mut(&f);
            }
            Combinator::ExcludeBytestrings(ExcludeBytestrings { inner, bytestrings_to_exclude }) => {
                inner.traverse_mut(&f);
            }
            Combinator::LookaheadContext(LookaheadContext { inner, persist_with_partial_lookahead }) => {
                inner.traverse_mut(&f);
            }
            Combinator::Profiled(Profiled { inner, tag }) => {
                inner.traverse_mut(&f);
            }
            Combinator::Opt(Opt { inner, greedy }) => {
                inner.traverse_mut(&f);
            }
        };
    }
}

