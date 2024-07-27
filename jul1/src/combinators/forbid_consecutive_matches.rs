use std::any::Any;
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct ForbidConsecutiveMatchesData {
    pub prev_match_ids: Vec<String>,
}

pub struct ForbidConsecutiveMatches {
    match_ids: Vec<String>,
}

impl CombinatorTrait for ForbidConsecutiveMatches {
    type Parser = FailParser;
    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        right_data.forbidden_consecutive_matches.prev_match_ids = self.match_ids.clone();
        (FailParser, ParseResults {
            right_data_vec: vec![right_data],
            up_data_vec: vec![],
            done: true,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ForbidConsecutiveMatchesClear {}

impl CombinatorTrait for ForbidConsecutiveMatchesClear {
    type Parser = FailParser;
    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        right_data.forbidden_consecutive_matches.prev_match_ids.clear();
        (FailParser, ParseResults {
            right_data_vec: vec![right_data],
            up_data_vec: vec![],
            done: true,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ForbidConsecutiveMatchesSet {
    match_id: String,
}

impl CombinatorTrait for ForbidConsecutiveMatchesSet {
    type Parser = FailParser;
    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        right_data.forbidden_consecutive_matches.prev_match_ids = vec![self.match_id.clone()];
        (FailParser, ParseResults {
            right_data_vec: vec![right_data],
            up_data_vec: vec![],
            done: true,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ForbidConsecutiveMatchesAdd {
    match_id: String,
}

impl CombinatorTrait for ForbidConsecutiveMatchesAdd {
    type Parser = FailParser;
    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        right_data.forbidden_consecutive_matches.prev_match_ids.push(self.match_id.clone());
        (FailParser, ParseResults {
            right_data_vec: vec![right_data],
            up_data_vec: vec![],
            done: true,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ForbidConsecutiveMatchesCheckNot {
    match_id: String,
}

impl CombinatorTrait for ForbidConsecutiveMatchesCheckNot {
    type Parser = FailParser;
    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        if right_data.forbidden_consecutive_matches.prev_match_ids.contains(&self.match_id) {
            (FailParser, ParseResults::empty_finished())
        } else {
            (FailParser, ParseResults {
                right_data_vec: vec![right_data],
                up_data_vec: vec![],
                done: true,
            })
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn forbid_consecutive_matches(match_ids: &[&str]) -> ForbidConsecutiveMatches {
    ForbidConsecutiveMatches { match_ids: match_ids.iter().map(|s| s.to_string()).collect() }
}

pub fn forbid_consecutive_matches_clear() -> ForbidConsecutiveMatchesClear {
    ForbidConsecutiveMatchesClear {}
}

pub fn forbid_consecutive_matches_set(match_id: &str) -> ForbidConsecutiveMatchesSet {
    ForbidConsecutiveMatchesSet { match_id: match_id.to_string() }
}

pub fn forbid_consecutive_matches_add(match_id: &str) -> ForbidConsecutiveMatchesAdd {
    ForbidConsecutiveMatchesAdd { match_id: match_id.to_string() }
}

pub fn forbid_consecutive_matches_check_not(match_id: &str) -> ForbidConsecutiveMatchesCheckNot {
    ForbidConsecutiveMatchesCheckNot { match_id: match_id.to_string() }
}