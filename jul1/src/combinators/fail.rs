use crate::{Combinator, CombinatorTrait, Parser, ParseResults, RightData, ParserTrait, U8Set, UnambiguousParseError, UnambiguousParseResults, ParseResultTrait};

#[derive(Debug)]
pub struct Fail;

#[derive(Debug)]
pub struct FailParser;

impl CombinatorTrait for Fail {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(self);
    }

    fn one_shot_parse(&self, right_data: RightData, _bytes: &[u8]) -> UnambiguousParseResults {
        UnambiguousParseResults::Err(UnambiguousParseError::Fail)
    }

    fn parse(&self, right_data: RightData, _bytes: &[u8]) -> (Parser, ParseResults) {
        (
            Parser::FailParser(crate::FailParser),
            ParseResults::empty_finished(),
        )
    }
}

impl ParserTrait for FailParser {
    fn get_u8set(&self) -> U8Set {
        panic!("fail parser has no u8set")
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        panic!("fail parser has no parse")
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
