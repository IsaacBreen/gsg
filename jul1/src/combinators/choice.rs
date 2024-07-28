use std::ops::Not;

use crate::{Combinator, CombinatorTrait, fail, Parser, ParseResults, ParserTrait, Squash, Stats};
use crate::parse_state::RightData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Choice {
    pub(crate) a: Box<Combinator>,
    pub(crate) b: Box<Combinator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChoiceParser {
    pub(crate) a: Option<Box<Parser>>,
    pub(crate) b: Option<Box<Parser>>,
}

impl CombinatorTrait for Choice {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let (a, parse_results_a) = self.a.parser(right_data.clone());
        let (b, parse_results_b) = self.b.parser(right_data);
        (
            Parser::ChoiceParser(ChoiceParser { a: parse_results_a.done.not().then_some(Box::new(a)), b: parse_results_b.done.not().then_some(Box::new(b)) }),
            parse_results_a.combine_inplace(parse_results_b)
        )
    }
}

impl ParserTrait for ChoiceParser {
    fn step(&mut self, c: u8) -> ParseResults {
        let mut parse_result = ParseResults::empty_finished();
        if let Some(a) = &mut self.a {
            let parse_result_a = a.step(c);
            if parse_result_a.done {
                self.a = None;
            }
            parse_result.combine(parse_result_a);
        }
        if let Some(b) = &mut self.b {
            let parse_result_b = b.step(c);
            if parse_result_b.done {
                self.b = None;
            }
            parse_result.combine(parse_result_b);
        }
        parse_result.squash();
        parse_result
    }

    fn collect_stats(&self, stats: &mut Stats) {
    }
}

pub fn choice(mut v: Vec<Combinator>) -> Combinator {
    if v.is_empty() {
        fail()
    } else if v.len() == 1 {
        v.pop().unwrap()
    } else {
        let b = v.split_off(v.len() / 2);
        Combinator::Choice(Choice {
            a: Box::new(choice(v)),
            b: Box::new(choice(b)),
        })
    }
}

#[macro_export]
macro_rules! choice {
    ($($a:expr),* $(,)?) => {$crate::choice(vec![$($a.clone()),*])};
}
