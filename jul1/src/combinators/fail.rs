use std::any::Any;
use crate::{choice, Choice2, CombinatorTrait, IntoCombinator, ParseResults, ParserTrait, Stats};
use crate::parse_state::{RightData, UpData};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fail;

#[derive(PartialEq)]
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

    fn as_any(&self) -> &dyn Any {
        self
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

    fn dyn_eq(&self, other: &dyn ParserTrait) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn fail() -> Fail {
    Fail
}