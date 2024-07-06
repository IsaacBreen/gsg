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

    fn parser(&self, parse_data: ParseData) -> Self::Parser {
        Choice2Parser {
            a: Some(self.a.parser(parse_data.clone())),
            b: Some(self.b.parser(parse_data)),
        }
    }
}

impl<ParserA, ParserB> Parser for Choice2Parser<ParserA, ParserB>
where
    ParserA: Parser,
    ParserB: Parser,
{
    fn result(&self) -> ParseResult {
        match self {
            Choice2Parser { a, b } => match (a, b) {
                (Some(a), Some(b)) => a.result().merge(b.result()),
                (Some(a), None) => a.result(),
                (None, Some(b)) => b.result(),
                (None, None) => ParseResult::new(U8Set::none(), None),
            },
        }
    }

    fn step(&mut self, c: u8) {
        if let Some(a) = &mut self.a {
            if a.result().u8set.is_empty() {
                self.a = None;
            } else {
                a.step(c);
            }
        }
        if let Some(b) = &mut self.b {
            if b.result().u8set.is_empty() {
                self.b = None;
            } else {
                b.step(c);
            }
        }
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