use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use crate::*;
use crate::fast_combinator::{FastCombinator, FastCombinatorResult};
use crate::FastCombinatorTrait;
use crate::tokenizer::finite_automata::ExprGroups;

pub struct FastCombinatorWrapper {
    pub(crate) fast: Rc<FastCombinator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FastParserWrapper {
    pub(crate) fast: FastParser,
    pub(crate) right_data: Option<RightData>,
}

impl Debug for FastCombinatorWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FastParser")
            .finish_non_exhaustive()
    }
}

impl Clone for FastCombinatorWrapper {
    fn clone(&self) -> Self {
        Self {
            fast: self.fast.clone(),
        }
    }
}

impl PartialEq for FastCombinatorWrapper {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.fast, &other.fast)
    }
}

impl Eq for FastCombinatorWrapper {}

impl Hash for FastCombinatorWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.fast).hash(state);
    }
}

impl CombinatorTrait for FastCombinatorWrapper {
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        match self.fast.parse(bytes) {
            FastCombinatorResult::Success(len) => {
                right_data.advance(len);
                (Parser::FailParser(FailParser), ParseResults::new_single(right_data, true))
            }
            FastCombinatorResult::Failure => {
                (Parser::FailParser(FailParser), ParseResults::empty_finished())
            }
            FastCombinatorResult::Incomplete(parser, consumed) => {
                (Parser::FastParserWrapper(FastParserWrapper { fast: parser, right_data: Some(right_data) }), ParseResults::empty_unfinished())
            }
        }
    }
}

impl ParserTrait for FastParserWrapper {
    fn get_u8set(&self) -> U8Set {
        self.fast.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let result = self.fast.parse(bytes);
        let mut right_data_vec = vec![];
        for i in result.finish_positions {
            let mut right_data = self.right_data.as_ref().unwrap().clone();
            right_data.advance(i);
            right_data_vec.push(right_data);
        }
        ParseResults::new(right_data_vec, result.done)
    }
}

pub fn fast_combinator(parser: FastCombinator) -> FastCombinatorWrapper {
    FastCombinatorWrapper { fast: Rc::new(parser) }
}

impl From<FastCombinatorWrapper> for Combinator {
    fn from(fast_combinator: FastCombinatorWrapper) -> Self {
        Combinator::Fast(fast_combinator)
    }
}

impl From<FastCombinator> for Combinator {
    fn from(value: FastCombinator) -> Self {
        Combinator::from(fast_combinator(value))
    }
}