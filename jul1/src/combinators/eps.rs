use crate::{Combinator, CombinatorTrait, Parser, ParseResults, RightData, UnambiguousParseResults, ParseResultTrait, ParserTrait, U8Set};

#[derive(Debug)]
pub struct Eps;

#[derive(Debug)]
pub struct EpsParser;

impl CombinatorTrait for Eps {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(self);
    }

    fn one_shot_parse(&self, right_data: RightData, _bytes: &[u8]) -> UnambiguousParseResults {
        UnambiguousParseResults::Ok(right_data)
    }

    fn parse(&self, right_data: RightData, _bytes: &[u8]) -> (Parser, ParseResults) {
        (
            Parser::FailParser(crate::FailParser),
            ParseResults::new_single(right_data, true),
        )
    }
}

impl ParserTrait for EpsParser {
    fn get_u8set(&self) -> U8Set {
        panic!("eps parser has no u8set")
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        panic!("eps parser has no parse")
    }
}

pub fn eps() -> Eps {
    Eps
}

// impl From<Eps> for Combinator {
//     fn from(value: Eps) -> Self {
//         Combinator::Eps(value)
//     }
// }
