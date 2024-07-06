use crate::{Combinator, ParseData, Parser, ParseResult};

#[derive(Clone)]
pub struct Repeat1<A> {
    a: A,
}

pub struct Repeat1Parser<A, ParserA> {
    a: A,
    parsers: Vec<ParserA>,
    result: ParseResult,
}

impl<A, ParserA> Combinator for Repeat1<A>
where
    A: Combinator<Parser = ParserA> + Clone,
    ParserA: Parser,
{
    type Parser = Repeat1Parser<A, ParserA>;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let (parser, result) = self.a.parser(parse_data.clone());
        (Repeat1Parser {
            a: self.a.clone(),
            parsers: vec![parser],
            result: result.clone(),
        }, result)
    }
}

impl<A, ParserA> Parser for Repeat1Parser<A, ParserA>
where
    A: Combinator<Parser = ParserA>,
    ParserA: Parser,
{
    fn step(&mut self, c: u8) -> ParseResult {
        self.parsers.retain(|parser| !self.result.u8set.is_empty());
        for parser in &mut self.parsers {
            self.result = parser.step(c);
        }
        if self.result.parse_data.is_some() {
            let (parser, result) = self.a.parser(ParseData::default());
            self.parsers.push(parser);
            self.result = self.result.clone().merge(result);
        }
        self.result.clone()
    }
}

pub fn repeat1<A>(a: A) -> Repeat1<A>
{
    Repeat1 { a }
}