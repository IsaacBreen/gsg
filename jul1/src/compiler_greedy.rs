use std::collections::HashSet;
use crate::*;

// Only the combinators below are supported by compile_greedy
// use crate::{
//     eps, fail, seq, eat_byte_range,
//     eat_bytestring_choice, eat_char, eat_char_choice, eat_char_negation, eat_char_negation_choice,
//     eat_string, exclude_strings, negative_lookahead,
// };
//
// use crate::{
//     choice_greedy, opt_greedy,
//     repeat0_greedy, repeat1_greedy,
//     repeatn_greedy,
// };

#[derive(Clone)]
pub enum SimpleCombinator {
    Eps,
    Fail,
    Seq(Vec<SimpleCombinator>),
    Choice(Vec<SimpleCombinator>),
    EatU8(U8Set),
    EatBytestringChoice(EatByteStringChoice),
    EatString(EatString),
    NegativeLookahead(Box<SimpleCombinator>),
    Opt(Box<SimpleCombinator>),
    Repeat1(Box<SimpleCombinator>),
}

impl Combinator {
    fn to_simple_combinator(&self) -> SimpleCombinator {
        match self {
            Combinator::Eps(_) => SimpleCombinator::Eps,
            Combinator::Fail(_) => SimpleCombinator::Fail,
            Combinator::Seq(Seq { children }) => SimpleCombinator::Seq(children.iter().map(|x| x.to_simple_combinator()).collect()),
            Combinator::Choice(Choice { children, greedy: true }) => SimpleCombinator::Choice(children.iter().map(|x| x.to_simple_combinator()).collect()),
            Combinator::Choice(Choice { greedy: false, .. }) => panic!("Choice with greedy=false is not supported"),
            Combinator::EatU8(EatU8 { u8set }) => SimpleCombinator::EatU8(u8set.clone()),
            Combinator::EatByteStringChoice(inner) => SimpleCombinator::EatBytestringChoice(inner.clone()),
            Combinator::EatString(inner) => SimpleCombinator::EatString(inner.clone()),
            Combinator::Lookahead(Lookahead { combinator, positive: true, .. }) => SimpleCombinator::NegativeLookahead(Box::new(combinator.to_simple_combinator())),
            Combinator::Opt(inner) => SimpleCombinator::Opt(Box::new(inner.inner.to_simple_combinator())),
            Combinator::Repeat1(Repeat1 { a, greedy: true }) => SimpleCombinator::Repeat1(Box::new(a.to_simple_combinator())),
            Combinator::Repeat1(Repeat1 { greedy: false, .. }) => panic!("Repeat1 with greedy=false is not supported"),
            _ => panic!("Unsupported combinator {:?}", self),
        }
    }

    pub fn compile_greedy(mut self) -> Combinator {
        let mut deferred_cache: HashMap<Deferred, Combinator> = HashMap::new();
        fn compile_greedy_inner(combinator: &mut Combinator, deferred_cache: &mut HashMap<Deferred, Combinator>) {
            // match combinator {
            //
            //     _ => {
            //         combinator.apply_mut(|combinator| {
            //             compile_greedy_inner(combinator, deferred_cache);
            //         });
            //     }
            // }
            todo!()
        }
        compile_greedy_inner(&mut self, &mut deferred_cache);
        self
    }
}