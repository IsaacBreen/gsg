use crate::CombinatorTrait;
use std::rc::Rc;
use crate::{_choice, choice, Choice, choice_greedy, Combinator, eps, opt, opt_greedy, repeat0, repeat0_greedy, seq, symbol, vecx};
use crate::VecX;

pub fn seprep1(a: impl CombinatorTrait, b: impl CombinatorTrait)-> impl CombinatorTrait {
    let a = symbol(a);
    seq!(&a, repeat0(seq!(b, &a)))
}

pub fn seprep0(a: impl CombinatorTrait, b: impl CombinatorTrait)-> impl CombinatorTrait {
    opt(seprep1(a, b)).into()
}

pub fn repeatn(n: usize, a: impl CombinatorTrait)-> impl CombinatorTrait {
    let a = symbol(a);
    // Choice { children: Rc::new(vecx![a; n]), greedy: false }.into()
    Choice { children: Rc::new(vec![a; n].into_iter().map(|x| Combinator::Symbol(x)).collect()), greedy: false }.into()
}

pub fn seprep1_greedy(a: impl CombinatorTrait, b: impl CombinatorTrait)-> impl CombinatorTrait {
    let a = symbol(a);
    seq!(&a, repeat0_greedy(seq!(b, &a)))
}

pub fn seprep0_greedy(a: impl CombinatorTrait, b: impl CombinatorTrait)-> impl CombinatorTrait {
    opt_greedy(seprep1_greedy(a, b)).into()
}

pub fn repeatn_greedy(n: usize, a: impl CombinatorTrait)-> impl CombinatorTrait {
    let a = symbol(a);
    Choice { children: Rc::new(vec![a; n].into_iter().map(|x| Combinator::Symbol(x)).collect()), greedy: true }.into()
}
