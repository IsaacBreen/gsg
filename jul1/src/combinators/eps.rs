use std::any::Any;
use crate::{choice, Choice2, CombinatorTrait, IntoCombinator, ParseResults, ParserTrait, Stats};
use crate::parse_state::{RightData, UpData};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Eps;

#[derive(PartialEq)]
pub struct EpsParser;

impl CombinatorTrait for Eps {
    type Parser = EpsParser;
    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        (EpsParser, ParseResults {
            right_data_vec: vec![right_data],
            up_data_vec: vec![],
            done: true,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ParserTrait for EpsParser {
    fn step(&mut self, c: u8) -> ParseResults {
        ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
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

pub fn eps() -> Eps {
    Eps
}

pub fn opt<A>(a: A) -> Choice2<A::Output, Eps>
where
    A: IntoCombinator,
{
    choice!(a.into_combinator(), eps())
}
