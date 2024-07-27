use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, Stats};
use crate::parse_state::RightData;

#[derive(PartialEq)]
pub struct Eps;

#[derive(PartialEq)]
pub struct EpsParser;

impl CombinatorTrait for Eps {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        (Parser::Eps(EpsParser), ParseResults {
            right_data_vec: vec![right_data],
            up_data_vec: vec![],
            done: true,
        })
    }
}

impl ParserTrait for EpsParser {
    fn step(&mut self, c: u8) -> ParseResults {
        panic!("EpsParser already consumed")
    }

    fn collect_stats(&self, stats: &mut Stats) {
        todo!()
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Parser> + 'a> {
        todo!()
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut Parser> + 'a> {
        todo!()
    }
}

pub fn eps() -> Combinator {
    Combinator::Eps(Eps)
}
