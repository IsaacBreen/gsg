// todo: does this behave properly with double consecutive left recursion?
use std::cell::RefCell;
use std::rc::Rc;
use crate::{CombinatorTrait, DownData, DynCombinator, IntoCombinator, ParserTrait, seq, seq2, Seq2, Seq2Parser};
use crate::parse_state::{RightData, UpData};

pub struct LeftRecursionGuard<A> where A: CombinatorTrait {
    // todo: problem: what if we have `a` somewhere else without a left recursion guard? then we have an oopsie?
    pub a: Rc<A>,
}

impl<A> Clone for LeftRecursionGuard<A> where A: CombinatorTrait {
    fn clone(&self) -> Self {
        LeftRecursionGuard { a: self.a.clone() }
    }
}

pub enum LeftRecursionGuardParser<A> where A: CombinatorTrait {
    Done,
    Normal(Vec<A::Parser>, Rc<A>),
}

impl<A> CombinatorTrait for LeftRecursionGuard<A> where A: CombinatorTrait {
    type Parser = LeftRecursionGuardParser<A>;

    fn parser(&self, right_data: RightData, mut down_data: DownData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        if let Some(skip_on_this_nonterminal_or_fail_on_any_terminal) = down_data.left_recursion_guard_data.skip_on_this_nonterminal_or_fail_on_any_terminal {
            if std::ptr::eq(skip_on_this_nonterminal_or_fail_on_any_terminal, Rc::as_ptr(&self.a) as *const u8) {
                // Skip
                // Strip all left recursion guard data.
                // down_data.left_recursion_guard_data.skip_on_this_nonterminal_or_fail_on_any_terminal = None;
                // down_data.left_recursion_guard_data.fail_on_these_nonterminals.clear();
                return (LeftRecursionGuardParser::Done, vec![right_data], vec![])
            }
        }
        if down_data.left_recursion_guard_data.fail_on_these_nonterminals.iter().any(|a| std::ptr::eq(*a, Rc::as_ptr(&self.a) as *const u8)) {
            // Fail
            return (LeftRecursionGuardParser::Done, vec![], vec![])
        }
        // Fail upon encountering the current nonterminal again without consuming.
        down_data.left_recursion_guard_data.fail_on_these_nonterminals.push(Rc::as_ptr(&self.a) as *const u8);
        let (parser, mut right_data_vec, mut up_data_vec) = self.a.parser(right_data, down_data.clone());
        let mut parsers = vec![parser];
        for right_data0 in right_data_vec.clone() {
            down_data.left_recursion_guard_data.fail_on_these_nonterminals.pop();
            down_data.left_recursion_guard_data.skip_on_this_nonterminal_or_fail_on_any_terminal = Some(Rc::as_ptr(&self.a) as *const u8);
            let (parser_new, right_data_vec_new, up_data_vec_new) = self.a.parser(right_data0.clone(), down_data.clone());
            parsers.push(parser_new);
            right_data_vec.extend(right_data_vec_new);
            // We're supposed to fail on any terminal.
            up_data_vec.retain(|up_data| !up_data.left_recursion_guard_data.did_skip);
            // up_data_vec.extend(up_data_vec_new);
        }
        (LeftRecursionGuardParser::Normal(parsers, self.a.clone()), right_data_vec, up_data_vec)
    }
}

impl<A> ParserTrait for LeftRecursionGuardParser<A> where A: CombinatorTrait {
    fn step(&mut self, c: u8, mut down_data: DownData) -> (Vec<RightData>, Vec<UpData>) {
        match self {
            LeftRecursionGuardParser::Done => (vec![], vec![]),
            LeftRecursionGuardParser::Normal(parsers, a) => {
                let mut right_data = vec![];
                let mut up_data = vec![];
                for parser in parsers.iter_mut() {
                    let (right_data0, up_data0) = parser.step(c, down_data.clone());
                    right_data.extend(right_data0);
                    up_data.extend(up_data0);
                }
                for mut right_data0 in right_data.clone() {
                    // Now skip the current nonterminal.
                    down_data.left_recursion_guard_data.skip_on_this_nonterminal_or_fail_on_any_terminal = Some(Rc::as_ptr(&a) as *const u8);

                    let (parser, right_data0, up_data0) = a.parser(right_data0, down_data.clone());
                    parsers.push(parser);
                    right_data.extend(right_data0);
                    // Discard up data since it indicates we have consumed a terminal, and we're supposed to fail on any terminal.
                    // up_data.extend(up_data0);
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

    fn parser(&self, right_data: RightData, down_data: DownData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        let should_fail = down_data.left_recursion_guard_data.skip_on_this_nonterminal_or_fail_on_any_terminal.is_some();
        let (parser, right_data_vec, mut up_data_vec) = self.a.parser(right_data, down_data);
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
    fn step(&mut self, c: u8, mut down_data: DownData) -> (Vec<RightData>, Vec<UpData>) {
        // By this point, at least one byte has been consumed, so we can just pass through.
        // Strip all left recursion guard data.
        let (mut right_data_vec, up_data_vec) = self.a.step(c, down_data.clone());
        for right_data in right_data_vec.iter_mut() {
            down_data.left_recursion_guard_data.skip_on_this_nonterminal_or_fail_on_any_terminal = None;
            down_data.left_recursion_guard_data.fail_on_these_nonterminals.clear();
        }
        (right_data_vec, up_data_vec)
    }
}

pub fn left_recursion_guard_terminal<A>(a: A) -> LeftRecursionGuardTerminal<A::Output> where A: IntoCombinator {
    LeftRecursionGuardTerminal { a: a.into_combinator() }
}

pub fn left_recursion_guard<A>(a: A) -> LeftRecursionGuard<A::Output>
where A: IntoCombinator {
    LeftRecursionGuard { a: a.into_combinator().into() }
}