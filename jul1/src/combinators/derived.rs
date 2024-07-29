use crate::{choice, Choice, Combinator, eps};

pub fn opt(a: impl Into<Combinator>) -> Choice {
    choice!(a, eps())
}
