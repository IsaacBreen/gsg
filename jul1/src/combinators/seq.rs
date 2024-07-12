use std::rc::Rc;

use crate::*;
use crate::parse_state::{RightData, UpData};

pub struct Seq2<A, B>
where
    A: CombinatorTrait,
    B: CombinatorTrait,
{
    a: A,
    b: Rc<B>,
}

pub struct Seq2Parser<B, ParserA>
where
    ParserA: ParserTrait,
    B: CombinatorTrait,
{
    a: Option<ParserA>,
    bs: Vec<B::Parser>,
    b: Rc<B>,
    right_data: RightData,
}

impl<A, B> CombinatorTrait for Seq2<A, B>
where
    A: CombinatorTrait,
    B: CombinatorTrait,
{
    type Parser = Seq2Parser<B, A::Parser>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        let (a, right_data_a, up_data_a) = self.a.parser(right_data.clone());
        let (mut bs, mut right_data_bs, mut up_data_bs) = (vec![], vec![], vec![]);
        for right_data_b in right_data_a {
            let (b, right_data_b, up_data_b) = self.b.parser(right_data_b);
            bs.push(b);
            right_data_bs.extend(right_data_b);
            up_data_bs.extend(up_data_b);
        }
        let parser = Seq2Parser {
            a: Some(a),
            bs: bs,
            b: self.b.clone(),
            right_data,
        };
        (parser, right_data_bs, up_data_bs.into_iter().chain(up_data_a).collect())
    }
}

impl<ParserA, B> ParserTrait for Seq2Parser<B, ParserA>
where
    ParserA: ParserTrait,
    B: CombinatorTrait,
{
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>) {
        let (right_data_a, up_data_a) = self.a.as_mut().map(|a| a.step(c)).unwrap_or((vec![], vec![]));
        let (mut right_data_bs, mut up_data_bs) = (vec![], vec![]);
        for b in self.bs.iter_mut() {
            let (right_data_b, up_data_b) = b.step(c);
            right_data_bs.extend(right_data_b);
            up_data_bs.extend(up_data_b);
        }
        for right_data_b in right_data_a {
            let (b, right_data_b, up_data_b) = self.b.parser(right_data_b);
            self.bs.push(b);
            right_data_bs.extend(right_data_b);
            up_data_bs.extend(up_data_b);
        }
        (right_data_bs, up_data_bs.into_iter().chain(up_data_a).collect())
    }
}

pub fn seq2<A, B>(a: A, b: B) -> Seq2<A::Output, B::Output>
where
    A: IntoCombinator,
    B: IntoCombinator,
{
    Seq2 { a: a.into_combinator(), b: Rc::new(b.into_combinator()) }
}

#[macro_export]
macro_rules! seq {
    ($a:expr $(,)?) => {
        $a
    };
    ($a:expr, $($b:expr),+ $(,)?) => {
        $crate::seq2($a, $crate::seq!($($b),+))
    };
}