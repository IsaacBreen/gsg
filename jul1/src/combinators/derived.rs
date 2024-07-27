use crate::{choice, Combinator, eps};

pub fn opt(a: Combinator) -> Combinator {
    choice(vec![a, eps()])
}
