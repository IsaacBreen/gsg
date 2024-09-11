use crate::{BaseCombinatorTrait, CombinatorTrait, IntoCombinator, ParseResultTrait, ParseResults, ParserTrait, RightData, RightDataGetters, Squash, U8Set, UnambiguousParseResults, VecY, UpData, OneShotUpData, OutputTrait};

macro_rules! profile {
    ($tag:expr, $body:expr) => {{
        $body
    }};
}

#[macro_export]
macro_rules! define_choice {
    ($choice_name:ident, $choice_parser_name:ident, $choice_enum_name:ident, $first:ident, $($rest:ident),+) => {
        #[derive(Debug)]
        pub struct $choice_name<$first, $($rest),+> {
            pub(crate) $first: $first,
            $(pub(crate) $rest: $rest,)+
            pub(crate) greedy: bool,
        }

        #[derive(Debug)]
        pub struct $choice_parser_name<'a, $first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+
        {
            pub(crate) combinator: &'a $choice_name<$first, $($rest),+>,
            pub(crate) $first: Vec<$first::Parser<'a>>,
            $(pub(crate) $rest: Vec<$rest::Parser<'a>>,)+
            pub(crate) position: usize,
        }

        pub struct $choice_enum_name<$first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+
        {
            pub(crate) $first: $first::Output,
            $(pub(crate) $rest: $rest::Output,)+
        }

        impl<$first, $($rest),+> $crate::DynCombinatorTrait for $choice_name<$first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+,
        {
            fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults<Self::Output>) {
                let (parser, parse_results) = self.parse(right_data, bytes);
                (Box::new(parser), parse_results)
            }

            fn one_shot_parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults<Self::Output> {
                self.one_shot_parse(right_data, bytes)
            }
        }

        impl<$first, $($rest),+> CombinatorTrait for $choice_name<$first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+,
        {
            type Parser<'a> = $choice_parser_name<'a, $first, $($rest),+> where Self: 'a;
            type Output = $choice_enum_name<$first, $($rest),+>;

            fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> $crate::UnambiguousParseResults<Self::Output> where Self::Output: 'b {
                use $crate::{UnambiguousParseResults, UnambiguousParseError};
                if self.greedy {
                    let first_combinator = &self.$first;
                    let parse_result = first_combinator.one_shot_parse(right_data.clone(), bytes);
                    if matches!(parse_result, Ok(_) | Err(UnambiguousParseError::Ambiguous | UnambiguousParseError::Incomplete)) {
                        return parse_result;
                    }

                    $(
                        let next_combinator = &self.$rest;
                        let parse_result = next_combinator.one_shot_parse(right_data.clone(), bytes);
                        if matches!(parse_result, Ok(_) | Err(UnambiguousParseError::Ambiguous | UnambiguousParseError::Incomplete)) {
                            return parse_result;
                        }
                    )+

                    Err(UnambiguousParseError::Fail)
                } else {
                    let first_combinator = &self.$first;
                    let mut final_parse_result = first_combinator.one_shot_parse(right_data.clone(), bytes);
                    if matches!(final_parse_result, Err(UnambiguousParseError::Ambiguous | UnambiguousParseError::Incomplete)) {
                        return final_parse_result;
                    }

                    $(
                        let next_combinator = &self.$rest;
                        let parse_result = next_combinator.one_shot_parse(right_data.clone(), bytes);
                        match (&parse_result, &final_parse_result) {
                            (Err(UnambiguousParseError::Ambiguous | UnambiguousParseError::Incomplete), _) => {
                                return parse_result;
                            },
                            (Ok(_), Ok(_)) => {
                                return parse_result;
                            },
                            (Ok(_), Err(UnambiguousParseError::Fail)) => {
                                final_parse_result = parse_result;
                            },
                            (Ok(_), Err(UnambiguousParseError::Incomplete | UnambiguousParseError::Ambiguous)) => unreachable!(),
                            (Err(UnambiguousParseError::Fail), _) => {},
                        }
                    )+

                    final_parse_result
                }
            }

            fn parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b {
                let start_position = right_data.get_fields1().position;

                let mut combined_results = ParseResults::empty_finished();

                let (first_parser, first_parse_results) = self.$first.parse(right_data.clone(), bytes);
                let mut parser = $choice_parser_name {
                    combinator: self,
                    $first: vec![],
                    $($rest: vec![],)+
                    position: start_position + bytes.len(),
                };
                if !first_parse_results.done {
                    parser.$first.push(first_parser);
                }
                let discard_rest = self.greedy && first_parse_results.succeeds_decisively();
                combined_results.merge_assign(first_parse_results);

                $(
                    if discard_rest {
                        return (parser, combined_results);
                    }
                    let (next_parser, next_parse_results) = self.$rest.parse(right_data.clone(), bytes);
                    if !next_parse_results.done() {
                        parser.$rest.push(next_parser);
                    }
                    let discard_rest = self.greedy && next_parse_results.succeeds_decisively();
                    combined_results.merge_assign(next_parse_results);
                )+

                (parser, combined_results)
            }
        }

        impl<'a, $first, $($rest),+> ParserTrait for $choice_parser_name<'a, $first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+
        {
            type Output = $choice_enum_name<$first, $($rest),+>;

            fn get_u8set(&self) -> U8Set {
                let mut u8set = U8Set::none();
                for parser in &self.$first {
                    u8set = u8set.union(&parser.get_u8set());
                }
                $(
                    for parser in &self.$rest {
                        u8set = u8set.union(&parser.get_u8set());
                    }
                )+
                u8set
            }

            fn parse<'b>(&mut self, bytes: &'b [u8]) -> ParseResults<Self::Output> where Self::Output: 'b {
                profile!(stringify!($choice_parser_name, "::parse"), {
                    let mut parse_result = ParseResults::empty_finished();
                    let mut discard_rest = false;

                    // First child
                    self.$first.retain_mut(|parser| {
                        if discard_rest {
                            return false;
                        }
                        let parse_results = parser.parse(bytes);
                        let done = parse_results.done;
                        discard_rest = self.combinator.greedy && parse_results.succeeds_decisively();
                        parse_result.merge_assign(parse_results);
                        !done
                    });

                    // Rest of the children
                    $(
                        if discard_rest {
                            return parse_result;
                        }

                        self.$rest.retain_mut(|parser| {
                            if discard_rest {
                                return false;
                            }
                            let parse_results = parser.parse(bytes);
                            let done = parse_results.done;
                            discard_rest = self.combinator.greedy && parse_results.succeeds_decisively();
                            parse_result.merge_assign(parse_results);
                            !done
                        });
                    )+

                    parse_result
                })
            }
        }

        impl<$first, $($rest),+> BaseCombinatorTrait for $choice_name<$first, $($rest),+>
        where
            $first: BaseCombinatorTrait,
            $($rest: BaseCombinatorTrait),+,
        {
            fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
                self
            }

            fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
                f(&self.$first);
                $(f(&self.$rest);)+
            }
        }
    };
}

