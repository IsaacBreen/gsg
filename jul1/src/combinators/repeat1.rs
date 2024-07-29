use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, seq, Squash, Stats, symbol};
use crate::combinators::derived::opt;
use crate::parse_state::RightData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Repeat1 {
    pub(crate) a: Rc<Combinator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Repeat1Parser {
    a: Rc<Combinator>,
    pub(crate) a_parsers: Vec<Parser>,
    right_data: RightData,
}

impl CombinatorTrait for Repeat1 {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let (a, mut parse_results) = self.a.parser(right_data.clone());
        assert!(parse_results.right_data_vec.is_empty());
        // parse_results.right_data_vec.clear();
        let a_parsers = if !parse_results.right_data_vec.is_empty() || !parse_results.up_data_vec.is_empty() {
            vec![a.clone()]
        } else {
            vec![]
        };
        (Parser::Repeat1Parser(Repeat1Parser { a: self.a.clone(), a_parsers, right_data }), parse_results)
    }
}

impl ParserTrait for Repeat1Parser {
    fn step(&mut self, c: u8) -> ParseResults {
        let mut right_data_as = vec![];
        let mut up_data_as = vec![];
        let mut new_parsers = vec![];

        for mut a_parser in self.a_parsers.drain(..) {
            let ParseResults { right_data_vec: right_data_a, up_data_vec: up_data_a, mut done} = a_parser.step(c);
            if !done {
                new_parsers.push(a_parser);
            }
            up_data_as.extend(up_data_a);
            right_data_as.extend(right_data_a);
        }

        right_data_as.squash();

        for right_data_a in right_data_as.clone() {
            let (a_parser, ParseResults { right_data_vec: right_data_a, up_data_vec: up_data_a, mut done }) = self.a.parser(right_data_a);
            new_parsers.push(a_parser);
            up_data_as.extend(up_data_a);
            right_data_as.extend(right_data_a);
        }

        self.a_parsers = new_parsers;

        ParseResults {
            right_data_vec: right_data_as,
            up_data_vec: up_data_as,
            done: self.a_parsers.is_empty(),
        }
    }
}

pub fn repeat1(a: impl Into<Combinator>) -> Repeat1 {
    Repeat1 {
        a: Rc::new(a.into()),
    }
}

pub fn repeat0(a: impl Into<Combinator>) -> Combinator {
    opt(repeat1(a)).into()
}

impl From<Repeat1> for Combinator {
    fn from(value: Repeat1) -> Self {
        Combinator::Repeat1(value)
    }
}
