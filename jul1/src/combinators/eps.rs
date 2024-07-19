use crate::{choice, Choice2, CombinatorTrait, IntoCombinator, ParseResults, ParserTrait, Stats};
use crate::parse_state::{RightData, UpData};

#[derive(Debug, Clone, Copy)]
pub struct Eps;

pub struct EpsParser;

impl CombinatorTrait for Eps {
    type Parser = EpsParser;
    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        (EpsParser, vec![right_data], vec![])
    }
}

impl ParserTrait for EpsParser {
    fn step(&mut self, c: u8) -> ParseResults {
        ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![]
        }
    }
}

pub fn eps() -> Eps {
    Eps
}

pub fn opt<A>(a: A) -> Choice2<A::Output, Eps>
where
    A: IntoCombinator,
{
    choice!(a.into_combinator(), eps())
}