define_choice!(Choice2, Choice2Parser, Choice2Enum, c0, c1);
define_choice!(Choice3, Choice3Parser, Choice3Enum, c0, c1, c2);
define_choice!(Choice4, Choice4Parser, Choice4Enum, c0, c1, c2, c3);
define_choice!(Choice5, Choice5Parser, Choice5Enum, c0, c1, c2, c3, c4);
define_choice!(Choice6, Choice6Parser, Choice6Enum, c0, c1, c2, c3, c4, c5);
define_choice!(Choice7, Choice7Parser, Choice7Enum, c0, c1, c2, c3, c4, c5, c6);
define_choice!(Choice8, Choice8Parser, Choice8Enum, c0, c1, c2, c3, c4, c5, c6, c7);
define_choice!(Choice9, Choice9Parser, Choice9Enum, c0, c1, c2, c3, c4, c5, c6, c7, c8);
define_choice!(Choice10, Choice10Parser, Choice10Enum, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
define_choice!(Choice11, Choice11Parser, Choice11Enum, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10);
define_choice!(Choice12, Choice12Parser, Choice12Enum, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11);
define_choice!(Choice13, Choice13Parser, Choice13Enum, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12);
define_choice!(Choice14, Choice14Parser, Choice14Enum, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13);
define_choice!(Choice15, Choice15Parser, Choice15Enum, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14);
define_choice!(Choice16, Choice16Parser, Choice16Enum, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15);