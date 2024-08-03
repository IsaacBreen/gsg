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
        let mut right_data_as = Vec::new();
        let mut new_parsers = Vec::new();
        let mut prev_parsers_all_done = true;

        let process_parse_results = |parse_results: ParseResults, new_parsers: &mut Vec<Parser>, right_data_as: &mut Vec<RightData>| {
            right_data_as.extend(parse_results.right_data_vec);
            if !parse_results.done {
                new_parsers.push(parse_results.parser);
            }
            parse_results.done
        };

        let (a, parse_results) = self.a.parse(right_data.clone(), bytes);
        prev_parsers_all_done &= process_parse_results(parse_results, &mut new_parsers, &mut right_data_as);

        while !right_data_as.is_empty() {
            let current_new_right_data = std::mem::take(&mut right_data_as);
            let mut current_parsers_all_done = true;

            for right_data_a in current_new_right_data {
                let offset = right_data_a.position - right_data.position;
                let (a, parse_results) = self.a.parse(right_data_a, &bytes[offset..]);

                if self.greedy && prev_parsers_all_done && !parse_results.right_data_vec.is_empty() {
                    right_data_as.clear();
                }

                current_parsers_all_done &= process_parse_results(parse_results, &mut new_parsers, &mut right_data_as);
            }

            prev_parsers_all_done &= current_parsers_all_done;
        }

        right_data_as.squash();

        (
            Parser::Repeat1Parser(Repeat1Parser {
                a: self.a.clone(),
                a_parsers: new_parsers,
                position: right_data.position + bytes.len(),
                greedy: self.greedy
            }),
            ParseResults {
                right_data_vec: right_data_as,
                done: new_parsers.is_empty(),
            }
        )
    }
}

impl ParserTrait for Repeat1Parser {
    fn get_u8set(&self) -> U8Set {
        self.a_parsers.iter().fold(U8Set::none(), |acc, p| acc | p.get_u8set())
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut right_data_as = Vec::new();
        let mut new_parsers = Vec::new();

        for mut a_parser in self.a_parsers.drain(..) {
            let parse_results = a_parser.parse(bytes);
            if !parse_results.done {
                new_parsers.push(a_parser);
            }
            right_data_as.extend(parse_results.right_data_vec);
            if self.greedy && parse_results.succeeds_tentatively() {
                break;
            }
        }

        right_data_as.squash();

        for right_data_a in right_data_as.clone() {
            let offset = right_data_a.position - self.position;
            let (a_parser, parse_results) = self.a.parse(right_data_a, &bytes[offset..]);
            right_data_as.extend(parse_results.right_data_vec);
            if !parse_results.done {
                new_parsers.push(a_parser);
            }
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
    Repeat1 { a: Rc::new(a.into()), greedy: false }
}

pub fn repeat1_greedy(a: impl Into<Combinator>) -> Repeat1 {
    Repeat1 { a: Rc::new(a.into()), greedy: true }
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