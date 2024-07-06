use crate::{Combinator, ParseData, Parser, ParseResult, U8Set};

pub struct Seq2<A, B> {
    a: A,
    b: B,
}

pub struct Seq2Parser<B, ParserA, ParserB> {
    b: B,
    parser_a: Option<ParserA>,
    parsers_b: Vec<ParserB>,
}

impl<A, B, ParserA, ParserB> Combinator for Seq2<A, B>
where
    A: Combinator<Parser = ParserA>,
    B: Combinator<Parser = ParserB> + Clone,
    ParserA: Parser,
    ParserB: Parser,
{
    type Parser = Seq2Parser<B, ParserA, ParserB>;

    fn _parser(&self, parse_data: ParseData) -> Self::Parser {
        let mut parser_a = self.a._parser(parse_data.clone());
        let mut parsers_b = Vec::new();
        let result_a = parser_a._result();
        if let Some(new_parse_data) = result_a.parse_data {
            parsers_b.push(self.b._parser(new_parse_data));
        }
        let parser_a = if result_a.u8set.is_empty() { None } else { Some(parser_a) };
        Seq2Parser {
            b: self.b.clone(),
            parser_a,
            parsers_b,
        }
    }
}

impl<B, ParserA, ParserB> Parser for Seq2Parser<B, ParserA, ParserB>
where
    B: Combinator<Parser = ParserB>,
    ParserA: Parser,
    ParserB: Parser,
{
    fn _result(&self) -> ParseResult {
        let mut result = match self.parser_a {
            Some(ref parser_a) => {
                let mut result = parser_a._result();
                result.parse_data = None;
                result
            }
            None => ParseResult::new(U8Set::none(), None),
        };
        for parser_b in &self.parsers_b {
            result = result.merge(parser_b._result());
        }
        result
    }

    fn _step(&mut self, c: u8) {
        self.parsers_b.retain(|parser_b| !parser_b._result().u8set.is_empty());
        for parser_b in &mut self.parsers_b {
            parser_b._step(c);
        }
        if let Some(ref mut parser_a) = self.parser_a {
            parser_a._step(c);
            if let Some(new_parse_data) = parser_a._result().parse_data {
                self.parsers_b.push(self.b._parser(new_parse_data));
            }
            if parser_a._result().u8set.is_empty() {
                self.parser_a = None;
            }
        }
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