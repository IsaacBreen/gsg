use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, eps, Parser, ParseResults, ParserTrait, RightData, Stats};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seq {
    pub children: Vec<Combinator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeqParser {
    pub parsers: Vec<Parser>,
}

impl CombinatorTrait for Seq {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let mut current_right_data = right_data;
        let mut parsers = Vec::new();
        let mut parse_results = ParseResults::default();

        for child in &self.children {
            let (parser, results) = child.parser(current_right_data);
            parsers.push(parser);
            parse_results.combine_inplace(results.clone());
            current_right_data = results.right_data_vec.first().cloned().unwrap_or_default();
        }

        (Parser::SeqParser(SeqParser { parsers }), parse_results)
    }
}

impl ParserTrait for SeqParser {
    fn step(&mut self, c: u8) -> ParseResults {
        let mut parse_results = ParseResults::default();

        for parser in &mut self.parsers {
            let results = parser.step(c);
            parse_results.combine_inplace(results);
        }

        parse_results
    }
}

pub fn _seq(children: Vec<Combinator>) -> Combinator {
    Combinator::Seq(Seq { children })
}

#[macro_export]
macro_rules! seq {
    ($($child:expr),+ $(,)?) => {
        $crate::_seq(vec![$($child.into()),*])
    };
}

impl From<Seq> for Combinator {
    fn from(value: Seq) -> Self {
        Combinator::Seq(value)
    }
}
