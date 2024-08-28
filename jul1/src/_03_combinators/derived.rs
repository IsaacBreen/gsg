
// src/_03_combinators/derived.rs
use crate::{opt, opt_greedy, repeat0, repeat0_greedy, seq, symbol, Choice};
use crate::{CombinatorTrait, IntoDyn};

pub fn seprep1(a: impl CombinatorTrait, b: impl CombinatorTrait)-> impl CombinatorTrait {
    let a = symbol(a);
    seq!(&a, repeat0(seq!(b, &a)))
}

pub fn seprep0(a: impl CombinatorTrait, b: impl CombinatorTrait)-> impl CombinatorTrait {
    opt(seprep1(a, b))
}

pub fn repeatn<'a>(n: usize, a: impl CombinatorTrait + 'a)-> impl CombinatorTrait {
    let a = symbol(a);
    // Choice { children: Rc::new(vecx![a; n]), greedy: false }.into()
    Choice { children: vec![a; n].into_iter().map(IntoDyn::into_dyn).collect(), greedy: false }
}

pub fn seprep1_greedy(a: impl CombinatorTrait, b: impl CombinatorTrait)-> impl CombinatorTrait {
    let a = symbol(a);
    seq!(&a, repeat0_greedy(seq!(b, &a)))
}

pub fn seprep0_greedy(a: impl CombinatorTrait, b: impl CombinatorTrait)-> impl CombinatorTrait {
    opt_greedy(seprep1_greedy(a, b))
}

pub fn repeatn_greedy<'a>(n: usize, a: impl CombinatorTrait + 'a)-> impl CombinatorTrait {
    let a = symbol(a);
    Choice { children: vec![a; n].into_iter().map(IntoDyn::into_dyn).collect(), greedy: true }
}
