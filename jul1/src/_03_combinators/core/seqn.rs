
// src/_03_combinators/core/seqn.rs
use crate::{BaseCombinatorTrait, CombinatorTrait, IntoCombinator, ParseResultTrait, ParseResults, ParserTrait, RightData, RightDataGetters, Squash, U8Set, UnambiguousParseResults, VecY, UpData, OneShotUpData};

macro_rules! profile {
    ($name:expr, $expr:expr) => {
        $expr
    };
}

macro_rules! count_hit { ($tag:expr) => {} }

#[macro_export]
macro_rules! define_seq {
    ($seq_name:ident, $seq_parser_name:ident, $seq_enum_name:ident, $seq_partial_enum_name:ident, $first:ident, $($rest:ident),+ $(,)?) => {
        #[derive(Debug)]
        pub struct $seq_name<$first, $($rest),+>
        where
            $($rest: CombinatorTrait),+
        {
            pub(crate) $first: $first,
            $(pub(crate) $rest: $rest),+
        }

        #[derive(Debug)]
        pub struct $seq_parser_name<'a, $first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+
        {
            pub(crate) combinator: &'a $seq_name<$first, $($rest),+>,
            pub(crate) $first: Option<$first::Parser<'a>>,
            $(pub(crate) $rest: Vec<$rest::Parser<'a>>,)+
            pub(crate) position: usize,
        }

        #[derive(Debug)]
        pub struct $seq_enum_name<$first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+
        {
            pub(crate) $first: $first::Output,
            $(pub(crate) $rest: $rest::Output),+
        }

        #[derive(Debug)]
        pub struct $seq_partial_enum_name<$first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+
        {
            pub(crate) $first: Option<$first::Output>,
            $(pub(crate) $rest: Option<$rest::Output>),+
        }

        impl<$first, $($rest),+> $crate::DynCombinatorTrait for $seq_name<$first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+,
        {
            fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait<Self::Output> + '_>, ParseResults<Self::Output>) {
                let (parser, parse_results) = self.parse(right_data, bytes);
                (Box::new(parser), parse_results)
            }

            fn one_shot_parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults<Self::Output> {
                self.one_shot_parse(right_data, bytes)
            }
        }

        impl<$first, $($rest),+> CombinatorTrait for $seq_name<$first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+,
        {
            type Parser<'a> = $seq_parser_name<'a, $first, $($rest),+> where Self: 'a;
            type Output = $seq_enum_name<$first, $($rest),+>;

            fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> $crate::UnambiguousParseResults<Self::Output> where Self::Output: 'b {
                let mut right_data = right_data;
                let start_position = right_data.get_fields1().position;
                let result = self.$first.one_shot_parse(right_data, bytes);
                match result {
                    Ok(one_shot_up_data) => {
                        right_data = one_shot_up_data.just_right_data();
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
                $(
                    let offset = right_data.get_fields1().position - start_position;
                    let result = self.$rest.one_shot_parse(right_data, &bytes[offset..]);
                    match result {
                        Ok(one_shot_up_data) => {
                            right_data = one_shot_up_data.just_right_data();
                        },
                        Err(err) => {
                            return Err(err);
                        }
                    }
                )+
                $crate::UnambiguousParseResults::Ok(OneShotUpData::new(right_data, $seq_enum_name {
                    $first: one_shot_up_data.output,
                    $($rest: one_shot_up_data.output),+
                }))
            }

            fn old_parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b {
                let start_position = right_data.get_fields1().position;

                let first_combinator = &self.$first;
                let (first_parser, first_parse_results) = profile!(stringify!($seq_name, " first child parse"), {
                    first_combinator.parse(right_data, &bytes)
                });

                let mut all_done = first_parse_results.done();
                if all_done && first_parse_results.up_data_vec.is_empty() {
                    // Shortcut
                    return (
                        $seq_parser_name {
                            combinator: self,
                            $first: None,
                            $($rest: vec![],)+
                            position: start_position + bytes.len(),
                        },
                        first_parse_results
                    );
                }
                let first_parser_vec = if all_done { None } else { Some(first_parser) };

                let mut next_up_data_vec = first_parse_results.up_data_vec;

                fn helper<'a, T: CombinatorTrait>(right_data: RightData, next_combinator: &'a T, bytes: &[u8], start_position: usize) -> (T::Parser<'a>, ParseResults<T::Output>) {
                    let offset = right_data.get_fields1().position - start_position;
                    profile!(stringify!($seq_name, " child parse"), {
                        next_combinator.parse(right_data, &bytes[offset..])
                    })
                }

                let mut seqn_parser = $seq_parser_name {
                    combinator: self,
                    $first: first_parser_vec,
                    $($rest: vec![],)+
                    position: start_position + bytes.len(),
                };

                // process each child combinator
                $(
                    if next_up_data_vec.is_empty() {
                        return (seqn_parser, ParseResults::empty(all_done));
                    }

                    let mut next_next_up_data_vec = VecY::new();
                    for up_data in next_up_data_vec {
                        let (parser, parse_results) = helper(up_data.just_right_data(), &self.$rest, &bytes, start_position);
                        if !parse_results.done() {
                            all_done = false;
                            seqn_parser.$rest.push(parser);
                        }
                        next_next_up_data_vec.extend(parse_results.up_data_vec);
                    }
                    next_up_data_vec = next_next_up_data_vec;
                )+

                let parse_results = ParseResults::new(next_up_data_vec, all_done);

                (seqn_parser, parse_results)
            }
        }

        impl<'a, $first, $($rest),+> ParserTrait<$seq_enum_name<$first, $($rest),+>> for $seq_parser_name<'a, $first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+
        {
            fn get_u8set(&self) -> U8Set {
                let mut u8set = U8Set::none();
                if let Some(parser) = &self.$first {
                    u8set = u8set.union(&parser.get_u8set());
                }
                $(
                    for parser in &self.$rest {
                        u8set = u8set.union(&parser.get_u8set());
                    }
                )+
                u8set
            }

            fn parse<'b>(&mut self, bytes: &'b [u8]) -> ParseResults<$seq_enum_name<$first, $($rest),+>> where $seq_enum_name<$first, $($rest),+>: 'b {
                profile!(stringify!($seq_parser_name, "::parse"), {
                    let mut new_up_data: VecY<UpData<$seq_enum_name<$first, $($rest),+>>> = VecY::new();

                    // first child
                    if let Some(parser) = &mut self.$first {
                        let parse_results = profile!(stringify!($seq_parser_name, "::parse child Parser::parse"), { parser.parse(bytes) });
                        new_up_data.extend(parse_results.up_data_vec);
                        if parse_results.done {
                            self.$first = None;
                        }
                    }

                    let mut all_done = self.$first.is_none();

                    // rest of the children
                    $(
                        let mut up_data_to_init_this_child = std::mem::take(&mut new_up_data);
                        // TODO: Squashing after each sequent is good for performance (~5% gain on Python parser), but makes it difficult to reason about building the parse tree.
                        //  So, I'm leaving it out for now. But it'd be good to revisit this and see if we could bring it back.
                        // up_data_to_init_this_child.squash();

                        // step existing parsers for this child
                        self.$rest.retain_mut(|parser| {
                            let parse_results = profile!(stringify!($seq_parser_name, "::parse child Parser::parse"), {
                                parser.parse(bytes)
                            });
                            new_up_data.extend(parse_results.up_data_vec);
                            !parse_results.done
                        });

                        // new parsers for this child, one for each right_data emitted by the previous child
                        for up_data in up_data_to_init_this_child {
                            let offset = up_data.get_fields1().position - self.position;
                            let combinator = &self.combinator.$rest;
                            let (parser, parse_results) = profile!(stringify!($seq_parser_name, "::parse child Combinator::parse"), {
                                combinator.parse(up_data.just_right_data(), &bytes[offset..])
                            });
                            if !parse_results.done() {
                                self.$rest.push(parser);
                            }
                            new_up_data.extend(parse_results.up_data_vec);
                        }

                        all_done &= self.$rest.is_empty();
                    )+

                    self.position += bytes.len();

                    ParseResults::new(new_up_data, all_done)
                })
            }
        }

        impl<$first, $($rest),+> BaseCombinatorTrait for $seq_name<$first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+,
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

