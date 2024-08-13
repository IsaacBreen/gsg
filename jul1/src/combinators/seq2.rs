use std::rc::Rc;
use std::collections::BTreeMap;
use crate::{Combinator, CombinatorTrait, FailParser, Parser, ParseResults, ParserTrait, profile, RightData, RightDataSquasher, U8Set, VecY};
use crate::SeqParser;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seq2<A, B>
where
    A: Into<Combinator>,
    B: Into<Combinator>,
{
    pub(crate) first: A,
    pub(crate) second: B,
}

impl<A, B> CombinatorTrait for Seq2<A, B>
where
    A: Into<Combinator> + Clone,
    B: Into<Combinator> + Clone,
{
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let start_position = right_data.right_data_inner.fields1.position;

        let first_combinator = self.first.clone().into();
        let (first_parser, first_parse_results) = profile!("seq2 first child parse", {
            first_combinator.parse(right_data, &bytes)
        });

        let done = first_parse_results.done();
        if done && first_parse_results.right_data_vec.is_empty() {
            // Shortcut
            return (Parser::FailParser(FailParser), first_parse_results);
        }

        let mut parsers = if done {
            vec![]
        } else {
            vec![(0, first_parser)]
        };

        let mut final_right_data = VecY::new();
        let mut next_right_data_vec = first_parse_results.right_data_vec;

        let mut helper = |right_data: RightData| {
            let offset = right_data.right_data_inner.fields1.position - start_position;
            let second_combinator = self.second.clone().into();
            let (parser, parse_results) = profile!("seq2 second child parse", {
                second_combinator.parse(right_data, &bytes[offset..])
            });
            if !parse_results.done() {
                parsers.push((1, parser));
            }
            parse_results.right_data_vec
        };

        let mut next_next_right_data_vec = VecY::new();
        for right_data in next_right_data_vec {
            next_next_right_data_vec.extend(helper(right_data));
        }
        final_right_data = next_next_right_data_vec;

        if parsers.is_empty() {
            return (Parser::FailParser(FailParser), ParseResults::new(final_right_data, true));
        }

        let parser = Parser::SeqParser(SeqParser {
            parsers,
            combinators: Rc::new(vec![self.first.clone().into(), self.second.clone().into()].into()),
            position: start_position + bytes.len(),
        });

        let parse_results = ParseResults::new(final_right_data, false);

        (parser.into(), parse_results)
    }
}


#[macro_export]
macro_rules! seq2 {
    ($first:expr, $second:expr) => {
        $crate::Seq2 {
            first: $first,
            second: $second,
        }
    };
}
//
// impl<A, B> From<Seq2<A, B>> for Combinator
// where
//     A: Into<Combinator>,
//     B: Into<Combinator>,
// {
//     fn from(value: Seq2<A, B>) -> Self {
//         Combinator::Seq2(value)
//     }
// }