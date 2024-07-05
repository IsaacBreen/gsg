use crate::ParseResult;

trait Combinator {
    type Parser: Parser;
    fn parser(&self) -> Self::Parser;
}

trait Parser {
    type State;
    fn result(&self) -> ParseResult;
    fn step(&self, c: u8);
}