use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, opt_greedy, Parser, ParseResults, ParserTrait, profile_internal, Squash, U8Set, VecY};
use crate::opt;
use crate::parse_state::RightData;
use crate::VecX;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Repeat1 {
    pub(crate) a: Rc<Combinator>,
    pub(crate) greedy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Repeat1Parser {
    // TODO: store a_parsers in a Vec<Vec<Parser>> where the index of each inner vec is the repetition count of those parsers. That way, we can easily discard earlier parsers when we get a decisively successful parse result.
    a: Rc<Combinator>,
    pub(crate) a_parsers: Vec<Parser>,
    position: usize,
    greedy: bool,
}

impl CombinatorTrait for Repeat1 {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let mut parsers = vec![];
        let (parser, ParseResults { mut right_data_vec, done }) = self.a.parse(right_data.clone(), bytes);
        if !done {
            parsers.push(parser);
        }

        let mut next_right_data = right_data_vec.clone();
        while next_right_data.len() > 0 {
            for new_right_data in std::mem::take(&mut next_right_data) {
                let offset = new_right_data.right_data_inner.position - right_data.right_data_inner.position;
                let (parser, parse_results) = self.a.parse(new_right_data, &bytes[offset..]);
                if !parse_results.done() {
                    parsers.push(parser);
                }
                if self.greedy && parse_results.succeeds_decisively() {
                    right_data_vec.clear();
                    parsers.clear();
                }
                // if !(self.greedy && parse_results.succeeds_decisively()) && parse_results.right_data_vec.len() > 0 && right_data_vec.len() > 0 {
                //     println!("parse_results: {:?}", parse_results);
                // }
                next_right_data.extend(parse_results.right_data_vec);
            }
            if !right_data_vec.is_empty() && !next_right_data.is_empty() {
                let end_pos = right_data.right_data_inner.position + bytes.len();
                let pos1 = right_data_vec[0].right_data_inner.position;
                let pos2 = next_right_data[0].right_data_inner.position;
                if end_pos < pos1 + 1000 || end_pos < pos2 + 1000 {
                    right_data_vec.clear();
                }
            }
            right_data_vec.extend(next_right_data.clone());
        }

        (
            Parser::Repeat1Parser(Repeat1Parser {
                a: self.a.clone(),
                a_parsers: parsers,
                position: right_data.right_data_inner.position + bytes.len(),
                greedy: self.greedy
            }),
            ParseResults {
                right_data_vec,
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
        let mut right_data_as = VecY::new();

        for mut a_parser in std::mem::take(&mut self.a_parsers) {
            let parse_results = a_parser.parse(bytes);
            if !parse_results.done() {
                self.a_parsers.push(a_parser);
            }
            right_data_as.extend(parse_results.right_data_vec);
        }

        let mut i = 0;
        while i < right_data_as.len() {
            let right_data_a = right_data_as[i].clone();
            let offset = right_data_a.right_data_inner.position - self.position;
            let (a_parser, ParseResults { right_data_vec: right_data_a, mut done }) = self.a.parse(right_data_a, &bytes[offset..]);
            right_data_as.extend(right_data_a);
            if !done {
                self.a_parsers.push(a_parser);
            }
            i += 1;
        }

        self.position += bytes.len();

        ParseResults {
            right_data_vec: right_data_as.into(),
            done: self.a_parsers.is_empty(),
        }
    }
}

pub fn repeat1(a: impl Into<Combinator>) -> Combinator {
    profile_internal("repeat1", Repeat1 {
        a: Rc::new(a.into()),
        greedy: false,
    })
}

pub fn repeat1_greedy(a: impl Into<Combinator>) -> Combinator {
    profile_internal("repeat1_greedy", Repeat1 {
        a: Rc::new(a.into()),
        greedy: true,
    })
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
