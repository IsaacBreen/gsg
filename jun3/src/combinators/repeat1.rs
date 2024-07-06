use crate::{Combinator, ParseData, Parser, ParseResult};

#[derive(Clone)]
pub struct Repeat1<A> {
    a: A,
}

pub struct Repeat1Parser<A, ParserA> {
    a: A,
    parsers: Vec<ParserA>,
}

impl<A, ParserA> Combinator for Repeat1<A>
where
    A: Combinator<Parser = ParserA> + Clone,
    ParserA: Parser,
{
    type Parser = Repeat1Parser<A, ParserA>;

    fn _parser(&self, parse_data: ParseData) -> Self::Parser {
        Repeat1Parser {
            a: self.a.clone(),
            parsers: vec![self.a._parser(parse_data)],
        }
    }
}

impl<A, ParserA> Parser for Repeat1Parser<A, ParserA>
where
    A: Combinator<Parser = ParserA>,
    ParserA: Parser,
{
    fn _result(&self) -> ParseResult {
        let mut result = self.parsers[0]._result();
        for parser in &self.parsers[1..] {
            result = result.merge(parser._result());
        }
        result
    }

    fn _step(&mut self, c: u8) {
        self.parsers.retain(|parser| !parser._result().u8set.is_empty());
        for parser in &mut self.parsers {
            parser._step(c)
        }
        let any_done = self.parsers.iter().any(|parser| parser._result().parse_data.is_some());
        if any_done {
            self.parsers.push(self.a._parser(ParseData::default()));
        }
    }
}

pub fn repeat1<A>(a: A) -> Repeat1<A>
{
    Repeat1 { a }
}
