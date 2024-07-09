use crate::parse_state::{HorizontalData, VerticalData};

pub trait CombinatorTrait {
    type Parser: ParserTrait;
    fn parser(&self, horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<VerticalData>);
}

pub trait ParserTrait {
    fn step(&mut self, c: u8) -> (Vec<HorizontalData>, Vec<VerticalData>);
}