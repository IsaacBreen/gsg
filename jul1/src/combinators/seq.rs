use std::rc::Rc;
use crate::*;
use crate::parse_state::{HorizontalData, VerticalData};

pub struct Seq2<A, B> where A: CombinatorTrait, B: CombinatorTrait {
    a: A,
    b: Rc<B>,
}

pub struct Seq2Parser<B, ParserA> where ParserA: ParserTrait, B: CombinatorTrait
{
    a: Option<ParserA>,
    bs: Vec<B::Parser>,
    b: Rc<B>,
    horizontal_data: HorizontalData,
}

impl<A, B> CombinatorTrait for Seq2<A, B> where A: CombinatorTrait, B: CombinatorTrait
{
    type Parser = Seq2Parser<B, A::Parser>;

    fn parser(&self, horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<VerticalData>) {
        let (a, horizontal_data_a, vertical_data_a) = self.a.parser(horizontal_data.clone());
        let (mut bs, mut horizontal_data_bs, mut vertical_data_bs) = (vec![], vec![], vec![]);
        for horizontal_data_b in horizontal_data_a {
            let (b, horizontal_data_b, vertical_data_b) = self.b.parser(horizontal_data_b);
            bs.push(b);
            horizontal_data_bs.extend(horizontal_data_b);
            vertical_data_bs.extend(vertical_data_b);
        }
        let parser = Seq2Parser {
            a: Some(a),
            bs: bs,
            b: self.b.clone(),
            horizontal_data,
        };
        (parser, horizontal_data_bs, vertical_data_bs.into_iter().chain(vertical_data_a).collect())
    }
}

impl<ParserA, B> ParserTrait for Seq2Parser<B, ParserA> where ParserA: ParserTrait, B: CombinatorTrait
{
    fn step(&mut self, c: u8) -> (Vec<HorizontalData>, Vec<VerticalData>) {
        let (horizontal_data_a, vertical_data_a) = self.a.as_mut().map(|a| a.step(c)).unwrap_or((vec![], vec![]));
        let (mut horizontal_data_bs, mut vertical_data_bs) = (vec![], vec![]);
        for b in self.bs.iter_mut() {
            let (horizontal_data_b, vertical_data_b) = b.step(c);
            horizontal_data_bs.extend(horizontal_data_b);
            vertical_data_bs.extend(vertical_data_b);
        }
        for horizontal_data_b in horizontal_data_a {
            let (b, horizontal_data_b, vertical_data_b) = self.b.parser(horizontal_data_b);
            self.bs.push(b);
            horizontal_data_bs.extend(horizontal_data_b);
            vertical_data_bs.extend(vertical_data_b);
        }
        (horizontal_data_bs, vertical_data_bs.into_iter().chain(vertical_data_a).collect())
    }
}

pub fn seq2<A, B>(a: A, b: B) -> Seq2<A, B> where A: CombinatorTrait, B: CombinatorTrait
{
    Seq2 { a, b: Rc::new(b) }
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