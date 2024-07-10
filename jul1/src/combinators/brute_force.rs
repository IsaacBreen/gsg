use std::rc::Rc;
use crate::{ParserTrait, CombinatorTrait, UpData, HorizontalData};

pub type BruteForceFn = fn(&Vec<u8>, &HorizontalData) -> (Vec<HorizontalData>, Vec<UpData>);

pub struct BruteForce {
    pub f: Rc<BruteForceFn>,
}

pub struct BruteForceParser {
    pub f: Rc<BruteForceFn>,
    pub values: Vec<u8>,
    pub horizontal_data: HorizontalData,
}

impl CombinatorTrait for BruteForce {
    type Parser = BruteForceParser;

    fn parser(&self, horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<UpData>) {
        let (horizontal_data2, up_data) = (self.f)(&Vec::new(), &horizontal_data);
        (BruteForceParser { f: self.f.clone(), values: Vec::new(), horizontal_data }, horizontal_data2, up_data)
    }
}

impl ParserTrait for BruteForceParser {
    fn step(&mut self, c: u8) -> (Vec<HorizontalData>, Vec<UpData>) {
        self.values.push(c);
        let (horizontal_data2, up_data) = (self.f)(&self.values, &self.horizontal_data);
        (horizontal_data2, up_data)
    }
}

pub fn brute_force(f: BruteForceFn) -> BruteForce {
    BruteForce { f: Rc::new(f) }
}