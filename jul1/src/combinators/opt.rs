use crate::*;
use crate::VecX;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Opt<'a> {
    pub(crate) inner: Box<Combinator<'a>>,
    pub(crate) greedy: bool,
}

impl CombinatorTrait<'_> for Opt<'_> {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (parser, mut parse_results) = self.inner.parse(right_data.clone(), bytes);
        if !(self.greedy && parse_results.succeeds_decisively()) {
            parse_results.right_data_vec.push(right_data);
        }
        (parser, parse_results)
    }
}

pub fn opt<'a>(a: impl Into<Combinator<'a>>) -> Combinator<'a> {
    profile_internal("opt", Opt { inner: Box::new(a.into()), greedy: false })
}

pub fn opt_greedy<'a>(a: impl Into<Combinator<'a>>) -> Combinator<'a> {
    profile_internal("opt_greedy", Opt { inner: Box::new(a.into()), greedy: true })
}

impl From<Opt<'_>> for Combinator<'_> {
    fn from(value: Opt<'_>) -> Self {
        Combinator::Opt(value)
    }
}