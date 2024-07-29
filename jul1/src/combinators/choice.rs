use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, eps, Parser, ParseResults, ParserTrait, Squash};
use crate::parse_state::RightData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Choice {
    pub(crate) children: Vec<Rc<Combinator>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChoiceParser {
    pub(crate) parsers: Vec<Parser>,
}

impl CombinatorTrait for Choice {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let mut parsers = Vec::new();
        let mut combined_results = ParseResults::empty_finished();

        for child in &self.children {
            let (parser, parse_results) = child.parser(right_data.clone());
            if !parse_results.done {
                parsers.push(parser);
            }
            combined_results = combined_results.combine_inplace(parse_results);
        }

        (
            Parser::ChoiceParser(ChoiceParser { parsers }),
            combined_results
        )
    }
}

impl ParserTrait for ChoiceParser {
    fn step(&mut self, c: u8) -> ParseResults {
        let mut parse_result = ParseResults::empty_finished();
        let mut remaining_parsers = Vec::new();

        for mut parser in self.parsers.drain(..) {
            let step_result = parser.step(c);
            if !step_result.done {
                remaining_parsers.push(parser);
            }
            parse_result.combine(step_result);
        }

        self.parsers = remaining_parsers;
        parse_result.squash();
        parse_result
    }
}

pub fn _choice(v: Vec<Combinator>) -> Combinator {
    Choice {
        children: v.into_iter().map(Rc::new).collect(),
    }.into()
}

#[macro_export]
macro_rules! choice {
     ($($expr:expr),+ $(,)?) => {
         $crate::_choice(vec![$($expr.into()),+])
     };
 }

impl From<Choice> for Combinator {
    fn from(value: Choice) -> Self {
        Combinator::Choice(value)
    }
}