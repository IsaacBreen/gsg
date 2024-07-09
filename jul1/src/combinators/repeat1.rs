use std::rc::Rc;
use crate::{CombinatorTrait, ParserTrait};
use crate::parse_state::{HorizontalData, VerticalData};

pub struct Repeat1<A> where A: CombinatorTrait {
    a: Rc<A>,
}

pub struct Repeat1Parser<A> where A: CombinatorTrait {
    a: Rc<A>,
    a_parsers: Vec<A::Parser>,
    horizontal_data: HorizontalData,
}

impl<A> CombinatorTrait for Repeat1<A> where A: CombinatorTrait
{
    type Parser = Repeat1Parser<A>;

    fn parser(&self, horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<VerticalData>) {
        let (a, horizontal_data_a, vertical_data_a) = self.a.parser(horizontal_data.clone());
        (Repeat1Parser { a: self.a.clone(), a_parsers: vec![a], horizontal_data }, horizontal_data_a, vertical_data_a)
    }
}

impl<A> ParserTrait for Repeat1Parser<A> where A: CombinatorTrait
{
    fn step(&mut self, c: u8) -> (Vec<HorizontalData>, Vec<VerticalData>) {
        let (mut horizontal_data_as, mut vertical_data_as) = (vec![], vec![]);
        for a_parser in self.a_parsers.iter_mut() {
            let (horizontal_data_a, vertical_data_a) = a_parser.step(c);
            horizontal_data_as.extend(horizontal_data_a);
            vertical_data_as.extend(vertical_data_a);
        }
        for horizontal_data_a in horizontal_data_as.clone() {
            let (a_parser, horizontal_data_a, vertical_data_a) = self.a.parser(horizontal_data_a);
            self.a_parsers.push(a_parser);
            horizontal_data_as.extend(horizontal_data_a);
            vertical_data_as.extend(vertical_data_a);
        }
        (horizontal_data_as, vertical_data_as)
    }
}

pub fn repeat1<A>(a: A) -> Repeat1<A> where A: CombinatorTrait {
    Repeat1 { a: Rc::new(a) }
}