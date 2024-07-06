use crate::{choice2, Choice2, Combinator, Eps, eps};

pub fn opt<A>(a: A) -> Choice2<A, Eps>
where
    A: Combinator,
{
    choice2(a, eps())
}