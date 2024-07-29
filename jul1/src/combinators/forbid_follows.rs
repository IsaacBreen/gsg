use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct ForbidFollowsData {
    pub prev_match_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForbidFollows {
    match_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForbidFollowsClear {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForbidFollowsCheckNot {
    match_id: String,
}

impl CombinatorTrait for ForbidFollows {
    fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
        right_data.forbidden_consecutive_matches.prev_match_ids = self.match_ids.clone();
        (combinator::Parser::FailParser(FailParser), ParseResults {
            right_data_vec: vec![right_data],
            up_data_vec: vec![],
            done: true,
        })
    }
}

impl CombinatorTrait for ForbidFollowsClear {
    fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
        right_data.forbidden_consecutive_matches.prev_match_ids.clear();
        (combinator::Parser::FailParser(FailParser), ParseResults {
            right_data_vec: vec![right_data],
            up_data_vec: vec![],
            done: true,
        })
    }
}

impl CombinatorTrait for ForbidFollowsCheckNot {
    fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
        if right_data.forbidden_consecutive_matches.prev_match_ids.contains(&self.match_id) {
            (combinator::Parser::FailParser(FailParser), ParseResults::empty_finished())
        } else {
            (combinator::Parser::FailParser(FailParser), ParseResults {
                right_data_vec: vec![right_data],
                up_data_vec: vec![],
                done: true,
            })
        }
    }
}

pub fn forbid_follows(match_ids: &[&str]) -> ForbidFollows {
    ForbidFollows { match_ids: match_ids.iter().map(|s| s.to_string()).collect() }
}

pub fn forbid_follows_clear() -> ForbidFollowsClear {
    ForbidFollowsClear {}
}

pub fn forbid_follows_check_not(match_id: &str) -> ForbidFollowsCheckNot {
    ForbidFollowsCheckNot { match_id: match_id.to_string() }
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
