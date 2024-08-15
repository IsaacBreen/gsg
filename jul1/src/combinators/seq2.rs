use std::rc::Rc;
use std::collections::BTreeMap;
use crate::{Combinator, CombinatorTrait, FailParser, Parser, ParseResults, ParserTrait, profile, RightData, RightDataSquasher, U8Set, VecY, vecx, Fail};
use crate::SeqParser;

#[derive(Debug)]
pub struct Seq2<A, B>
where
    A: CombinatorTrait,
    B: CombinatorTrait,
{
    pub(crate) first: A,
    pub(crate) second: Rc<B>,
}

impl<A, B> CombinatorTrait for Seq2<A, B>
where
    A: CombinatorTrait,
    B: CombinatorTrait + 'static,
{
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let start_position = right_data.right_data_inner.fields1.position;

        let first_combinator = &self.first;
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

        fn helper<T: CombinatorTrait>(right_data: RightData, next_combinator: &Rc<T>, bytes: &[u8], start_position: usize, parsers: &mut Vec<(usize, Parser)>) -> VecY<RightData> {
            let offset = right_data.right_data_inner.fields1.position - start_position;
            let (parser, parse_results) = profile!("seq2 second child parse", {
                next_combinator.parse(right_data, &bytes[offset..])
            });
            if !parse_results.done() {
                parsers.push((1, parser));
            }
            parse_results.right_data_vec
        };

        let mut next_next_right_data_vec = VecY::new();
        for right_data in next_right_data_vec {
            next_next_right_data_vec.extend(helper(right_data, &self.second, &bytes, start_position, &mut parsers));
        }
        next_right_data_vec = next_next_right_data_vec;

        final_right_data = next_right_data_vec;

        if parsers.is_empty() {
            return (Parser::FailParser(FailParser), ParseResults::new(final_right_data, true));
        }

        let parser = Parser::SeqParser(SeqParser {
            parsers,
            combinators: Rc::new(vecx![Combinator::Fail(Fail), Combinator::DynRc(self.second.clone())]),
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

#[derive(Debug)]
pub struct Seq3<A, B, C>
where
    A: CombinatorTrait,
    B: CombinatorTrait,
    C: CombinatorTrait,
{
    pub(crate) first: A,
    pub(crate) second: Rc<B>,
    pub(crate) third: Rc<C>,
}

impl<A, B, C> CombinatorTrait for Seq3<A, B, C>
where
    A: CombinatorTrait,
    B: CombinatorTrait + 'static,
    C: CombinatorTrait + 'static,
{
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let start_position = right_data.right_data_inner.fields1.position;

        let first_combinator = &self.first;
        let (first_parser, first_parse_results) = profile!("seq3 first child parse", {
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

        fn helper<T: CombinatorTrait>(right_data: RightData, next_combinator: &Rc<T>, bytes: &[u8], start_position: usize, parsers: &mut Vec<(usize, Parser)>) -> VecY<RightData> {
            let offset = right_data.right_data_inner.fields1.position - start_position;
            let (parser, parse_results) = profile!("seq3 second child parse", {
                next_combinator.parse(right_data, &bytes[offset..])
            });
            if !parse_results.done() {
                parsers.push((1, parser));
            }
            parse_results.right_data_vec
        }

        let mut next_next_right_data_vec = VecY::new();
        for right_data in next_right_data_vec {
            next_next_right_data_vec.extend(helper(right_data, &self.second, &bytes, start_position, &mut parsers));
        }
        next_right_data_vec = next_next_right_data_vec;

        let mut next_next_right_data_vec = VecY::new();
        for right_data in next_right_data_vec {
            next_next_right_data_vec.extend(helper(right_data, &self.third, &bytes, start_position, &mut parsers));
        }
        next_right_data_vec = next_next_right_data_vec;

        final_right_data = next_right_data_vec;

        if parsers.is_empty() {
            return (Parser::FailParser(FailParser), ParseResults::new(final_right_data, true));
        }

        let parser = Parser::SeqParser(SeqParser {
            parsers,
            combinators: Rc::new(vecx![Combinator::Fail(Fail), Combinator::DynRc(self.second.clone())]),
            position: start_position + bytes.len(),
        });

        let parse_results = ParseResults::new(final_right_data, false);

        (parser.into(), parse_results)
    }
}

#[macro_export]
macro_rules! seq3 {
    ($first:expr, $second:expr, $third:expr) => {
        $crate::Seq3 {
            first: $first,
            second: $second,
            third: $third,
        }
    };
}