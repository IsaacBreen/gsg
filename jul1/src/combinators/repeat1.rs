use std::rc::Rc;

use crate::{Choice2, CombinatorTrait, Eps, IntoCombinator, opt, ParseResults, ParserTrait, Stats};
use crate::parse_state::{RightData, UpData};

pub struct Repeat1<A>
where
    A: CombinatorTrait,
{
    a: Rc<A>,
}

pub struct Repeat1Parser<A>
where
    A: CombinatorTrait,
{
    pub(crate) a: Rc<A>,
    pub(crate) a_parsers: Vec<A::Parser>,
    right_data: RightData,
}

impl<A> CombinatorTrait for Repeat1<A>
where
    A: CombinatorTrait,
{
    type Parser = Repeat1Parser<A>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        let (a, right_data_a, up_data_a) = self.a.parser(right_data.clone());
        (Repeat1Parser { a: self.a.clone(), a_parsers: vec![a], right_data }, right_data_a, up_data_a)
    }
}

impl<A> ParserTrait for Repeat1Parser<A>
where
    A: CombinatorTrait,
{
    fn step(&mut self, c: u8) -> ParseResults {
        // TODO: modify this to use the new `cut` field.
        let (mut right_data_as, mut up_data_as) = (vec![], vec![]);
        self.a_parsers.retain_mut(|a_parser| {
            let ParseResults { right_data_vec: right_data_a, up_data_vec: up_data_a , cut} = a_parser.step(c);
            if right_data_a.is_empty() && up_data_a.is_empty() {
                false
            } else {
                right_data_as.extend(right_data_a);
                up_data_as.extend(up_data_a);
                true
            }
        });
        for right_data_a in right_data_as.clone() {
            let (a_parser, right_data_a, up_data_a) = self.a.parser(right_data_a);
            self.a_parsers.push(a_parser);
            right_data_as.extend(right_data_a);
            up_data_as.extend(up_data_a);
        }
        ParseResults {
            right_data_vec: right_data_as,
            up_data_vec: up_data_as,
            cut: false,
        }
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a dyn ParserTrait> + 'a> {
        Box::new(self.a_parsers.iter().map(|a| a as &dyn ParserTrait))
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut dyn ParserTrait> + 'a> {
        Box::new(self.a_parsers.iter_mut().map(|a| a as &mut dyn ParserTrait))
    }
}

pub fn repeat1<A>(a: A) -> Repeat1<A::Output>
where
    A: IntoCombinator,
{
    Repeat1 { a: Rc::new(a.into_combinator()) }
}

pub fn repeat0<A>(a: A) -> Choice2<Repeat1<A::Output>, Eps>
where
    A: IntoCombinator,
{
    opt(repeat1(a.into_combinator()))
}