use crate::{Combinator, CombinatorTrait, Symbol};

pub trait IntoCombinator {
    type Output: CombinatorTrait;
    fn into_combinator(self) -> Self::Output;
}

impl<T: Into<Combinator>> IntoCombinator for T {
    type Output = Combinator;
    fn into_combinator(self) -> Self::Output {
        self.into()
    }
}

