use crate::{Combinator, ParseData, Parser, ParseResult, U8Set};

pub struct Seq2<A, B> {
    a: A,
    b: B,
}

pub struct Seq2Parser<B, ParserA, ParserB> {
    b: B,
    parser_a: Option<ParserA>,
    parsers_b: Vec<ParserB>,
    result: ParseResult,
}

impl<A, B, ParserA, ParserB> Combinator for Seq2<A, B>
where
    A: Combinator<Parser = ParserA>,
    B: Combinator<Parser = ParserB> + Clone,
    ParserA: Parser,
    ParserB: Parser,
{
    type Parser = Seq2Parser<B, ParserA, ParserB>;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let (parser_a, result_a) = self.a.parser(parse_data.clone());
        let mut parsers_b = Vec::new();
        let mut result = result_a;
        if let Some(ref new_parse_data) = result.parse_data {
            let (parser_b, result_b) = self.b.parser(new_parse_data.clone());
            parsers_b.push(parser_b);
            result = result.forward(result_b);
        }
        let parser_a = if result.u8set.is_empty() { None } else { Some(parser_a) };
        (Seq2Parser {
            b: self.b.clone(),
            parser_a,
            parsers_b,
            result: result.clone(),
        }, result)
    }
}

impl<B, ParserA, ParserB> Parser for Seq2Parser<B, ParserA, ParserB>
where
    B: Combinator<Parser = ParserB>,
    ParserA: Parser,
    ParserB: Parser,
{
    fn step(&mut self, c: u8) -> ParseResult {
        self.parsers_b.retain(|parser_b| !self.result.u8set.is_empty());
        self.result = ParseResult::empty();
        for parser_b in &mut self.parsers_b {
            self.result = self.result.clone().merge(parser_b.step(c));
        }
        if let Some(parser_a) = &mut self.parser_a {
            self.result = parser_a.step(c);
            if self.result.u8set.is_empty() {
                self.parser_a = None;
            }
            if let Some(new_parse_data) = self.result.parse_data.clone() {
                let (parser_b, result_b) = self.b.parser(new_parse_data);
                self.parsers_b.push(parser_b);
                self.result = self.result.clone().forward(result_b);
            }
        }
        self.result.clone()
    }
}

pub fn seq2<A, B>(a: A, b: B) -> Seq2<A, B>
{
    Seq2 { a, b }
}

#[macro_export]
macro_rules! seq {
    ($a:expr, $b:expr) => {
        $crate::combinators::seq2($a, $b)
    };
    ($a:expr, $b:expr, $($rest:expr),+) => {
        $crate::combinators::seq2($a, $crate::combinators::seq2($b, $($rest),+))
    };
}