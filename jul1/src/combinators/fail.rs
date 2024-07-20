use crate::{choice, Choice2, CombinatorTrait, IntoCombinator, ParseResults, ParserTrait, Stats};
use crate::parse_state::{RightData, UpData};

#[derive(Debug, Clone, Copy)]
pub struct Fail;

#[derive(PartialEq, Eq)]
pub struct FailParser;

impl CombinatorTrait for Fail {
    type Parser = FailParser;
    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        (FailParser, ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            cut: false,
        })
    }
}

impl ParserTrait for FailParser {
    fn step(&mut self, c: u8) -> ParseResults {
        ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            cut: false,
        }
    }
}

pub fn fail() -> Fail {
    Fail
}