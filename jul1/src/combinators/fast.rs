use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use crate::*;
use crate::fast_combinator::{FastParserResult, FastParserTrait};

pub struct FastCombinator {
    pub(crate) fast: Rc<dyn FastParserTrait>,
    pub(crate) slow: Box<Combinator>,
}

impl Debug for FastCombinator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FastParser")
            .field("slow", &self.slow)
            .finish_non_exhaustive()
    }
}

impl Clone for FastCombinator {
    fn clone(&self) -> Self {
        Self {
            fast: self.fast.clone(),
            slow: self.slow.clone(),
        }
    }
}

impl PartialEq for FastCombinator {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.fast, &other.fast)
    }
}

impl Eq for FastCombinator {}

impl Hash for FastCombinator {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.fast).hash(state);
    }
}

impl CombinatorTrait for FastCombinator {
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

pub fn fast_parser(parser: impl FastParserTrait + 'static) -> FastCombinator {
    let slow = parser.slow();
    FastCombinator { fast: Rc::new(parser), slow: Box::new(slow) }
}

impl From<FastCombinator> for Combinator {
    fn from(fast_combinator: FastCombinator) -> Self {
        Combinator::Fast(fast_combinator)
    }
}
