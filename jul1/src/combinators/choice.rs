use crate::{CombinatorTrait, ParserTrait};
use crate::parse_state::{RightData, UpData};

pub struct Choice2<A, B> where A: CombinatorTrait, B: CombinatorTrait {
    a: A,
    b: B,
}

pub struct Choice2Parser<ParserA, ParserB> where ParserA: ParserTrait, ParserB: ParserTrait {
    a: ParserA,
    b: ParserB,
}

impl<A, B> CombinatorTrait for Choice2<A, B> where A: CombinatorTrait, B: CombinatorTrait {
    type Parser = Choice2Parser<A::Parser, B::Parser>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        let (a, right_data_a, up_data_a) = self.a.parser(right_data.clone());
        let (b, right_data_b, up_data_b) = self.b.parser(right_data);
        (
            Choice2Parser { a, b },
            right_data_a.into_iter().chain(right_data_b).collect(),
            up_data_a.into_iter().chain(up_data_b).collect()
        )
    }
}

impl<A, B> ParserTrait for Choice2Parser<A, B> where A: ParserTrait, B: ParserTrait
{
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>) {
        let (right_data_a, up_data_a) = self.a.step(c);
        let (right_data_b, up_data_b) = self.b.step(c);
        (
            right_data_a.into_iter().chain(right_data_b).collect(),
            up_data_a.into_iter().chain(up_data_b).collect()
        )
    }
}

pub fn choice2<A, B>(a: A, b: B) -> Choice2<A, B> where A: CombinatorTrait, B: CombinatorTrait {
    Choice2 { a, b }
}

#[macro_export]
macro_rules! choice {
    ($a:expr) => {
        $a
    };
    ($a:expr, $($b:expr),+) => {
        $crate::combinators::choice2($a, $crate::choice!($($b),+))
    };
}