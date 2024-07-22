use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct PreventConsecutiveMatchesData {
    pub prev_match_ids: Vec<String>,
}

pub struct PreventConsecutiveMatches {
    match_id: String,
}

impl CombinatorTrait for PreventConsecutiveMatches {
    type Parser = FailParser;
    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        if right_data.prevent_consecutive_matches.prev_match_ids.contains(&self.match_id) {
            (FailParser, ParseResults::no_match())
        } else {
            right_data.prevent_consecutive_matches.prev_match_ids = vec![self.match_id.clone()];
            (FailParser, ParseResults {
                right_data_vec: vec![right_data],
                up_data_vec: vec![],
                cut: false,
            })
        }
    }
}

pub struct PreventConsecutiveMatchesClear {}

impl CombinatorTrait for PreventConsecutiveMatchesClear {
    type Parser = FailParser;
    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        right_data.prevent_consecutive_matches.prev_match_ids.clear();
        (FailParser, ParseResults {
            right_data_vec: vec![right_data],
            up_data_vec: vec![],
            cut: false,
        })
    }
}

pub struct PreventConsecutiveMatchesSet {
    match_id: String,
}

impl CombinatorTrait for PreventConsecutiveMatchesSet {
    type Parser = FailParser;
    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        right_data.prevent_consecutive_matches.prev_match_ids = vec![self.match_id.clone()];
        (FailParser, ParseResults {
            right_data_vec: vec![right_data],
            up_data_vec: vec![],
            cut: false,
        })
    }
}

pub struct PreventConsecutiveMatchesException {
    match_id: String,
}

impl CombinatorTrait for PreventConsecutiveMatchesException {
    type Parser = FailParser;
    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        if right_data.prevent_consecutive_matches.prev_match_ids.contains(&self.match_id) {
            (FailParser, ParseResults {
                right_data_vec: vec![right_data],
                up_data_vec: vec![],
                cut: false,
            })
        } else {
            right_data.prevent_consecutive_matches.prev_match_ids.push(self.match_id.clone());
            (FailParser, ParseResults::no_match())
        }
    }
}

pub struct PreventConsecutiveMatchesAdd {
    match_id: String,
}

impl CombinatorTrait for PreventConsecutiveMatchesAdd {
    type Parser = FailParser;
    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        right_data.prevent_consecutive_matches.prev_match_ids.push(self.match_id.clone());
        (FailParser, ParseResults {
            right_data_vec: vec![right_data],
            up_data_vec: vec![],
            cut: false,
        })
    }
}

pub fn prevent_consecutive_matches(match_id: &str) -> PreventConsecutiveMatches {
    PreventConsecutiveMatches { match_id: match_id.to_string() }
}

pub fn prevent_consecutive_matches_clear() -> PreventConsecutiveMatchesClear {
    PreventConsecutiveMatchesClear {}
}

pub fn prevent_consecutive_matches_set(match_id: &str) -> PreventConsecutiveMatchesSet {
    PreventConsecutiveMatchesSet { match_id: match_id.to_string() }
}

pub fn prevent_consecutive_matches_exception(match_id: &str) -> PreventConsecutiveMatchesException {
    PreventConsecutiveMatchesException { match_id: match_id.to_string() }
}

pub fn prevent_consecutive_matches_add(match_id: &str) -> PreventConsecutiveMatchesAdd {
    PreventConsecutiveMatchesAdd { match_id: match_id.to_string() }
}