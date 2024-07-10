use crate::{choice, Choice2, CombinatorTrait, ParserTrait};
use crate::parse_state::{HorizontalData, UpData};

#[derive(Debug, Clone, Copy)]
pub struct Eps;

pub struct EpsParser;

impl CombinatorTrait for Eps {
    type Parser = EpsParser;
    fn parser(&self, horizontal_data: HorizontalData) -> (EpsParser, Vec<HorizontalData>, Vec<UpData>) {
        (EpsParser, vec![horizontal_data], vec![])
    }
}

impl ParserTrait for EpsParser {
    fn step(&mut self, c: u8) -> (Vec<HorizontalData>, Vec<UpData>) {
        (vec![], vec![])
    }
}

pub fn eps() -> Eps {
    Eps
}

pub fn opt<A>(a: A) -> Choice2<A, Eps> where A: CombinatorTrait
{
    choice!(a, eps())
}