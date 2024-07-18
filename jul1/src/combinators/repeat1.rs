use std::rc::Rc;

use crate::{Choice2, CombinatorTrait, Eps, IntoCombinator, opt, ParserTrait, Stats};
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
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>) {
        let (mut right_data_as, mut up_data_as) = (vec![], vec![]);
        self.a_parsers.retain_mut(|a_parser| {
            let (right_data_a, up_data_a) = a_parser.step(c);
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
        (right_data_as, up_data_as)
    }

    fn collect_stats(&self, stats: &mut Stats) {
        self.a_parsers.iter().for_each(|a| a.collect_stats(stats));
        stats.active_parser_type_counts.entry("Repeat1Parser".to_string()).and_modify(|c| *c += 1).or_insert(1);
    }
}

pub fn repeat1<A>(a: A) -> Repeat1<A::Output>
where
    A: IntoCombinator,
{
    Repeat1 { a: Rc::new(a.into_combinator()) }
}

pub fn repeat<A>(a: A) -> Choice2<Repeat1<A::Output>, Eps>
where
    A: IntoCombinator,
{
    opt(repeat1(a.into_combinator()))
}