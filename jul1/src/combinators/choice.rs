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
        let mut parse_results = ParseResults::empty_finished();

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
        let mut parse_results = ParseResults::empty_finished();

        for parser in &mut self.parsers {
            let results = parser.step(c);
            parse_results.combine_inplace(results);
        }

        parse_results
    }
}

pub fn _choice(children: Vec<Combinator>) -> Combinator {
    Combinator::Choice(Choice { children })
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
