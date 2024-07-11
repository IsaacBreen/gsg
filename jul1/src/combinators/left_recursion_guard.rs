use std::cell::RefCell;
use std::rc::Rc;
use crate::{CombinatorTrait, DownData, DynCombinator, ParserTrait, seq, seq2, Seq2, Seq2Parser};
use crate::parse_state::{RightData, UpData};

#[derive(Clone)]
pub struct LeftRecursionGuard {
    a: Rc<DynCombinator>,
}

pub enum LeftRecursionGuardParser {
    Done,
    Normal(Vec<Box<dyn ParserTrait>>, Rc<DynCombinator>),
}

impl CombinatorTrait for LeftRecursionGuard {
    type Parser = LeftRecursionGuardParser;

    fn parser(&self, mut right_data: RightData, mut down_data: DownData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        if let Some(to_fail) = &right_data.left_recursion_guard_data.to_fail {
            if std::ptr::eq(to_fail.as_ref(), self.a.as_ref()) {
                return (LeftRecursionGuardParser::Done, vec![], vec![])
            }
        }
        if right_data.left_recursion_guard_data.to_pass.iter().any(|a| std::ptr::eq(a.as_ref(), self.a.as_ref())) {
            return (LeftRecursionGuardParser::Done, vec![right_data], vec![])
        }
        if let Some(to_fail) = &right_data.left_recursion_guard_data.to_fail {
            right_data.left_recursion_guard_data.to_pass.push(to_fail.clone());
        }
        right_data.left_recursion_guard_data.to_fail = Some(self.a.clone());
        let (parser, right_data, up_data) = self.a.parser(right_data, down_data);
        (LeftRecursionGuardParser::Normal(vec![parser], self.a.clone()), right_data, up_data)
    }
}

impl ParserTrait for LeftRecursionGuardParser {
    fn step(&mut self, c: u8, down_data: DownData) -> (Vec<RightData>, Vec<UpData>) {
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
                    right_data0.left_recursion_guard_data.to_pass.push(a.clone());
                    let (parser, right_data0, up_data0) = a.parser(right_data0, down_data.clone());
                    parsers.push(parser);
                    right_data.extend(right_data0);
                    up_data.extend(up_data0);
                }
                (right_data, up_data)
            }
        }
    }
}

pub fn left_recursion_guard(a: Rc<DynCombinator>) -> LeftRecursionGuard {
    LeftRecursionGuard { a }
}
