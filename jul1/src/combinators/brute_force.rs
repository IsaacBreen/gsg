use std::rc::Rc;

use crate::{CombinatorTrait, ParseResults, ParserTrait, RightData, Stats, UpData};

pub type BruteForceFn = fn(&Vec<u8>, &RightData) -> ParseResults;

pub struct BruteForce {
    pub f: Rc<BruteForceFn>,
}

pub struct BruteForceParser {
    pub f: Rc<BruteForceFn>,
    pub values: Vec<u8>,
    pub right_data: RightData,
}

impl CombinatorTrait for BruteForce {
    type Parser = BruteForceParser;

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        let ParseResults(right_data2, up_data) = (self.f)(&Vec::new(), &right_data);
        (BruteForceParser { f: self.f.clone(), values: Vec::new(), right_data }, right_data2, up_data)
    }
}

impl ParserTrait for BruteForceParser {
    fn step(&mut self, c: u8) -> ParseResults {
        self.values.push(c);
        let ParseResults(right_data2, up_data) = (self.f)(&self.values, &self.right_data);
        ParseResults(right_data2, up_data)
    }
}

pub fn brute_force(f: BruteForceFn) -> BruteForce {
    BruteForce { f: Rc::new(f) }
}