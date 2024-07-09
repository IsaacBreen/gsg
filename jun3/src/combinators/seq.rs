use std::ops::Not;
use std::rc::Rc;

use crate::{Combinator, ParseData, Parser, ParseResult};

pub struct Seq2<A, B> where A: Combinator, B: Combinator {
    a: A,
    b: Rc<B>,
}

pub struct Seq2Parser<B, ParserA, ParserB> {
    b: Rc<B>,
    parser_a: Option<ParserA>,
    parsers_b: Vec<ParserB>,
}

impl<A, B, ParserA, ParserB> Combinator for Seq2<A, B>
where
    A: Combinator<Parser = ParserA>,
    B: Combinator<Parser = ParserB>,
    ParserA: Parser,
    ParserB: Parser,
{
    type Parser = Seq2Parser<B, ParserA, ParserB>;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, Vec<ParseResult>) {
        // let (parser_a, mut result) = self.a.parser(parse_data.clone());
        // let parsers_b = result.parse_data.clone()
        //     .map(|pd| self.b.parser(pd))
        //     .map(|(parser_b, result_b)| {
        //         result.forward_assign(result_b);
        //         parser_b
        //     })
        //     .into_iter()
        //     .collect();
        //
        // (Seq2Parser {
        //     b: Rc::clone(&self.b),
        //     parser_a: result.u8set.is_empty().not().then(|| parser_a),
        //     parsers_b,
        // }, result)
        let (parser_a, mut results_a) = self.a.parser(parse_data.clone());
        let parsers_b_and_results = results_a.iter()
            .map(|result_a| result_a.parse_data)
            .filter(|parse_data| parse_data.is_some())
            .map(|parse_data| self.b.parser(parse_data.unwrap()))
            .collect::<Vec<_>>();
        let results_b = parsers_b_and_results.iter()
            .map(|(_, result_b)| result_b.clone())
            .flatten()
            .collect::<Vec<_>>();
        let parsers_b = parsers_b_and_results.into_iter()
            .map(|(parser_b, result_b)| parser_b)
            .collect::<Vec<_>>();
        (Seq2Parser {
            b: Rc::clone(&self.b),
            parser_a: Some(parser_a),
            parsers_b,
        }, results_a.into_iter().filter(|result_a| result_a.parse_data.is_some()).chain(results_b.into_iter()))
    }
}

impl<B, ParserA, ParserB> Parser for Seq2Parser<B, ParserA, ParserB>
where
    B: Combinator<Parser = ParserB>,
    ParserA: Parser,
    ParserB: Parser,
{
    fn step(&mut self, c: u8) -> Vec<ParseResult> {
        let mut results_b = Vec::new();

        self.parsers_b.retain_mut(|parser_b| {
            let new_results_b = parser_b.step(c);
            results_b.extend(new_results_b);
            !new_results_b.is_empty()
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

pub fn seq2<A: Combinator, B: Combinator>(a: A, b: B) -> Seq2<A, B> {
    Seq2 { a, b: Rc::new(b) }
}

#[macro_export]
macro_rules! seq {
    ($a:expr $(,)?) => {
        $a
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::combinators::seq2($a, $crate::seq!($($rest),+))
    };
}