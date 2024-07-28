use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, RightData, Stats};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seq {
    a: Rc<Combinator>,
    b: Rc<Combinator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeqParser {
    a: Option<Box<Parser>>,
    bs: Vec<Parser>,
    b: Rc<Combinator>,
    right_data: RightData,
}

impl CombinatorTrait for Seq {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let (a, parse_results_a) = self.a.parser(right_data.clone());
        let mut a = (!parse_results_a.done).then_some(Box::new(a));
        let (mut bs, mut right_data_bs, mut up_data_bs) = (vec![], vec![], vec![]);
        for right_data_b in parse_results_a.right_data_vec {
            let (b, ParseResults { right_data_vec: right_data_b, up_data_vec: up_data_b, done }) = self.b.parser(right_data_b);
            if !done {
                bs.push(b);
            }
            up_data_bs.extend(up_data_b);
            right_data_bs.extend(right_data_b);
        }
        let done = a.is_none() && bs.is_empty();
        let parser = Parser::Seq(SeqParser {
            a,
            bs,
            b: self.b.clone(),
            right_data,
        });
        (parser, ParseResults {
            right_data_vec: right_data_bs,
            up_data_vec: up_data_bs.into_iter().chain(parse_results_a.up_data_vec).collect(),
            done,
        })
    }
}

impl ParserTrait for SeqParser {
    fn step(&mut self, c: u8) -> ParseResults {
        let mut right_data_a = vec![];
        let mut up_data_a = vec![];

        if let Some(a) = &mut self.a {
            let ParseResults { right_data_vec, up_data_vec, done: done_a } = a.step(c);
            right_data_a = right_data_vec;
            up_data_a = up_data_vec;
            if done_a {
                self.a = None;
            }
        }

        let mut right_data_bs = vec![];
        let mut up_data_bs = vec![];
        let mut new_bs = vec![];

        for mut b in self.bs.drain(..) {
            let ParseResults { right_data_vec, up_data_vec, done } = b.step(c);
            if !done {
                new_bs.push(b);
            }
            up_data_bs.extend(up_data_vec);
            right_data_bs.extend(right_data_vec);
        }

        for right_data_b in right_data_a {
            let (b, ParseResults { right_data_vec: right_data_b, up_data_vec: up_data_b, done}) = self.b.parser(right_data_b);
            if !done {
                new_bs.push(b);
            }
            up_data_bs.extend(up_data_b);
            right_data_bs.extend(right_data_b);
        }

        self.bs = new_bs;

        ParseResults {
            right_data_vec: right_data_bs,
            up_data_vec: up_data_bs.into_iter().chain(up_data_a).collect(),
            done: self.a.is_none() && self.bs.is_empty(),
        }
    }

    fn collect_stats(&self, stats: &mut Stats) {
        stats.active_parser_type_counts.entry("SeqParser".to_string()).and_modify(|c| *c += 1).or_insert(1);
        if let Some(a) = &self.a {
            a.collect_stats(stats);
        }
        for b in &self.bs {
            b.collect_stats(stats);
        }
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Parser> + 'a> {
        todo!()
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut Parser> + 'a> {
        todo!()
    }
}

pub fn seq(v: Vec<Combinator>) -> Combinator {
    todo!()
}

#[macro_export]
macro_rules! seq {
    ($($a:expr),* $(,)?) => {$crate::seq(vec![$($a.into()),*])};
}