use crate::*;
use crate::VecX;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Opt {
    pub(crate) inner: Box<Combinator>,
    pub(crate) greedy: bool,
}

impl CombinatorTrait for Opt {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (parser, mut parse_results) = self.inner.parse(right_data.clone(), bytes);
        if !(self.greedy && parse_results.succeeds_decisively()) {
            parse_results.right_data_vec.push(right_data);
        }
        (parser, parse_results)
    }
}

pub fn opt(a: impl Into<Combinator>) -> Combinator {
    profile_internal("opt", Opt { inner: Box::new(a.into()), greedy: false })
}

pub fn opt_greedy(a: impl Into<Combinator>) -> Combinator {
    profile_internal("opt_greedy", Opt { inner: Box::new(a.into()), greedy: true })
}

impl From<Opt> for Combinator {
    fn from(value: Opt) -> Self {
        Combinator::Opt(value)
    }
}
