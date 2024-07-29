use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, eps, Parser, ParseResults, ParserTrait, RightData, Stats};

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
        let parser = Parser::SeqParser(SeqParser {
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
}

pub fn _seq(mut v: Vec<Combinator>) -> Combinator {
    if v.is_empty() {
        eps().into()
    } else if v.len() == 1 {
        v.pop().unwrap().into()
    } else {
        let b = v.split_off(v.len() / 2);
        Seq {
            a: Rc::new(_seq(v).into()),
            b: Rc::new(_seq(b).into()),
        }.into()
    }
}

#[macro_export]
macro_rules! seq {
    // Ensure there's at least two sequents
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::_seq(vec![$a.into(), $($rest.into()),*])
    };
}

impl From<Seq> for Combinator {
    fn from(value: Seq) -> Self {
        Combinator::Seq(value)
    }
}
