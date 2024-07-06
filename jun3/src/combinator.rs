use crate::ParseData;
use crate::ParseResult;

pub trait Combinator where Self: 'static {
    type Parser: Parser;
    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult);
}

pub trait Parser {
    fn step(&mut self, c: u8) -> ParseResult;
}

impl Parser for Box<dyn Parser> {
    fn step(&mut self, c: u8) -> ParseResult {
        (**self).step(c)
    }
}