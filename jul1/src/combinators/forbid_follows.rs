use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct ForbidFollowsData {
    pub prev_match_ids: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForbidFollows {
    match_ids: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForbidFollowsClear {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForbidFollowsCheckNot {
    match_id: usize,
}

impl CombinatorTrait for ForbidFollows {
    fn parser_with_steps(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        right_data.forbidden_consecutive_matches.prev_match_ids = self.match_ids.clone();
        (combinator::Parser::FailParser(FailParser), ParseResults {
            right_data_vec: vec![right_data],
            done: true,
        })
    }
}

impl CombinatorTrait for ForbidFollowsClear {
    fn parser_with_steps(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        right_data.forbidden_consecutive_matches.prev_match_ids.clear();
        (combinator::Parser::FailParser(FailParser), ParseResults {
            right_data_vec: vec![right_data],
            done: true,
        })
    }
}

impl CombinatorTrait for ForbidFollowsCheckNot {
    fn parser_with_steps(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        if right_data.forbidden_consecutive_matches.prev_match_ids.contains(&self.match_id) {
            (combinator::Parser::FailParser(FailParser), ParseResults::empty_finished())
        } else {
            right_data.forbidden_consecutive_matches.prev_match_ids.clear();
            (combinator::Parser::FailParser(FailParser), ParseResults {
                right_data_vec: vec![right_data],
                done: true,
            })
        }
    }
}

impl ParserTrait for ForbidFollows {
    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        panic!("ForbidFollows parser should not be stepped")
    }

    fn next_u8set(&self, bytes: &[u8]) -> U8Set {
        U8Set::none()
    }
}

impl ParserTrait for ForbidFollowsClear {
    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        panic!("ForbidFollowsClear parser should not be stepped")
    }

    fn next_u8set(&self, bytes: &[u8]) -> U8Set {
        U8Set::none()
    }
}

impl ParserTrait for ForbidFollowsCheckNot {
    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        panic!("ForbidFollowsCheckNot parser should not be stepped")
    }

    fn next_u8set(&self, bytes: &[u8]) -> U8Set {
        U8Set::none()
    }
}

pub fn forbid_follows(match_ids: &[usize]) -> ForbidFollows {
    ForbidFollows { match_ids: match_ids.to_vec() }
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
