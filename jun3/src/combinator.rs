use crate::{FrameStack, ParseResult};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ParseData {
    pub(crate) frame_stack: FrameStack,
}

impl ParseData {
    pub fn merge(&self, other: Self) -> Self {
        Self { frame_stack: self.frame_stack.clone() | other.frame_stack }
    }
}

pub trait Combinator where Self: 'static {
    type Parser: Parser;
    fn parser(&self, parse_data: ParseData) -> (ParseResult, Self::Parser);
}

pub trait Parser {
    fn step(self, c: u8) -> (ParseResult, Self::Parser);
}

impl Parser for Box<dyn Parser> {
    fn step(self, c: u8) -> (ParseResult, Self::Parser) {
        (**self).step(c)
    }
}