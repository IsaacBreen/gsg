use std::cell::RefCell;
use std::rc::Rc;
use crate::{CombinatorTrait, DynCombinator, ParserTrait, seq, seq2, Seq2, Seq2Parser};
use crate::parse_state::{RightData, UpData};

#[derive(Clone)]
pub struct LeftRecursionGuard {
    // todo: problem: what if we have `a` somewhere else without a left recursion guard? then we have an oopsie?
    a: Rc<DynCombinator>,
}

pub enum LeftRecursionGuardParser {
    Done,
    Normal(Vec<Box<dyn ParserTrait>>, Rc<DynCombinator>),
}

impl CombinatorTrait for LeftRecursionGuard {
    type Parser = LeftRecursionGuardParser;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        if let Some(skip_on_this_nonterminal_or_fail_on_any_terminal) = &right_data.left_recursion_guard_data.skip_on_this_nonterminal_or_fail_on_any_terminal {
            if std::ptr::eq(skip_on_this_nonterminal_or_fail_on_any_terminal.as_ref(), self.a.as_ref()) {
                // Skip
                return (LeftRecursionGuardParser::Done, vec![right_data], vec![])
            }
        }
        if right_data.left_recursion_guard_data.fail_on_these_nonterminals.iter().any(|a| std::ptr::eq(a.as_ref(), self.a.as_ref())) {
            // Fail
            return (LeftRecursionGuardParser::Done, vec![], vec![])
        }
        // Fail upon encountering the current nonterminal again without consuming.
        right_data.left_recursion_guard_data.fail_on_these_nonterminals.push(self.a.clone());
        let (parser, right_data, up_data) = self.a.parser(right_data);
        (LeftRecursionGuardParser::Normal(vec![parser], self.a.clone()), right_data, up_data)
    }
}

impl ParserTrait for LeftRecursionGuardParser {
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>) {
        match self {
            LeftRecursionGuardParser::Done => (vec![], vec![]),
            LeftRecursionGuardParser::Normal(parsers, a) => {
                let mut right_data = vec![];
                let mut up_data = vec![];
                for parser in parsers.iter_mut() {
                    let (right_data0, up_data0) = parser.step(c);
                    right_data.extend(right_data0);
                    up_data.extend(up_data0);
                }
                for mut right_data0 in right_data.clone() {
                    // All left recursion guard data should have been stripped.
                    assert!(right_data0.left_recursion_guard_data.skip_on_this_nonterminal_or_fail_on_any_terminal.is_none());
                    assert!(right_data0.left_recursion_guard_data.fail_on_these_nonterminals.is_empty());

                    // Now skip the current nonterminal.
                    right_data0.left_recursion_guard_data.skip_on_this_nonterminal_or_fail_on_any_terminal = Some(a.clone());

                    let (parser, right_data0, up_data0) = a.parser(right_data0);
                    parsers.push(parser);
                    right_data.extend(right_data0);
                    up_data.extend(up_data0);
                }
                (right_data, up_data)
            }
        }
    }
}

pub struct LeftRecursionGuardTerminal<A> {
    a: A,
}

pub struct LeftRecursionGuardTerminalParser<A> {
    a: A,
}

impl<A> CombinatorTrait for LeftRecursionGuardTerminal<A>
where
    A: CombinatorTrait
{
    type Parser = LeftRecursionGuardTerminalParser<A::Parser>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        let should_fail = right_data.left_recursion_guard_data.skip_on_this_nonterminal_or_fail_on_any_terminal.is_some();
        let (parser, right_data_vec, mut up_data_vec) = self.a.parser(right_data);
        if should_fail {
            // Force fail on any terminal by emptying the up data.
            up_data_vec = vec![];
            // We don't empty the right data because it isn't really a terminal (or it's a terminal that doesn't consume anything, which is really what we want - to prevent consumption).
        }
        (LeftRecursionGuardTerminalParser { a: parser }, right_data_vec, up_data_vec)
    }
}

impl<A> ParserTrait for LeftRecursionGuardTerminalParser<A>
where
    A: ParserTrait
{
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>) {
        // By this point, at least one byte has been consumed, so we can just pass through.
        // Strip all left recursion guard data.
        let (mut right_data_vec, up_data_vec) = self.a.step(c);
        for right_data in right_data_vec.iter_mut() {
            right_data.left_recursion_guard_data.skip_on_this_nonterminal_or_fail_on_any_terminal = None;
            right_data.left_recursion_guard_data.fail_on_these_nonterminals.clear();
        }
        (right_data_vec, up_data_vec)
    }
}

pub fn left_recursion_guard_terminal<A>(a: A) -> LeftRecursionGuardTerminal<A>
where
    A: CombinatorTrait
{
    LeftRecursionGuardTerminal { a }
}

pub fn left_recursion_guard(a: Rc<DynCombinator>) -> LeftRecursionGuard {
    LeftRecursionGuard { a }
}