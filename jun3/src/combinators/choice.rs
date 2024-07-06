use crate::{Combinator, ParseData, Parser, ParseResult, U8Set};

pub struct Choice2<A, B> {
    a: A,
    b: B,
}

pub struct Choice2Parser<ParserA, ParserB> {
    a: Option<ParserA>,
    b: Option<ParserB>,
}

impl<A, B, ParserA, ParserB> Combinator for Choice2<A, B>
where
    A: Combinator<Parser = ParserA>,
    B: Combinator<Parser = ParserB>,
    ParserA: Parser,
    ParserB: Parser,
{
    type Parser = Choice2Parser<ParserA, ParserB>;

    fn parser(&self, parse_data: ParseData) -> (ParseResult, Self::Parser) {
        let (result_a, parser_a) = self.a.parser(parse_data.clone());
        let (result_b, parser_b) = self.b.parser(parse_data);
        (result_a.merge(result_b), Choice2Parser {
            a: Some(parser_a),
            b: Some(parser_b),
        })
    }
}

impl<ParserA, ParserB> Parser for Choice2Parser<ParserA, ParserB>
where
    ParserA: Parser,
    ParserB: Parser,
{
    fn step(self, c: u8) -> (ParseResult, Self::Parser) {
        let (result_a, a) = if let Some(parser_a) = self.a {
            parser_a.step(c)
        } else {
            (ParseResult::new(U8Set::none(), None), None)
        };
        let (result_b, b) = if let Some(parser_b) = self.b {
            parser_b.step(c)
        } else {
            (ParseResult::new(U8Set::none(), None), None)
        };
        (result_a.merge(result_b), Choice2Parser { a, b })
    }
}

pub fn choice2<A, B>(a: A, b: B) -> Choice2<A, B>
{
    Choice2 { a, b }
}

#[macro_export]
macro_rules! choice {
    ($a:expr, $b:expr) => {
        $crate::combinators::choice2($a, $b)
    };
    ($a:expr, $b:expr, $($rest:expr),+) => {
        $crate::combinators::choice2($a, $crate::combinators::choice2($b, $($rest),+))
    };
}