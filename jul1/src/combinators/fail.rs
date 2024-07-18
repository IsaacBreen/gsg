use crate::{choice, Choice2, CombinatorTrait, IntoCombinator, ParserTrait, Stats};
use crate::parse_state::{RightData, UpData};

#[derive(Debug, Clone, Copy)]
pub struct Fail;

pub struct FailParser;

impl CombinatorTrait for Fail {
    type Parser = FailParser;
    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        (FailParser, vec![], vec![])
    }
}

impl ParserTrait for FailParser {
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>) {
        (vec![], vec![])
    }
    fn collect_stats(&self, stats: &mut Stats) {
        stats.active_parser_type_counts.entry("FailParser".to_string()).and_modify(|c| *c += 1).or_insert(1);
    }
}

pub fn fail() -> Fail {
    Fail
}