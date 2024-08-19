// src/combinators/choice.rs
use std::any::Any;
use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, eps, Parser, ParseResults, ParserTrait, profile_internal, Squash, U8Set, VecX};
use crate::parse_state::{RightData, ParseResultTrait};

#[derive(Debug)]
pub struct Choice {
    pub(crate) children: VecX<Combinator>,
    pub(crate) greedy: bool,
}

#[derive(Debug)]
pub struct ChoiceParser<'a> {
    pub(crate) parsers: Vec<Parser<'a>>,
    pub(crate) greedy: bool,
}

impl CombinatorTrait for Choice {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let mut parsers = Vec::new();
        let mut combined_results = ParseResults::empty_finished();

        for child in self.children.iter() {
            let (parser, parse_results) = child.parse(right_data.clone(), bytes);
            if !parse_results.done() {
                parsers.push(parser);
            }
            let discard_rest = self.greedy && parse_results.succeeds_decisively();
            combined_results.merge_assign(parse_results);
            if discard_rest {
                break;
            }
        }

        (
            Parser::ChoiceParser(ChoiceParser { parsers, greedy: self.greedy }),
            combined_results
        )
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        for child in self.children.iter() {
            f(child);
        }
    }
}

impl ParserTrait for ChoiceParser<'_> {
    fn get_u8set(&self) -> U8Set {
        let mut u8set = U8Set::none();
        for parser in &self.parsers {
            u8set |= parser.get_u8set();
        }
        u8set
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut parse_result = ParseResults::empty_finished();
        let mut discard_rest = false;

        self.parsers.retain_mut(|mut parser| {
            if discard_rest {
                return false;
            }
            let parse_results = parser.parse(bytes);
            discard_rest = self.greedy && parse_results.succeeds_decisively();
            let done = parse_results.done();
            parse_result.merge_assign(parse_results);
            !done
        });
        parse_result
    }

}

pub fn _choice(v: Vec<Combinator>)-> impl CombinatorTrait {
    Choice {
        children: v.into_iter().collect(),
        greedy: false,
    }
}

pub fn _choice_greedy(v: Vec<Combinator>)-> impl CombinatorTrait {
    profile_internal("choice", Choice {
        children: v.into_iter().collect(),
        greedy: true,
    })
}

// #[macro_export]
// macro_rules! choice {
//     ($($expr:expr),+ $(,)?) => {
//         $crate::_choice(vec![$($expr.into()),+])
//     };
// }
//
// #[macro_export]
// macro_rules! choice_greedy {
//     ($($expr:expr),+ $(,)?) => {
//         $crate::_choice_greedy(vec![$($expr.into()),+])
//     };
// }

// impl From<Choice> for Combinator {
//     fn from(value: Choice) -> Self {
//
//     }
// }