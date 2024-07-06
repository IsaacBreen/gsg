use crate::{Combinator, Eps, eps, ParseData, Parser, ParseResult, U8Set};

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

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let (a, result_a) = self.a.parser(parse_data.clone());
        let (b, result_b) = self.b.parser(parse_data);
        let mut parser = Choice2Parser {
            a: Some(a),
            b: Some(b),
        };

        if result_a.u8set.is_empty() {
            parser.a = None;
        }
        if result_b.u8set.is_empty() {
            parser.b = None;
        }

        (parser, result_a.merge(result_b))
    }
}

impl<ParserA, ParserB> Parser for Choice2Parser<ParserA, ParserB>
where
    ParserA: Parser,
    ParserB: Parser,
{
    fn step(&mut self, c: u8) -> ParseResult {
        let mut result = ParseResult::empty();

        if let Some(a) = &mut self.a {
            result = a.step(c);
            if result.u8set.is_empty() {
                self.a = None;
            }
        }

        if let Some(b) = &mut self.b {
            let result_b = b.step(c);
            if result_b.u8set.is_empty() {
                self.b = None;
            }
            result = result.merge(result_b);
        }

        result
    }
}

pub fn choice2<A, B>(a: A, b: B) -> Choice2<A, B> {
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

pub fn opt<A>(a: A) -> Choice2<A, Eps>
where
    A: Combinator,
{
    choice2(a, eps())
}
