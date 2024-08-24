// src/combinators/choicen.rs
use std::collections::BTreeMap;
use crate::{CombinatorTrait, FailParser, ParseResults, ParserTrait, profile, ParseResultTrait, RightDataSquasher, U8Set, VecY, vecx, Fail, IntoCombinator, RightData, UnambiguousParseResults, BaseCombinatorTrait, Squash};

macro_rules! profile {
    ($tag:expr, $body:expr) => {{
        $body
    }};
}

#[macro_export]
macro_rules! define_choice {
    ($choice_name:ident, $choice_parser_name:ident, $first:ident, $($rest:ident),+) => {
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

        impl<$first, $($rest),+> $crate::DynCombinatorTrait for $choice_name<$first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+,
            for<'a> $first: 'a,
            $(for<'a> $rest: 'a),+
        {
            fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait>, ParseResults) {
                todo!()
            }
        }

        impl<$first, $($rest),+> CombinatorTrait for $choice_name<$first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+,
            // todo: can we move the 'a bound to the impl block?
            for<'a> $first: 'a,
            $(for<'a> $rest: 'a),+
        {
            type Parser<'a> = $choice_parser_name<'a, $first, $($rest),+> where Self: 'a;

            fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> $crate::UnambiguousParseResults {
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

            fn old_parse<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'a>, ParseResults) {
                let start_position = right_data.right_data_inner.fields1.position;

                let first_combinator = &self.$first;
                let (first_parser, first_parse_results) = profile!(stringify!($choice_name, " first child parse"), {
                    first_combinator.parse(right_data.clone(), bytes)
                });

                let mut all_done = first_parse_results.done();
                let first_parser_vec = if all_done { vec![] } else { vec![first_parser] };

                let mut next_right_data_vec = first_parse_results.right_data_vec;

                let mut choicen_parser = $choice_parser_name {
                    combinator: self,
                    $first: first_parser_vec,
                    $($rest: vec![],)+
                    position: start_position + bytes.len(),
                };

                // Macro to process each child combinator
                $(
                    let mut next_next_right_data_vec = VecY::new();
                    for right_data in next_right_data_vec {
                        let (parser, parse_results) = profile!(stringify!($choice_name, " child parse"), {
                            self.$rest.parse(right_data, bytes)
                        });
                        if !parse_results.done() {
                            all_done = false;
                            choicen_parser.$rest.push(parser);
                        }
                        next_next_right_data_vec.extend(parse_results.right_data_vec);
                    }
                    next_right_data_vec = next_next_right_data_vec;
                )+

                let parse_results = ParseResults::new(next_right_data_vec, all_done);

                (choicen_parser, parse_results)
            }
        }

        impl<'a, $first, $($rest),+> ParserTrait for $choice_parser_name<'a, $first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+
        {
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

            fn parse(&mut self, bytes: &[u8]) -> ParseResults {
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
            for<'a> $first: 'a,
            $(for<'a> $rest: 'a),+
        {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
                f(&self.$first);
                $(f(&self.$rest);)+
            }
        }
    };
}

