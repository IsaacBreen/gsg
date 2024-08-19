use crate::{CombinatorTrait, IntoDyn};
use std::rc::Rc;
use crate::{_choice, choice, Choice, choice_greedy, Combinator, eps, opt, opt_greedy, repeat0, repeat0_greedy, seq, symbol, vecx};
use crate::VecX;
use crate::UnambiguousParseResults;

pub fn seprep1(a: impl CombinatorTrait + 'static, b: impl CombinatorTrait + 'static)-> impl CombinatorTrait {
    let a = symbol(a);
    seq!(&a, repeat0(seq!(b, &a)))
}

pub fn seprep0(a: impl CombinatorTrait + 'static, b: impl CombinatorTrait + 'static)-> impl CombinatorTrait {
    opt(seprep1(a, b))
}

pub fn repeatn(n: usize, a: impl CombinatorTrait + 'static)-> impl CombinatorTrait {
    let a = symbol(a);
    // Choice { children: Rc::new(vecx![a; n]), greedy: false }.into()
    Choice { children: vec![a; n].into_iter().map(IntoDyn::into_dyn).collect(), greedy: false }
}

pub fn seprep1_greedy(a: impl CombinatorTrait + 'static, b: impl CombinatorTrait + 'static)-> impl CombinatorTrait {
    let a = symbol(a);
    seq!(&a, repeat0_greedy(seq!(b, &a)))
}

pub fn seprep0_greedy(a: impl CombinatorTrait + 'static, b: impl CombinatorTrait + 'static)-> impl CombinatorTrait {
    opt_greedy(seprep1_greedy(a, b))
}

pub fn repeatn_greedy(n: usize, a: impl CombinatorTrait + 'static)-> impl CombinatorTrait {
    let a = symbol(a);
    Choice { children: vec![a; n].into_iter().map(IntoDyn::into_dyn).collect(), greedy: true }
}

impl CombinatorTrait for Choice {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        for child in &self.children {
            child.apply(f);
        }
    }

    fn one_shot_parse(&self, right_ RightData, bytes: &[u8]) -> UnambiguousParseResults {
        if self.greedy {
            for child in &self.children {
                let result = child.one_shot_parse(right_data.clone(), bytes);
                if !result.is_empty() {
                    return result;
                }
            }
            UnambiguousParseResults::empty_finished()
        } else {
            let mut results = UnambiguousParseResults::empty_unfinished();
            for child in &self.children {
                let result = child.one_shot_parse(right_data.clone(), bytes);
                results.append(result);
            }
            results
        }
    }

    fn parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        todo!()
    }
}
