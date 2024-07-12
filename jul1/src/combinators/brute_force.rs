use std::rc::Rc;
use crate::{ParserTrait, CombinatorTrait, UpData, RightData, DownData};

pub type BruteForceFn = fn(&Vec<u8>, &RightData) -> (Vec<RightData>, Vec<UpData>);

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

    fn parser(&self, right_data: RightData, down_data: DownData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        let (right_data2, up_data) = (self.f)(&Vec::new(), &right_data);
        (BruteForceParser { f: self.f.clone(), values: Vec::new(), right_data }, right_data2, up_data)
    }
}

impl ParserTrait for BruteForceParser {
    fn step(&mut self, c: u8, down_data: DownData) -> (Vec<RightData>, Vec<UpData>) {
        self.values.push(c);
        let (right_data2, up_data) = (self.f)(&self.values, &self.right_data);
        (right_data2, up_data)
    }
}

pub fn brute_force(f: BruteForceFn) -> BruteForce {
    BruteForce { f: Rc::new(f) }
}