use crate::{choice, Choice, Combinator, eps};

pub fn opt(a: impl Into<Combinator>) -> Combinator {
    choice!(a, eps())
}
