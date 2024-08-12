use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use crate::*;
use crate::tokenizer::finite_automata::{Expr, ExprGroups, Regex, RegexState};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FastCombinatorWrapper {
    pub(crate) regex: Rc<Regex>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FastParserWrapper {
    pub(crate) regex_state: RegexState,
    pub(crate) right_data: Option<RightData>,
}

impl CombinatorTrait for FastCombinatorWrapper {
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let mut regex_state = self.regex.init();
        regex_state.execute(String::from_utf8_lossy(bytes).as_ref());
        if regex_state.failed {
            (Parser::FailParser(FailParser), ParseResults::empty_finished())
        } else if let Some(find_return) = regex_state.find_return {
            let position = find_return.position;
            let mut new_right_data = right_data.clone();
            new_right_data.advance(position);
            (Parser::FastParserWrapper(FastParserWrapper { regex_state, right_data: Some(right_data) }), ParseResults::new_single(new_right_data, false))
        } else {
            (Parser::FastParserWrapper(FastParserWrapper { regex_state, right_data: Some(right_data) }), ParseResults::empty_unfinished())
        }
    }
}

impl ParserTrait for FastParserWrapper {
    fn get_u8set(&self) -> U8Set {
        todo!()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut regex_state = self.regex_state.clone();
        regex_state.execute(String::from_utf8_lossy(bytes).as_ref());
        if regex_state.failed {
            ParseResults::empty_finished()
        } else if let Some(find_return) = regex_state.find_return {
            let position = find_return.position;
            let mut new_right_data = self.right_data.clone().unwrap();
            new_right_data.advance(position);
            ParseResults::new_single(new_right_data, false)
        } else {
            ParseResults::empty_unfinished()
        }
    }
}

pub fn fast_combinator(expr: Expr) -> FastCombinatorWrapper {
    let regex = expr.build();
    FastCombinatorWrapper { regex: Rc::new(regex) }
}

impl From<FastCombinatorWrapper> for Combinator {
    fn from(fast_combinator: FastCombinatorWrapper) -> Self {
        Combinator::Fast(fast_combinator)
    }
}

impl From<Expr> for Combinator {
    fn from(value: Expr) -> Self {
        fast_combinator(value).into()
    }
}