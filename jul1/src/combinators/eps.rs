use crate::{Combinator, CombinatorTrait, Parser, ParseResults, RightData};

#[derive(Debug)]
pub struct Eps;

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

pub fn eps() -> Eps {
    Eps
}

// impl From<Eps> for Combinator {
//     fn from(value: Eps) -> Self {
//         Combinator::Eps(value)
//     }
// }
