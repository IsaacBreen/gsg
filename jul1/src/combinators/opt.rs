use crate::*;
use crate::VecX;

#[derive(Debug)]
pub struct Opt<T: CombinatorTrait> {
    pub(crate) inner: T,
    pub(crate) greedy: bool,
}

impl<T: CombinatorTrait + 'static> CombinatorTrait for Opt<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(&self.inner);
    }

    fn parse<'a, 'b>(&'a self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'b>, ParseResults) where 'a: 'b {
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

pub fn opt<T: IntoCombinator>(a: T) -> Opt<T::Output> {
    Opt { inner: a.into_combinator(), greedy: false }
}

pub fn opt_greedy(a: impl IntoCombinator + 'static)-> impl CombinatorTrait {
    profile_internal("opt_greedy", Opt { inner: Box::new(a.into_combinator()), greedy: true })
}

// impl From<Opt<Box<Combinator>>> for Combinator {
//     fn from(value: Opt<Box<Combinator>>) -> Self {
//         Combinator::Opt(*Box::new(value))
//     }
// }
