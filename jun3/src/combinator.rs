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
    fn _parser(&self, parse_data: ParseData) -> Self::Parser;
    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let parser = self._parser(parse_data);
        let result = parser._result();
        (parser, result)
    }
}

pub trait Parser {
    fn _result(&self) -> ParseResult;
    fn _step(&mut self, c: u8);
    fn step(&mut self, c: u8) -> ParseResult {
        self._step(c);
        self._result()
    }
}

impl Parser for Box<dyn Parser> {
    fn _result(&self) -> ParseResult {
        (**self)._result()
    }
    fn _step(&mut self, c: u8) {
        (**self)._step(c)
    }
}