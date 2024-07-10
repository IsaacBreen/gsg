use std::rc::Rc;
use crate::{Choice2, CombinatorTrait, DownData, Eps, opt, ParserTrait, Seq2};
use crate::parse_state::{RightData, UpData};

pub struct Repeat1<A> where A: CombinatorTrait {
    a: Rc<A>,
}

pub struct Repeat1Parser<A> where A: CombinatorTrait {
    a: Rc<A>,
    a_parsers: Vec<A::Parser>,
    right_data: RightData,
}

impl<A> CombinatorTrait for Repeat1<A> where A: CombinatorTrait
{
    type Parser = Repeat1Parser<A>;

    fn parser(&self, right_data: RightData, down_data: DownData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        let (a, right_data_a, up_data_a) = self.a.parser(right_data.clone(), down_data);
        (Repeat1Parser { a: self.a.clone(), a_parsers: vec![a], right_data }, right_data_a, up_data_a)
    }
}

impl<A> ParserTrait for Repeat1Parser<A> where A: CombinatorTrait
{
    fn step(&mut self, c: u8, down_data: DownData) -> (Vec<RightData>, Vec<UpData>) {
        let (mut right_data_as, mut up_data_as) = (vec![], vec![]);
        for a_parser in self.a_parsers.iter_mut() {
            let (right_data_a, up_data_a) = a_parser.step(c, down_data.clone());
            right_data_as.extend(right_data_a);
            up_data_as.extend(up_data_a);
        }
        for right_data_a in right_data_as.clone() {
            let (a_parser, right_data_a, up_data_a) = self.a.parser(right_data_a, down_data.clone());
            self.a_parsers.push(a_parser);
            right_data_as.extend(right_data_a);
            up_data_as.extend(up_data_a);
        }
        (right_data_as, up_data_as)
    }
}

pub fn repeat1<A>(a: A) -> Repeat1<A> where A: CombinatorTrait {
    Repeat1 { a: Rc::new(a) }
}

pub fn repeat<A>(a: A) -> Choice2<Repeat1<A>, Eps> where A: CombinatorTrait
{
    opt(repeat1(a))
}