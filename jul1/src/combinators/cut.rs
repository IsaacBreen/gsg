use crate::{choice, Choice2, CombinatorTrait, IntoCombinator, ParseResults, ParserTrait, Stats};
use crate::parse_state::{RightData, UpData};

#[derive(Debug, Clone, Copy)]
pub struct Cut;

pub struct CutParser;

impl CombinatorTrait for Cut {
    type Parser = CutParser;
    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        (CutParser, ParseResults {
            right_data_vec: vec![right_data],
            up_data_vec: vec![],
            cut: true,
        })
    }
}

impl ParserTrait for CutParser {
    fn step(&mut self, c: u8) -> ParseResults {
        ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            cut: false,
        }
    }
}

pub fn cut() -> Cut {
    Cut
}

pub fn opt<A>(a: A) -> Choice2<A::Output, Cut>
where
    A: IntoCombinator,
{
    choice!(a.into_combinator(), cut())
}
