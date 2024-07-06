use crate::{Combinator, ParseData, Parser, ParseResult};

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

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let (parser_a, mut result) = self.a.parser(parse_data.clone());
        let parsers_b = result.parse_data.clone()
            .map(|pd| self.b.parser(pd))
            .map(|(parser_b, result_b)| {
                result.forward_assign(result_b);
                parser_b
            })
            .into_iter()
            .collect();

        (Seq2Parser {
            b: self.b.clone(),
            parser_a: result.u8set.is_empty().then(|| parser_a),
            parsers_b,
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
        let mut results_b = ParseResult::default();

        self.parsers_b.retain_mut(|parser_b| {
            let result_b = parser_b.step(c);
            results_b.merge_assign(result_b);
            !results_b.u8set.is_empty()
        });

        let result_a = if let Some(parser_a) = &mut self.parser_a {
            let result_a = parser_a.step(c);
            if result_a.u8set.is_empty() {
                self.parser_a = None;
            }
            if let Some(new_parse_data) = result_a.parse_data.clone() {
                let (parser_b, result_b) = self.b.parser(new_parse_data);
                self.parsers_b.push(parser_b);
                results_b.forward_assign(result_b);
            }
            result_a
        } else {
            ParseResult::default()
        };

        result_a.forward(results_b)
    }
}

pub fn seq2<A, B>(a: A, b: B) -> Seq2<A, B> {
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