define_choice!(Choice2, Choice2Parser, c0, c1);
define_choice!(Choice3, Choice3Parser, c0, c1, c2);
define_choice!(Choice4, Choice4Parser, c0, c1, c2, c3);
define_choice!(Choice5, Choice5Parser, c0, c1, c2, c3, c4);
define_choice!(Choice6, Choice6Parser, c0, c1, c2, c3, c4, c5);
define_choice!(Choice7, Choice7Parser, c0, c1, c2, c3, c4, c5, c6);
define_choice!(Choice8, Choice8Parser, c0, c1, c2, c3, c4, c5, c6, c7);
define_choice!(Choice9, Choice9Parser, c0, c1, c2, c3, c4, c5, c6, c7, c8);
define_choice!(Choice10, Choice10Parser, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
define_choice!(Choice11, Choice11Parser, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10);
define_choice!(Choice12, Choice12Parser, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11);
define_choice!(Choice13, Choice13Parser, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12);
define_choice!(Choice14, Choice14Parser, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13);
define_choice!(Choice15, Choice15Parser, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14);
define_choice!(Choice16, Choice16Parser, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15);

pub fn choicen_helper<T: IntoCombinator>(x: T) -> T::Output {
        IntoCombinator::into_combinator(x)
}

#[macro_export]
macro_rules! choice_generalised {
    ($greedy:expr, $c0:expr, $c1:expr $(,)?) => {{
        use $crate::choicen_helper as h;
        $crate::Choice2 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            greedy: $greedy,
        }
    }};
    ($greedy:expr, $c0:expr, $c1:expr, $c2:expr $(,)?) => {{
        use $crate::choicen_helper as h;
        $crate::Choice3 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            greedy: $greedy,
        }
    }};
    ($greedy:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr $(,)?) => {{
        use $crate::choicen_helper as h;
        $crate::Choice4 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            greedy: $greedy,
        }
    }};
    ($greedy:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr $(,)?) => {{
        use $crate::choicen_helper as h;
        $crate::Choice5 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
            greedy: $greedy,
        }
    }};
    ($greedy:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr $(,)?) => {{
        use $crate::choicen_helper as h;
        $crate::Choice6 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
            c5: h($c5),
            greedy: $greedy,
        }
    }};
    ($greedy:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr $(,)?) => {{
        use $crate::choicen_helper as h;
        $crate::Choice7 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
            c5: h($c5),
            c6: h($c6),
            greedy: $greedy,
        }
    }};
    ($greedy:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr $(,)?) => {{
        use $crate::choicen_helper as h;
        $crate::Choice8 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
            c5: h($c5),
            c6: h($c6),
            c7: h($c7),
            greedy: $greedy,
        }
    }};
    ($greedy:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr $(,)?) => {{
        use $crate::choicen_helper as h;
        $crate::Choice9 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
            c5: h($c5),
            c6: h($c6),
            c7: h($c7),
            c8: h($c8),
            greedy: $greedy,
        }
    }};
    ($greedy:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr $(,)?) => {{
        use $crate::choicen_helper as h;
        $crate::Choice10 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
            c5: h($c5),
            c6: h($c6),
            c7: h($c7),
            c8: h($c8),
            c9: h($c9),
            greedy: $greedy,
        }
    }};
    ($greedy:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr $(,)?) => {{
        use $crate::choicen_helper as h;
        $crate::Choice11 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
            c5: h($c5),
            c6: h($c6),
            c7: h($c7),
            c8: h($c8),
            c9: h($c9),
            c10: h($c10),
            greedy: $greedy,
        }
    }};
    ($greedy:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr $(,)?) => {{
        use $crate::choicen_helper as h;
        $crate::Choice12 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
            c5: h($c5),
            c6: h($c6),
            c7: h($c7),
            c8: h($c8),
            c9: h($c9),
            c10: h($c10),
            c11: h($c11),
            greedy: $greedy,
        }
    }};
    ($greedy:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr $(,)?) => {{
        use $crate::choicen_helper as h;
        $crate::Choice13 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
            c5: h($c5),
            c6: h($c6),
            c7: h($c7),
            c8: h($c8),
            c9: h($c9),
            c10: h($c10),
            c11: h($c11),
            c12: h($c12),
            greedy: $greedy,
        }
    }};
    ($greedy:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr, $c13:expr $(,)?) => {{
        use $crate::choicen_helper as h;
        $crate::Choice14 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
            c5: h($c5),
            c6: h($c6),
            c7: h($c7),
            c8: h($c8),
            c9: h($c9),
            c10: h($c10),
            c11: h($c11),
            c12: h($c12),
            c13: h($c13),
            greedy: $greedy,
        }
    }};
    ($greedy:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr, $c13:expr, $c14:expr $(,)?) => {{
        use $crate::choicen_helper as h;
        $crate::Choice15 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
            c5: h($c5),
            c6: h($c6),
            c7: h($c7),
            c8: h($c8),
            c9: h($c9),
            c10: h($c10),
            c11: h($c11),
            c12: h($c12),
            c13: h($c13),
            c14: h($c14),
            greedy: $greedy,
        }
    }};
    ($greedy:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr, $c13:expr, $c14:expr, $c15:expr $(,)?) => {{
        use $crate::choicen_helper as h;
        $crate::Choice16 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
            c5: h($c5),
            c6: h($c6),
            c7: h($c7),
            c8: h($c8),
            c9: h($c9),
            c10: h($c10),
            c11: h($c11),
            c12: h($c12),
            c13: h($c13),
            c14: h($c14),
            c15: h($c15),
            greedy: $greedy,
        }
    }};
    ($greedy:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr, $c13:expr, $c14:expr, $c15:expr, $($rest:expr),+ $(,)?) => {{
        fn convert<T: $crate::IntoCombinator>(x: T) -> Box<dyn $crate::CombinatorTrait> where T::Output {
            Box::new(x.into_combinator())
        }
        if $greedy {
            $crate::_choice_greedy(vec![convert($c0), convert($c1), convert($c2), convert($c3), convert($c4), convert($c5), convert($c6), convert($c7), convert($c8), convert($c9), convert($c10), convert($c11), convert($c12), convert($c13), convert($c14), convert($c15), $(convert($rest)),+])
        } else {
            $crate::_choice(vec![convert($c0), convert($c1), convert($c2), convert($c3), convert($c4), convert($c5), convert($c6), convert($c7), convert($c8), convert($c9), convert($c10), convert($c11), convert($c12), convert($c13), convert($c14), convert($c15), $(convert($rest)),+])
        }
    }};
}

#[macro_export]
macro_rules! choice {
    ($($rest:expr),+ $(,)?) => {{
        $crate::choice_generalised!(false, $($rest),+)
    }};
}

#[macro_export]
macro_rules! choice_greedy {
    ($($rest:expr),+ $(,)?) => {{
        $crate::choice_generalised!(true, $($rest),+)
    }};
}