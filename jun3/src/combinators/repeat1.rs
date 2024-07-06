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

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let (parser, result) = self.a.parser(parse_data);
        (Repeat1Parser {
            a: self.a.clone(),
            parsers: vec![parser],
        }, result)
    }
}

impl<A, ParserA> Parser for Repeat1Parser<A, ParserA>
where
    A: Combinator<Parser = ParserA>,
    ParserA: Parser,
{
    fn step(&mut self, c: u8) -> ParseResult {
        let mut final_result = ParseResult::empty();

        self.parsers.retain_mut(|parser| {
            let result = parser.step(c);
            final_result.merge_assign(result.clone());
            !result.u8set.is_empty()
        });

        if let Some(new_parse_data) = &final_result.parse_data {
            let (parser, result) = self.a.parser(new_parse_data.clone());
            self.parsers.push(parser);
            final_result.merge_assign(result);
        }

        final_result
    }
}

pub fn repeat1<A>(a: A) -> Repeat1<A> {
    Repeat1 { a }
}