use std::ops::Not;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, Squash, Stats};
use crate::parse_state::RightData;

#[derive(PartialEq)]
pub struct Choice {
    pub(crate) a: Box<Combinator>,
    pub(crate) b: Box<Combinator>,
}

#[derive(PartialEq)]
pub struct ChoiceParser {
    pub(crate) a: Option<Box<Parser>>,
    pub(crate) b: Option<Box<Parser>>,
}

impl CombinatorTrait for Choice {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let (a, parse_results_a) = self.a.parser(right_data.clone());
        let (b, parse_results_b) = self.b.parser(right_data);
        (
            Parser::Choice(ChoiceParser { a: parse_results_a.done.not().then_some(Box::new(a)), b: parse_results_b.done.not().then_some(Box::new(b)) }),
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
        todo!()
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Parser> + 'a> {
        todo!()
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut Parser> + 'a> {
        todo!()
    }
}

pub fn choice_from_vec(v: Vec<Combinator>) -> Combinator { todo!() }

pub fn choice(v: Vec<Combinator>) -> Combinator {
    todo!()
}

#[macro_export]
macro_rules! choice {
    ($($a:expr),* $(,)?) => {$crate::choice_from_vec(&[$($a),*])};
}
