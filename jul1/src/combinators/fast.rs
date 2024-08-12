use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use crate::*;
use crate::fast_combinator::{FastCombinator, FastParserResult};

pub struct FastCombinatorWrapper {
    pub(crate) fast: Rc<FastCombinator>,
    pub(crate) slow: Box<Combinator>,
}

impl Debug for FastCombinatorWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FastParser")
            .field("slow", &self.slow)
            .finish_non_exhaustive()
    }
}

impl Clone for FastCombinatorWrapper {
    fn clone(&self) -> Self {
        Self {
            fast: self.fast.clone(),
            slow: self.slow.clone(),
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
            FastParserResult::Success(len) => {
                right_data.advance(len);
                (Parser::FailParser(FailParser), ParseResults::new_single(right_data, true))
            }
            FastParserResult::Failure => {
                (Parser::FailParser(FailParser), ParseResults::empty_finished())
            }
            FastParserResult::Incomplete => {
                self.slow.parse(right_data, bytes)
            }
        }
    }
}

pub fn fast_combinator(parser: FastCombinator) -> FastCombinatorWrapper {
    let slow = parser.slow();
    FastCombinatorWrapper { fast: Rc::new(parser), slow: Box::new(slow) }
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