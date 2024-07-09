use crate::{choice, CombinatorTrait, ParserTrait};
use crate::parse_state::{HorizontalData, VerticalData};

pub struct Eps;

pub struct EpsParser;

impl CombinatorTrait for Eps {
    type Parser = EpsParser;
    fn parser(&self, horizontal_data: HorizontalData) -> (EpsParser, Vec<HorizontalData>, Vec<VerticalData>) {
        (EpsParser, vec![horizontal_data], vec![])
    }
}

impl ParserTrait for EpsParser {
    fn step(&mut self, c: u8) -> (Vec<HorizontalData>, Vec<VerticalData>) {
        (vec![], vec![])
    }
}

pub fn eps() -> Eps {
    Eps
}

pub fn opt(a: impl CombinatorTrait) -> impl CombinatorTrait {
    choice!(a, eps())
}