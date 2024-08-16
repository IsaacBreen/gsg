use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use crate::*;
use crate::tokenizer::finite_automata::{Expr, ExprGroups, Regex, RegexState};

#[derive(Debug)]
pub struct FastCombinatorWrapper {
    pub(crate) regex: Rc<Regex>,
}

#[derive(Debug)]
pub struct FastParserWrapper {
    pub(crate) regex_state: RegexState,
    pub(crate) right_data: Option<RightData>,
}

impl CombinatorTrait for FastCombinatorWrapper {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn parse<'a, 'b>(&'a self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'b>, ParseResults) where 'a: 'b {
        let mut regex_state = self.regex.init();
        regex_state.execute(bytes);
        if regex_state.failed() {
            (Parser::FailParser(FailParser), ParseResults::empty_finished())
        } else {
            let mut right_data_vec: VecY<RightData> = vecy![];
            let done = regex_state.done();
            if let Some(new_match) = regex_state.prev_match() {
                let mut new_right_data = right_data.clone();
                let position = new_match.position;
                new_right_data.advance(position);
                right_data_vec.push(new_right_data);
            }
            (Parser::FastParserWrapper(FastParserWrapper { regex_state, right_data: Some(right_data) }), ParseResults::new(right_data_vec, done))
        }
    }
}

impl ParserTrait for FastParserWrapper {
    fn get_u8set(&self) -> U8Set {
        self.regex_state.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut regex_state = &mut self.regex_state;
        let prev_match = regex_state.prev_match();
        regex_state.execute(bytes);
        if regex_state.failed() {
            ParseResults::empty_finished()
        } else {
            let mut right_data_vec: VecY<RightData> = vecy![];
            let done = regex_state.done();
            if let Some(new_match) = regex_state.prev_match() {
                if Some(new_match) != prev_match {
                    let mut new_right_data = self.right_data.clone().unwrap();
                    let position = new_match.position;
                    new_right_data.advance(position);
                    right_data_vec.push(new_right_data);
                }
            }
            ParseResults::new(right_data_vec, done)
        }
    }
}

pub fn fast_combinator(expr: Expr) -> FastCombinatorWrapper {
    let regex = expr.build();
    FastCombinatorWrapper { regex: Rc::new(regex) }
}

// impl From<FastCombinatorWrapper> for Combinator {
//     fn from(fast_combinator: FastCombinatorWrapper) -> Self {
//         Combinator::Fast(fast_combinator)
//     }
// }
//
impl From<Expr> for FastCombinatorWrapper {
    fn from(value: Expr) -> Self {
        fast_combinator(value)
    }
}
