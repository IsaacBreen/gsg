use std::rc::Rc;
use crate::{_choice, choice, Choice, choice_greedy, Combinator, eps, repeat0, repeat0_greedy, seq, symbol};

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

pub fn repeatn(n: usize, a: impl Into<Combinator>) ->Combinator {
    let a = Rc::new(a.into());
    Choice { children: vec![a.clone(); n], greedy: false }.into()
}

pub fn opt_greedy(a: impl Into<Combinator>) -> Combinator {
    choice_greedy!(a, eps())
}

pub fn seprep1_greedy(a: impl Into<Combinator>, b: impl Into<Combinator>) -> Combinator {
    let a = symbol(a);
    seq!(&a, repeat0_greedy(seq!(b, &a)))
}

pub fn seprep0_greedy(a: impl Into<Combinator>, b: impl Into<Combinator>) -> Combinator {
    opt_greedy(seprep1_greedy(a, b)).into()
}

pub fn repeatn_greedy(n: usize, a: impl Into<Combinator>) -> Combinator {
    let a = Rc::new(a.into());
    Choice { children: vec![a.clone(); n], greedy: true }.into()
}```

jul1/src/combinators/tag.rs
