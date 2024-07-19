use crate::{choice, Choice2, CombinatorTrait, IntoCombinator, ParseResults, ParserTrait, Stats};
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
    fn step(&mut self, c: u8) -> ParseResults {
        ParseResults(vec![], vec![])
    }
}

pub fn fail() -> Fail {
    Fail
}