define_seq!(Seq2, Seq2Parser, Seq2Enum, Seq2PartialEnum, c0, c1);
define_seq!(Seq3, Seq3Parser, Seq3Enum, Seq3PartialEnum, c0, c1, c2);
define_seq!(Seq4, Seq4Parser, Seq4Enum, Seq4PartialEnum, c0, c1, c2, c3);
define_seq!(Seq5, Seq5Parser, Seq5Enum, Seq5PartialEnum, c0, c1, c2, c3, c4);
define_seq!(Seq6, Seq6Parser, Seq6Enum, Seq6PartialEnum, c0, c1, c2, c3, c4, c5);
define_seq!(Seq7, Seq7Parser, Seq7Enum, Seq7PartialEnum, c0, c1, c2, c3, c4, c5, c6);
define_seq!(Seq8, Seq8Parser, Seq8Enum, Seq8PartialEnum, c0, c1, c2, c3, c4, c5, c6, c7);
define_seq!(Seq9, Seq9Parser, Seq9Enum, Seq9PartialEnum, c0, c1, c2, c3, c4, c5, c6, c7, c8);
define_seq!(Seq10, Seq10Parser, Seq10Enum, Seq10PartialEnum, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
define_seq!(Seq11, Seq11Parser, Seq11Enum, Seq11PartialEnum, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10);
define_seq!(Seq12, Seq12Parser, Seq12Enum, Seq12PartialEnum, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11);
define_seq!(Seq13, Seq13Parser, Seq13Enum, Seq13PartialEnum, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12);