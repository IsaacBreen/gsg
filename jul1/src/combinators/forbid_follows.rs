use crate::*;
use crate::VecX;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct ForbidFollowsData {
    pub prev_match_ids: VecX<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForbidFollows {
    pub(crate) match_ids: VecX<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForbidFollowsClear {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForbidFollowsCheckNot {
    pub(crate) match_id: usize,
}

impl CombinatorTrait<'_> for ForbidFollows {
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        Rc::make_mut(&mut right_data.right_data_inner).forbidden_consecutive_matches.prev_match_ids = self.match_ids.clone();
        (combinator::Parser::FailParser(FailParser), ParseResults::new_single(right_data, true))
    }
}

impl CombinatorTrait<'_> for ForbidFollowsClear {
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        Rc::make_mut(&mut right_data.right_data_inner).forbidden_consecutive_matches.prev_match_ids.clear();
        (combinator::Parser::FailParser(FailParser), ParseResults::new_single(right_data, true))
    }
}

impl CombinatorTrait<'_> for ForbidFollowsCheckNot {
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        if right_data.right_data_inner.forbidden_consecutive_matches.prev_match_ids.contains(&self.match_id) {
            (combinator::Parser::FailParser(FailParser), ParseResults::empty_finished())
        } else {
            Rc::make_mut(&mut right_data.right_data_inner).forbidden_consecutive_matches.prev_match_ids.clear();
            (combinator::Parser::FailParser(FailParser), ParseResults::new_single(right_data, true))
        }
    }
}

pub fn forbid_follows(match_ids: &[usize]) -> ForbidFollows {
    ForbidFollows { match_ids: match_ids.into() }
}

pub fn forbid_follows_clear() -> ForbidFollowsClear {
    ForbidFollowsClear {}
}

pub fn forbid_follows_check_not(match_id: usize) -> ForbidFollowsCheckNot {
    ForbidFollowsCheckNot { match_id }
}

impl From<ForbidFollows> for Combinator<'_> {
    fn from(value: ForbidFollows) -> Self {
        Combinator::ForbidFollows(value)
    }
}

impl From<ForbidFollowsClear> for Combinator<'_> {
    fn from(value: ForbidFollowsClear) -> Self {
        Combinator::ForbidFollowsClear(value)
    }
}

impl From<ForbidFollowsCheckNot> for Combinator<'_> {
    fn from(value: ForbidFollowsCheckNot) -> Self {
        Combinator::ForbidFollowsCheckNot(value)
    }
}
