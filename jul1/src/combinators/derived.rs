use std::rc::Rc;
use crate::{_choice, choice, Choice, Combinator, eps, repeat0, seq, symbol};

pub fn opt(a: impl Into<Combinator>) -> Combinator {
    choice!(a, eps())
}

pub fn seprep1(a: impl Into<Combinator>, b: impl Into<Combinator>) -> Combinator {
    let a = symbol(a);
    seq!(&a, repeat0(seq!(b, &a)))
}

pub fn seprep0(a: impl Into<Combinator>, b: impl Into<Combinator>) -> Combinator {
    opt(seprep1(a, b)).into()
}

pub fn repeatn(n: usize, a: impl Into<Combinator>) -> Combinator {
    let a = Rc::new(a.into());
    Choice { children: vec![a.clone(); n] }.into()
}