use std::rc::Rc;
use crate::{ParserTrait, CombinatorTrait, VerticalData, HorizontalData};

pub struct BruteForce<F: Fn(&Vec<u8>, &HorizontalData) -> (Vec<HorizontalData>, Vec<VerticalData>)> {
    pub f: Rc<F>,
}

pub struct BruteForceParser<F: Fn(&Vec<u8>, &HorizontalData) -> (Vec<HorizontalData>, Vec<VerticalData>)> {
    pub f: Rc<F>,
    pub values: Vec<u8>,
    pub horizontal_data: HorizontalData,
}

impl<F: Fn(&Vec<u8>, &HorizontalData) -> (Vec<HorizontalData>, Vec<VerticalData>) + 'static> CombinatorTrait for BruteForce<F> {
    type Parser = BruteForceParser<F>;

    fn parser(&self, horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<VerticalData>) {
        let (horizontal_data2, vertical_data) = (self.f)(&Vec::new(), &horizontal_data);
        (BruteForceParser { f: self.f.clone(), values: Vec::new(), horizontal_data }, horizontal_data2, vertical_data)
    }
}

impl<F: Fn(&Vec<u8>, &HorizontalData) -> (Vec<HorizontalData>, Vec<VerticalData>)> ParserTrait for BruteForceParser<F> {
    fn step(&mut self, c: u8) -> (Vec<HorizontalData>, Vec<VerticalData>) {
        self.values.push(c);
        let (horizontal_data2, vertical_data) = (self.f)(&self.values, &self.horizontal_data);
        (horizontal_data2, vertical_data)
    }
}

pub fn brute_force<F: Fn(&Vec<u8>, &HorizontalData) -> (Vec<HorizontalData>, Vec<VerticalData>)>(f: F) -> BruteForce<F> {
    BruteForce { f: Rc::new(f) }
}