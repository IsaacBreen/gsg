use std::rc::Rc;
use crate::{ParserTrait, CombinatorTrait, VerticalData, HorizontalData};

pub enum DataEnum {
    Horizontal(HorizontalData),
    Vertical(VerticalData),
    None,
}

pub type BruteForceFn = fn(&Vec<u8>, &HorizontalData) -> DataEnum;

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

    fn parser(&self, horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<VerticalData>) {
        let data_enum = (self.f)(&Vec::new(), &horizontal_data);
        let (mut horizontal_data_vec, mut vertical_data_vec) = (vec![], vec![]);
        match data_enum {
            DataEnum::Horizontal(horizontal_data) => {
                horizontal_data_vec.push(horizontal_data);
            },
            DataEnum::Vertical(vertical_data) => {
                vertical_data_vec.push(vertical_data);
            },
            DataEnum::None => {},
        }
        (BruteForceParser { f: self.f.clone(), values: Vec::new(), horizontal_data }, horizontal_data_vec, vertical_data_vec)
    }
}

impl ParserTrait for BruteForceParser {
    fn step(&mut self, c: u8) -> (Vec<HorizontalData>, Vec<VerticalData>) {
        self.values.push(c);
        let data_enum = (self.f)(&self.values, &self.horizontal_data);
        let (mut horizontal_data_vec, mut vertical_data_vec) = (vec![], vec![]);
        match data_enum {
            DataEnum::Horizontal(horizontal_data) => {
                horizontal_data_vec.push(horizontal_data);
            },
            DataEnum::Vertical(vertical_data) => {
                vertical_data_vec.push(vertical_data);
            },
            DataEnum::None => {},
        }
        (horizontal_data_vec, vertical_data_vec)
    }
}


impl DataEnum {
    pub fn to_vecs(self) -> (Vec<HorizontalData>, Vec<VerticalData>) {
        match self {
            DataEnum::Horizontal(horizontal_data) => (vec![horizontal_data], vec![]),
            DataEnum::Vertical(vertical_data) => (vec![], vec![vertical_data]),
            DataEnum::None => (vec![], vec![]),
        }
    }
}

pub fn brute_force(f: BruteForceFn) -> BruteForce {
    BruteForce { f: Rc::new(f) }
}