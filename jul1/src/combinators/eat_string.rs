use crate::{CombinatorTrait, ParserTrait, U8Set};
use crate::parse_state::{RightData, UpData};

pub struct EatString {
    string: Vec<u8>,
}

pub struct EatStringParser {
    string: Vec<u8>,
    index: usize,
    right_data: Option<RightData>,
}

impl CombinatorTrait for EatString {
    type Parser = EatStringParser;

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        let mut parser = EatStringParser {
            string: self.string.clone(),
            index: 0,
            right_data: Some(right_data),
        };
        (parser, vec![], vec![UpData { u8set: U8Set::from_u8(self.string[0]) }])
    }
}

impl ParserTrait for EatStringParser {
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>) {
        if self.index < self.string.len() {
            if self.string[self.index] == c {
                self.index += 1;
                if self.index == self.string.len() {
                    (vec![self.right_data.take().unwrap()], vec![])
                } else {
                    (vec![], vec![UpData { u8set: U8Set::from_u8(self.string[self.index]) }])
                }
            } else {
                (vec![], vec![])
            }
        } else {
            (vec![], vec![])
        }
    }
}

pub fn eat_string(string: &str) -> EatString {
    EatString {
        string: string.as_bytes().to_vec(),
    }
}