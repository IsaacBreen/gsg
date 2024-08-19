use crate::{Combinator, CombinatorTrait, Parser, ParseResults, RightData};

#[derive(Debug)]
pub struct Fail;

impl CombinatorTrait for Fail {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(self);
    }

    fn one_shot_parse(&self, _right_ RightData, _bytes: &[u8]) -> UnambiguousParseResults {
        UnambiguousParseResults::Err(UnambiguousParseError::Fail)
    }

    fn parse(&self, _right_ RightData, _bytes: &[u8]) -> (Parser, ParseResults) {
        (
            Parser::FailParser(crate::FailParser),
            ParseResults::empty_finished(),
        )
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
