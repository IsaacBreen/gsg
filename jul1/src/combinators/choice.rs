use std::ops::Not;
use std::rc::Rc;
use crate::{Combinator, CombinatorTrait, eps, fail, Parser, ParseResults, ParserTrait, Squash, Stats};
use crate::parse_state::RightData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Choice {
    pub children: Vec<Combinator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChoiceParser {
    pub parsers: Vec<Parser>,
}

impl CombinatorTrait for Choice {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let mut parsers = Vec::new();
        let mut parse_results = ParseResults::default();

        for child in &self.children {
            let (parser, results) = child.parser(right_data.clone());
            parsers.push(parser);
            parse_results.combine_inplace(results);
        }

        (Parser::ChoiceParser(ChoiceParser { parsers }), parse_results)
    }
}

impl ParserTrait for ChoiceParser {
    fn step(&mut self, c: u8) -> ParseResults {
        let mut parse_results = ParseResults::default();

        for parser in &mut self.parsers {
            let results = parser.step(c);
            parse_results.combine_inplace(results);
        }

        parse_results
    }
}

pub fn _choice(mut v: Vec<Combinator>) -> Combinator {
    if v.is_empty() {
        eps().into()
    } else if v.len() == 1 {
        v.pop().unwrap()
    } else {
        let b = v.split_off(v.len() / 2);
        Choice {
            a: Box::new(_choice(v)),
            b: Box::new(_choice(b)),
        }.into()
    }
}

#[macro_export]
macro_rules! choice {
    ($($child:expr),+ $(,)?) => {
        $crate::_choice(vec![$($child.into()),*])
    };
}

impl From<Choice> for Combinator {
    fn from(value: Choice) -> Self {
        Combinator::Choice(value)
    }
}
