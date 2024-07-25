use std::any::Any;
use crate::{choice, Choice2, CombinatorTrait, IntoCombinator, ParseResults, ParserTrait, Stats};
use crate::parse_state::{RightData, UpData};

#[derive(Debug, Clone, Copy)]
pub struct Cut;

#[derive(PartialEq)]
pub struct CutParser;

impl CombinatorTrait for Cut {
    type Parser = CutParser;
    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        (CutParser, ParseResults {
            right_data_vec: vec![right_data],
            up_data_vec: vec![],
            cut: true,
            done: true,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ParserTrait for CutParser {
    fn step(&mut self, c: u8) -> ParseResults {
        ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            cut: false,
            done: true,
        }
    }

    fn dyn_eq(&self, other: &dyn ParserTrait) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
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
