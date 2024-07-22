use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct PreventConsecutiveMatchesData {
    pub prev_match_id: Option<String>,
}

pub struct PreventConsecutiveMatches {
    match_id: String,
}

impl CombinatorTrait for PreventConsecutiveMatches {
    type Parser = FailParser;
    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        let maybe_prev_match_id = right_data.prevent_consecutive_matches.prev_match_id.replace(self.match_id.clone());
        if maybe_prev_match_id.map_or(false, |prev_match_id| prev_match_id == self.match_id) {
            (FailParser, ParseResults {
                right_data_vec: vec![right_data],
                up_data_vec: vec![],
                cut: false,
            })
        } else {
            (FailParser, ParseResults::no_match())
        }
    }
}

pub struct PreventConsecutiveMatchesClear {}

impl CombinatorTrait for PreventConsecutiveMatchesClear {
    type Parser = EpsParser;
    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        right_data.prevent_consecutive_matches.prev_match_id = None;
        (EpsParser, ParseResults::no_match())
    }
}

pub fn prevent_consecutive_matches(match_id: &str) -> PreventConsecutiveMatches {
    PreventConsecutiveMatches { match_id: match_id.to_string() }
}

pub fn prevent_consecutive_matches_clear() -> PreventConsecutiveMatchesClear {
    PreventConsecutiveMatchesClear {}
}