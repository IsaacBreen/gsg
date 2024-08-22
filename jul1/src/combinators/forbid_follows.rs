use crate::*;
use std::rc::Rc;
use crate::BaseCombinatorTrait;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct ForbidFollowsData {
    pub prev_match_ids: u16, // Using a bitset
}

#[derive(Debug)]
pub struct ForbidFollows {
    pub(crate) match_ids: u16, // Using a bitset
}

#[derive(Debug)]
pub struct ForbidFollowsClear {}

#[derive(Debug)]
pub struct ForbidFollowsCheckNot {
    pub(crate) match_ids: u16,
}

impl CombinatorTrait for ForbidFollows {
    type Parser = FailParser;

    fn one_shot_parse(&self, mut right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        right_data.get_inner_mut().fields1.forbidden_consecutive_matches.prev_match_ids = self.match_ids;
        Ok(right_data)
    }
    fn old_parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
        right_data.get_inner_mut().fields1.forbidden_consecutive_matches.prev_match_ids = self.match_ids;
        (FailParser, ParseResults::new_single(right_data, true))
    }
}

impl BaseCombinatorTrait for ForbidFollows {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl CombinatorTrait for ForbidFollowsClear {
    type Parser = FailParser;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        Ok(right_data)
    }
    fn old_parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
        right_data.get_inner_mut().fields1.forbidden_consecutive_matches.prev_match_ids = 0;
        (FailParser, ParseResults::new_single(right_data, true))
    }
}

impl BaseCombinatorTrait for ForbidFollowsClear {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl CombinatorTrait for ForbidFollowsCheckNot {
    type Parser = FailParser;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        Ok(right_data)
    }
    fn old_parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
        if right_data.right_data_inner.fields1.forbidden_consecutive_matches.prev_match_ids & self.match_ids != 0 {
            (FailParser, ParseResults::empty_finished())
        } else {
            right_data.get_inner_mut().fields1.forbidden_consecutive_matches.prev_match_ids = 0;
            (FailParser, ParseResults::new_single(right_data, true))
        }
    }
}

impl BaseCombinatorTrait for ForbidFollowsCheckNot {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub fn forbid_follows(match_ids: &[usize]) -> ForbidFollows { // Using a bitset
    let mut bitset = 0;
    for &id in match_ids {
        bitset |= 1 << TryInto::<u16>::try_into(id).unwrap();
    }
    ForbidFollows { match_ids: bitset }
}

pub fn forbid_follows_clear() -> ForbidFollowsClear {
    ForbidFollowsClear {}
}

pub fn forbid_follows_check_not(match_id: usize) -> ForbidFollowsCheckNot {
    let bitmask = 1 << match_id;
    ForbidFollowsCheckNot { match_ids: bitmask }
}

// impl From<ForbidFollows> for Combinator {
//     fn from(value: ForbidFollows) -> Self {
//         Combinator::ForbidFollows(value)
//     }
// }
//
// impl From<ForbidFollowsClear> for Combinator {
//     fn from(value: ForbidFollowsClear) -> Self {
//         Combinator::ForbidFollowsClear(value)
//     }
// }
//
// impl From<ForbidFollowsCheckNot> for Combinator {
//     fn from(value: ForbidFollowsCheckNot) -> Self {
//         Combinator::ForbidFollowsCheckNot(value)
//     }
// }