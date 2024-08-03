use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, opt_greedy, Parser, ParseResults, ParserTrait, Squash, U8Set};
use crate::combinators::derived::opt;
use crate::parse_state::RightData;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Repeat1 {
    pub(crate) a: Rc<Combinator>,
    pub(crate) greedy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Repeat1Parser {
    a: Rc<Combinator>,
    pub(crate) a_parsers: Vec<Parser>,
    position: usize,
    greedy: bool,
}

impl CombinatorTrait for Repeat1 {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        // Not done -> automatically passes
        // Not greedy -> automatically passes
        // Greedy and done -> only passes if a new parser called on right data doesn't have a right data without lookaheads (we can call this a dominating result).
        let mut right_data_as = vec![];
        let mut new_parsers = vec![];
        let mut prev_parsers_all_done = true;

        let (a, parse_results) = self.a.parse(right_data.clone(), bytes);
        right_data_as.extend(parse_results.right_data_vec);
        if !parse_results.done {
            new_parsers.push(a);
        }
        prev_parsers_all_done &= parse_results.done;

        let mut new_right_data = right_data_as.clone();
        while new_right_data.len() > 0 {
            let current_new_right_data = new_right_data;
            new_right_data = vec![];
            let mut current_parsers_all_done = true;
            for right_data_a in current_new_right_data {
                let offset = right_data_a.position - right_data.position;
                let (a, parse_results) = self.a.parse(right_data_a, &bytes[offset..]);
                if self.greedy && prev_parsers_all_done && !parse_results.right_data_vec.is_empty() {
                    // Clear all previous right and up data
                    new_right_data.clear();
                }
                new_right_data.extend(parse_results.right_data_vec);
                if !parse_results.done {
                    new_parsers.push(a);
                }
                current_parsers_all_done &= parse_results.done;
            }
            right_data_as.extend(new_right_data.clone());
            prev_parsers_all_done &= current_parsers_all_done;
        }

        right_data_as.squash();

        let position = right_data.position + bytes.len();

        let done = new_parsers.is_empty();

        (
            Parser::Repeat1Parser(Repeat1Parser {
                a: self.a.clone(),
                a_parsers: new_parsers,
                position,
                greedy: self.greedy
            }),
            ParseResults {
                right_data_vec: right_data_as,
                done,
            }
        )
    }
}

impl ParserTrait for Repeat1Parser {
    fn get_u8set(&self) -> U8Set {
        if self.a_parsers.is_empty() {
            U8Set::none()
        } else {
            self.a_parsers.iter().map(|p| p.get_u8set()).reduce(|acc, e| acc | e).unwrap_or(U8Set::none())
        }
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut right_data_as = vec![];
        let mut new_parsers = vec![];

        for mut a_parser in self.a_parsers.drain(..) {
            let parse_results = a_parser.parse(bytes);
            if !parse_results.done {
                new_parsers.push(a_parser);
            }
            let discard_rest = self.greedy && parse_results.succeeds_tentatively();
            right_data_as.extend(parse_results.right_data_vec);
            if discard_rest {
                break;
            }
        }

        right_data_as.squash();

        let mut i = 0;
        while i < right_data_as.len() {
            let right_data_a = right_data_as[i].clone();
            let offset = right_data_a.position - self.position;
            let (mut a_parser, ParseResults { right_data_vec: right_data_a, mut done }) = self.a.parse(right_data_a, &bytes[offset..]);
            // todo: ??
            right_data_as.extend(right_data_a);
            if !done {
                new_parsers.push(a_parser);
            }
            i += 1;
        }

        self.a_parsers = new_parsers;

        self.position += bytes.len();

        ParseResults {
            right_data_vec: right_data_as,
            done: self.a_parsers.is_empty(),
        }
    }
}

pub fn repeat1(a: impl Into<Combinator>) -> Repeat1 {
    Repeat1 {
        a: Rc::new(a.into()),
        greedy: false,
    }
}

pub fn repeat1_greedy(a: impl Into<Combinator>) -> Repeat1 {
    Repeat1 {
        a: Rc::new(a.into()),
        greedy: true,
    }
}

pub fn repeat0(a: impl Into<Combinator>) -> Combinator {
    opt(repeat1(a)).into()
}

pub fn repeat0_greedy(a: impl Into<Combinator>) -> Combinator {
    opt_greedy(repeat1_greedy(a)).into()
}

impl From<Repeat1> for Combinator {
    fn from(value: Repeat1) -> Self {
        Combinator::Repeat1(value)
    }
}
