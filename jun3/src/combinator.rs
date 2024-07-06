use crate::{FrameStack, ParseResult};
use std::fmt::Debug;

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
    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult);
}

pub trait Parser {
    fn step(&mut self, c: u8) -> Result<ParseResult, String>;
}

impl Parser for Box<dyn Parser> {
    fn step(&mut self, c: u8) -> Result<ParseResult, String> {
        (**self).step(c)
    }
}