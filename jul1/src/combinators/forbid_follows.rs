use crate::*;
use fixedbitset::FixedBitSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct ForbidFollowsData {
    pub prev_match_ids: FixedBitSet,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForbidFollows {
    pub(crate) match_ids: FixedBitSet,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForbidFollowsClear {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForbidFollowsCheckNot {
    pub(crate) match_id: usize,
}

impl CombinatorTrait for ForbidFollows {
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        Rc::make_mut(&mut right_data.right_data_inner).forbidden_consecutive_matches.prev_match_ids = self.match_ids.clone();
        (combinator::Parser::FailParser(FailParser), ParseResults::new_single(right_data, true))
    }
}

impl CombinatorTrait for ForbidFollowsClear {
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        Rc::make_mut(&mut right_data.right_data_inner).forbidden_consecutive_matches.prev_match_ids.clear();
        (combinator::Parser::FailParser(FailParser), ParseResults::new_single(right_data, true))
    }
}

impl CombinatorTrait for ForbidFollowsCheckNot {
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        if right_data.right_data_inner.forbidden_consecutive_matches.prev_match_ids.contains(self.match_id) {
            (combinator::Parser::FailParser(FailParser), ParseResults::empty_finished())
        } else {
            Rc::make_mut(&mut right_data.right_data_inner).forbidden_consecutive_matches.prev_match_ids.clear();
            (combinator::Parser::FailParser(FailParser), ParseResults::new_single(right_data, true))
        }
    }
}

pub fn forbid_follows(match_ids: &[usize]) -> ForbidFollows {
    let mut bitset = FixedBitSet::with_capacity(match_ids.iter().max().unwrap_or(&0) + 1);
    for &id in match_ids {
        bitset.insert(id);
    }
    ForbidFollows { match_ids: bitset }
}

pub fn forbid_follows_clear() -> ForbidFollowsClear {
    ForbidFollowsClear {}
}

pub fn forbid_follows_check_not(match_id: usize) -> ForbidFollowsCheckNot {
    ForbidFollowsCheckNot { match_id }
}

impl From<ForbidFollows> for Combinator {
    fn from(value: ForbidFollows) -> Self {
        Combinator::ForbidFollows(value)
    }
}

impl From<ForbidFollowsClear> for Combinator {
    fn from(value: ForbidFollowsClear) -> Self {
        Combinator::ForbidFollowsClear(value)
    }
}

impl From<ForbidFollowsCheckNot> for Combinator {
    fn from(value: ForbidFollowsCheckNot) -> Self {
        Combinator::ForbidFollowsCheckNot(value)
    }
}