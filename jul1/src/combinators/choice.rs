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
}

pub fn _choice(mut v: Vec<Combinator>) -> Choice {
    if v.len() == 2 {
        Choice {
            a: Box::new(v.pop().unwrap()),
            b: Box::new(v.pop().unwrap()),
        }
    } else if v.len() == 3 {
        let c = v.pop().unwrap();
        Choice {
            a: Box::new(v.pop().unwrap()),
            b: Box::new(_choice(v).into()),
        }
    } else if v.len() >= 4 {
        let b = v.split_off(v.len() / 2);
        Choice {
            a: Box::new(_choice(v).into()),
            b: Box::new(_choice(b).into()),
        }.into()
    } else {
        panic!("choice! must have at least 2 arguments");
    }
}

#[macro_export]
macro_rules! choice {
    // Ensure there's at least two choices
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::_choice(vec![$a.into(), $($rest.into()),*])
    };
}

impl From<Choice> for Combinator {
    fn from(value: Choice) -> Self {
        Combinator::Choice(value)
    }
}
