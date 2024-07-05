use crate::{FrameStack, ParseResult};

#[derive(Clone, PartialEq, Debug)]
pub struct ParseData {
    pub(crate) frame_stack: FrameStack,
}

impl ParseData {
    pub fn merge(&self, other: Self) -> Self {
        Self { frame_stack: self.frame_stack.clone() | other.frame_stack }
    }
}

pub trait Combinator {
    type Parser: Parser;
    fn parser(&self, parse_data: ParseData) -> Self::Parser;
}

pub trait Parser {
    fn result(&self) -> ParseResult;
    fn step(&mut self, c: u8);
}