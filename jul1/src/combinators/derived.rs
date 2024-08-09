use std::rc::Rc;
use crate::{_choice, choice, Choice, choice_greedy, Combinator, eps, opt, opt_greedy, repeat0, repeat0_greedy, seq, symbol, vecx};
use crate::VecX;

pub fn seprep1<'a>(a: impl Into<Combinator<'a>>, b: impl Into<Combinator<'a>>) -> Combinator<'a> {
    let a = symbol(a);
    seq!(&a, repeat0(seq!(b, &a)))
}

pub fn seprep0<'a>(a: impl Into<Combinator<'a>>, b: impl Into<Combinator<'a>>) -> Combinator<'a> {
    opt(seprep1(a, b)).into()
}

pub fn repeatn<'a>(n: usize, a: impl Into<Combinator<'a>>) -> Combinator<'a> {
    let a: Combinator<'a> = a.into();
    Choice { children: Rc::new(vecx![a; n]), greedy: false }.into()
}

pub fn seprep1_greedy<'a>(a: impl Into<Combinator<'a>>, b: impl Into<Combinator<'a>>) -> Combinator<'a> {
    let a = symbol(a);
    seq!(&a, repeat0_greedy(seq!(b, &a)))
}

pub fn seprep0_greedy<'a>(a: impl Into<Combinator<'a>>, b: impl Into<Combinator<'a>>) -> Combinator<'a> {
    opt_greedy(seprep1_greedy(a, b)).into()
}

pub fn repeatn_greedy<'a>(n: usize, a: impl Into<Combinator<'a>>) -> Combinator<'a> {
    let a: Combinator<'a> = a.into();
    Choice { children: Rc::new(vecx![a; n]), greedy: true }.into()
}