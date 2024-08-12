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
        todo!()
    }
}

impl ParserTrait for FastParserWrapper {
    fn get_u8set(&self) -> U8Set {
        todo!()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        todo!()
    }
}

pub fn fast_combinator(expr: Expr) -> FastCombinatorWrapper {
    println!("building regex");
    println!("{:?}", expr);
    let regex = expr.build();
    println!("built regex");
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