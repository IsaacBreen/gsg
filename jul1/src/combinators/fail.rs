use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::RightData;

#[repr(transparent)]
#[derive(Debug)]
pub struct FailParser;

#[derive(Debug)]
pub struct Fail;

impl CombinatorTrait for Fail {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn parse<'a, 'b, 'c>(&'c self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'b>, ParseResults) where Self: 'a, 'a: 'b {
        (Parser::FailParser(FailParser), ParseResults::empty_finished())
    }
}

impl ParserTrait for FailParser {
    fn get_u8set(&self) -> U8Set {
        panic!("FailParser.get_u8set() called")
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        panic!("FailParser already consumed")
    }
}

pub fn fail() -> Fail {
    Fail
}

// impl From<Fail> for Combinator {
//     fn from(value: Fail) -> Self {
//         Combinator::Fail(value)
//     }
// }
