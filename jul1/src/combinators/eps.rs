use crate::{choice, Choice2, CombinatorTrait, DownData, ParserTrait};
use crate::parse_state::{RightData, UpData};

#[derive(Debug, Clone, Copy)]
pub struct Eps;

pub struct EpsParser;

impl CombinatorTrait for Eps {
    type Parser = EpsParser;
    fn parser(&self, right_data: RightData, down_data: DownData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        (EpsParser, vec![right_data], vec![])
    }
}

impl ParserTrait for EpsParser {
    fn step(&mut self, c: u8, down_data: DownData) -> (Vec<RightData>, Vec<UpData>) {
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