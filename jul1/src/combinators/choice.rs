use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, eps, Parser, ParseResults, ParserTrait, Squash, U8Set};
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
    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let mut parsers = Vec::new();
        let mut combined_results = ParseResults::empty_finished();

        for child in &self.children {
            let (parser, parse_results) = child.parser_with_steps(right_data.clone(), bytes);
            if !parse_results.done {
                parsers.push(parser);
            }
            let discard_rest = self.greedy && parse_results.succeeds_tentatively();
            combined_results = combined_results.merge(parse_results);
            if discard_rest {
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
    fn get_u8set(&self) -> U8Set {
        let mut u8set = U8Set::none();
        for parser in &self.parsers {
            u8set = u8set.union(&parser.get_u8set());
        }
        u8set
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        let mut parse_result = ParseResults::empty_finished();
        let mut discard_rest = false;

        self.parsers.retain_mut(|mut parser| {
            if discard_rest {
                return false;
            }
            let parse_results = parser.steps(bytes);
            discard_rest = self.greedy && parse_results.succeeds_tentatively();
            let done = parse_results.done;
            parse_result.merge_assign(parse_results);
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
