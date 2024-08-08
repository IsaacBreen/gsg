use crate::{CacheContext, Cached, CheckRightData, Choice, Combinator, Deferred, EatByteStringChoice, EatString, EatU8, Eps, ExcludeBytestrings, Fail, ForbidFollows, ForbidFollowsCheckNot, ForbidFollowsClear, ForwardRef, IndentCombinator, Lookahead, LookaheadContext, MutateRightData, Opt, Profiled, Repeat1, Seq, Symbol, tag, Tagged};

impl Combinator {
    pub fn traverse<F>(&self, f: F) -> Combinator
    where
        F: Fn(&Combinator),
    {
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
        todo!()
    }
}

