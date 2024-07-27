// use std::any::Any;
// use crate::*;
//
// #[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
// pub struct ForbidFollowsData {
//     pub prev_match_ids: Vec<String>,
// }
//
// pub struct ForbidFollows {
//     match_ids: Vec<String>,
// }
//
// impl CombinatorTrait for ForbidFollows {
//     type Parser = FailParser;
//     fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
//         right_data.forbidden_consecutive_matches.prev_match_ids = self.match_ids.clone();
//         (FailParser, ParseResults {
//             right_data_vec: vec![right_data],
//             up_data_vec: vec![],
//             done: true,
//         })
//     }
//
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }
//
// pub struct ForbidFollowsClear {}
//
// impl CombinatorTrait for ForbidFollowsClear {
//     type Parser = FailParser;
//     fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
//         right_data.forbidden_consecutive_matches.prev_match_ids.clear();
//         (FailParser, ParseResults {
//             right_data_vec: vec![right_data],
//             up_data_vec: vec![],
//             done: true,
//         })
//     }
//
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }
//
// pub struct ForbidFollowsSet {
//     match_id: String,
// }
//
// impl CombinatorTrait for ForbidFollowsSet {
//     type Parser = FailParser;
//     fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
//         right_data.forbidden_consecutive_matches.prev_match_ids = vec![self.match_id.clone()];
//         (FailParser, ParseResults {
//             right_data_vec: vec![right_data],
//             up_data_vec: vec![],
//             done: true,
//         })
//     }
//
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }
//
// pub struct ForbidFollowsAdd {
//     match_id: String,
// }
//
// impl CombinatorTrait for ForbidFollowsAdd {
//     type Parser = FailParser;
//     fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
//         right_data.forbidden_consecutive_matches.prev_match_ids.push(self.match_id.clone());
//         (FailParser, ParseResults {
//             right_data_vec: vec![right_data],
//             up_data_vec: vec![],
//             done: true,
//         })
//     }
//
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }
//
// pub struct ForbidFollowsCheckNot {
//     match_id: String,
// }
//
// impl CombinatorTrait for ForbidFollowsCheckNot {
//     type Parser = FailParser;
//     fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
//         if right_data.forbidden_consecutive_matches.prev_match_ids.contains(&self.match_id) {
//             (FailParser, ParseResults::empty_finished())
//         } else {
//             (FailParser, ParseResults {
//                 right_data_vec: vec![right_data],
//                 up_data_vec: vec![],
//                 done: true,
//             })
//         }
//     }
//
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }
//
// pub fn forbid_follows(match_ids: &[&str]) -> ForbidFollows {
//     ForbidFollows { match_ids: match_ids.iter().map(|s| s.to_string()).collect() }
// }
//
// pub fn forbid_follows_clear() -> ForbidFollowsClear {
//     ForbidFollowsClear {}
// }
//
// pub fn forbid_follows_set(match_id: &str) -> ForbidFollowsSet {
//     ForbidFollowsSet { match_id: match_id.to_string() }
// }
//
// pub fn forbid_follows_add(match_id: &str) -> ForbidFollowsAdd {
//     ForbidFollowsAdd { match_id: match_id.to_string() }
// }
//
// pub fn forbid_follows_check_not(match_id: &str) -> ForbidFollowsCheckNot {
//     ForbidFollowsCheckNot { match_id: match_id.to_string() }
// }