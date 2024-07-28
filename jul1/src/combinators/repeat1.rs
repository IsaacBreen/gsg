use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, seq, Squash, Stats};
use crate::combinators::derived::opt;
use crate::parse_state::RightData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Repeat1 {
    a: Rc<Combinator>,
    a_parsers: Vec<Combinator>,
    right_data: RightData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Repeat1Parser {
    a: Rc<Combinator>,
    a_parsers: Vec<Parser>,
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

        // right_data_as.squash();

        self.a_parsers = new_parsers;

        ParseResults {
            right_data_vec: right_data_as,
            up_data_vec: up_data_as,
            done: self.a_parsers.is_empty(),
        }
    }

    fn collect_stats(&self, stats: &mut Stats) {
        stats.active_parser_type_counts.entry("Repeat1Parser".to_string()).and_modify(|c| *c += 1).or_insert(1);
        for a_parser in &self.a_parsers {
            a_parser.collect_stats(stats);
        }
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Parser> + 'a> {
        todo!()
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut Parser> + 'a> {
        todo!()
    }
}

pub fn repeat1(a: Combinator) -> Combinator {
    Combinator::Repeat1(Repeat1 {
        a: Rc::new(a),
        a_parsers: vec![],
        right_data: RightData::default(),
    })
}

pub fn repeat0(a: Combinator) -> Combinator {
    opt(repeat1(a))
}

pub fn seprep1(a: Combinator, b: Combinator) -> Combinator {
    seq(vec![a.clone(), repeat0(seq(vec![b, a]))])
}

pub fn seprep0(a: Combinator, b: Combinator) -> Combinator {
    seq(vec![opt(repeat1(seq(vec![a.clone(), b]))), a])
}