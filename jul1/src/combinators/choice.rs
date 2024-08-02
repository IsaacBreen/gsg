use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, eps, Parser, ParseResults, ParserTrait, Squash};
use crate::parse_state::RightData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Choice {
    pub(crate) children: Vec<Rc<Combinator>>,
    pub(crate) greedy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChoiceParser {
    pub(crate) parsers: Vec<Parser>,
    pub(crate) greedy: bool,
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
            let discard_rest = self.greedy && !parse_results.right_data_vec.is_empty() && parse_results.right_data_vec.iter().all(|rd| rd.lookahead_data.partial_lookaheads.is_empty());
            combined_results = combined_results.combine_inplace(parse_results);
            if discard_rest {
                break;
            }
        }

        (
            Parser::ChoiceParser(ChoiceParser { parsers, greedy: self.greedy }),
            combined_results
        )
    }

    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let mut parsers = Vec::new();
        let mut combined_results = ParseResults::empty_finished();

        for (i, child) in self.children.iter().enumerate() {
            let (parser, parse_results) = child.parser_with_steps(right_data.clone(), bytes);
            if !parse_results.done {
                parsers.push(parser);
            }
            // TODO: can't have lookaheads if done.
            let discard_rest = self.greedy && !parse_results.right_data_vec.is_empty() && parse_results.right_data_vec.iter().all(|rd| rd.lookahead_data.partial_lookaheads.is_empty());
            combined_results = combined_results.combine_inplace(parse_results);
            if discard_rest {
                println!("discarding rest: {:?}", self.children[i+1..].to_vec());
                break;
            }
        }

        (
            Parser::ChoiceParser(ChoiceParser { parsers, greedy: self.greedy }),
            combined_results
        )
    }
}

impl ParserTrait for ChoiceParser {
    fn step(&mut self, c: u8) -> ParseResults {
        let mut parse_result = ParseResults::empty_finished();
        let mut discard_rest = false;

        self.parsers.retain_mut(|mut parser| {
            if discard_rest {
                return false;
            }
            let step_result = parser.step(c);
            discard_rest = self.greedy && !step_result.right_data_vec.is_empty() && step_result.right_data_vec.iter().all(|rd| rd.lookahead_data.partial_lookaheads.is_empty());
            let done = step_result.done;
            parse_result.combine(step_result);
            !done
        });

        parse_result.squash();
        parse_result
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        let mut parse_result = ParseResults::empty_finished();
        let mut discard_rest = false;

        self.parsers.retain_mut(|mut parser| {
            if discard_rest {
                return false;
            }
            let step_result = parser.steps(bytes);
            discard_rest = self.greedy && !step_result.right_data_vec.is_empty() && step_result.right_data_vec.iter().all(|rd| rd.lookahead_data.partial_lookaheads.is_empty());
            let done = step_result.done;
            parse_result.combine(step_result);
            !done
        });

        parse_result.squash();
        parse_result
    }

}

pub fn _choice(v: Vec<Combinator>) -> Combinator {
    Choice {
        children: v.into_iter().map(Rc::new).collect(),
        greedy: false,
    }.into()
}

pub fn _choice_greedy(v: Vec<Combinator>) -> Combinator {
    Choice {
        children: v.into_iter().map(Rc::new).collect(),
        greedy: true,
    }.into()
}

#[macro_export]
macro_rules! choice {
    ($($expr:expr),+ $(,)?) => {
        $crate::_choice(vec![$($expr.into()),+])
    };
}

#[macro_export]
macro_rules! choice_greedy {
    ($($expr:expr),+ $(,)?) => {
        $crate::_choice_greedy(vec![$($expr.into()),+])
    };
}

impl From<Choice> for Combinator {
    fn from(value: Choice) -> Self {
        Combinator::Choice(value)
    }
}
