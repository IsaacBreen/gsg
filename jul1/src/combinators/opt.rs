use crate::*;
use crate::VecX;

#[derive(Debug)]
pub struct Opt<T: CombinatorTrait> {
    pub(crate) inner: T,
    pub(crate) greedy: bool,
}

impl<T: CombinatorTrait> CombinatorTrait for Opt<T> {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (parser, mut parse_results) = self.inner.parse(right_data.clone(), bytes);
        if !(self.greedy && parse_results.succeeds_decisively()) {
            // TODO: remove the condition below. It's a hack.
            if parse_results.right_data_vec.is_empty() {  // TODO: remove this line
                parse_results.right_data_vec.push(right_data);
            }
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

impl From<Opt<Box<Combinator>>> for Combinator {
    fn from(value: Opt<Box<Combinator>>) -> Self {
        Combinator::Opt(value)
    }
}
