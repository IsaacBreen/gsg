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

    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        let ParseResults { right_data_vec: right_data2, up_data_vec: up_data, cut } = (self.f)(&Vec::new(), &right_data);
        (BruteForceParser { f: self.f.clone(), values: Vec::new(), right_data }, ParseResults {
            right_data_vec: right_data2,
            up_data_vec: up_data,
            cut,
        })
    }
}

impl ParserTrait for BruteForceParser {
    fn step(&mut self, c: u8) -> ParseResults {
        self.values.push(c);
        let ParseResults { right_data_vec: right_data2, up_data_vec: up_data, cut } = (self.f)(&self.values, &self.right_data);
        ParseResults {
            right_data_vec: right_data2,
            up_data_vec: up_data,
            cut,
        }
    }
    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a dyn ParserTrait> + 'a> {
        Box::new(std::iter::empty())
    }
    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut dyn ParserTrait> + 'a> {
        Box::new(std::iter::empty())
    }
}

pub fn brute_force(f: BruteForceFn) -> BruteForce {
    BruteForce { f: Rc::new(f) }
}