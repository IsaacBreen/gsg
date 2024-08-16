use std::rc::Rc;
use std::collections::BTreeMap;
use crate::{CombinatorTrait, FailParser, Parser, ParseResults, ParserTrait, profile, RightData, RightDataSquasher, U8Set, VecY, vecx, Fail, IntoCombinator};
use crate::ChoiceParser;

macro_rules! profile {
    ($tag:expr, $body:expr) => {{
        $body
    }};
}

#[macro_export]
macro_rules! define_choice {
    ($choice_name:ident, $first:ident, $($rest:ident),+) => {
        #[derive(Debug)]
        pub struct $choice_name<$first, $($rest),+>
        where
            $($rest: CombinatorTrait),+
        {
            pub(crate) $first: $first,
            $(pub(crate) $rest: Rc<$rest>,)+
            pub(crate) greedy: bool,
        }

        impl<$first: 'static, $($rest: 'static),+> CombinatorTrait for $choice_name<$first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+
        {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
                f(&self.$first);
                $(f(self.$rest.as_ref());)+
            }

            fn parse<'a, 'b, 'c>(&'c self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'b>, ParseResults) where Self: 'a, 'a: 'b {
                let mut parsers = Vec::new();
                let mut combined_results = ParseResults::empty_finished();

                let first_combinator = &self.$first;
                let (first_parser, first_parse_results) = profile!(stringify!($choice_name, " first child parse"), {
                    first_combinator.parse(right_data.clone(), bytes)
                });
                if !first_parse_results.done() {
                    parsers.push(first_parser);
                }
                let discard_rest = self.greedy && first_parse_results.succeeds_decisively();
                combined_results.merge_assign(first_parse_results);
                if discard_rest {
                    return (
                        Parser::ChoiceParser(ChoiceParser { parsers, greedy: self.greedy }),
                        combined_results
                    );
                }

                $(
                    let next_combinator = &self.$rest;
                    let (next_parser, next_parse_results) = profile!(stringify!($choice_name, " child parse"), {
                        next_combinator.parse(right_data.clone(), bytes)
                    });
                    if !next_parse_results.done() {
                        parsers.push(next_parser);
                    }
                    let discard_rest = self.greedy && next_parse_results.succeeds_decisively();
                    combined_results.merge_assign(next_parse_results);
                    if discard_rest {
                        return (
                            Parser::ChoiceParser(ChoiceParser { parsers, greedy: self.greedy }),
                            combined_results
                        );
                    }
                )+

                (
                    Parser::ChoiceParser(ChoiceParser { parsers, greedy: self.greedy }),
                    combined_results
                )
            }
        }
    };
}

define_choice!(Choice2, c0, c1);
define_choice!(Choice3, c0, c1, c2);
define_choice!(Choice4, c0, c1, c2, c3);
define_choice!(Choice5, c0, c1, c2, c3, c4);
define_choice!(Choice6, c0, c1, c2, c3, c4, c5);
define_choice!(Choice7, c0, c1, c2, c3, c4, c5, c6);
define_choice!(Choice8, c0, c1, c2, c3, c4, c5, c6, c7);
define_choice!(Choice9, c0, c1, c2, c3, c4, c5, c6, c7, c8);
define_choice!(Choice10, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
define_choice!(Choice11, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10);
define_choice!(Choice12, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11);
define_choice!(Choice13, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12);
define_choice!(Choice14, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13);
define_choice!(Choice15, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14);
define_choice!(Choice16, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15);

pub fn choicen_helper<T: IntoCombinator>(x: T) -> Rc<T::Output> {
        Rc::new(IntoCombinator::into_combinator(x))
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
        fn convert(x: impl $crate::IntoCombinator + 'static) -> Box<dyn $crate::CombinatorTrait> {
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
    ($($rest:expr),+ $(,)?) => {
        $crate::choice_generalised!(false, $($rest),+)
    };
}

#[macro_export]
macro_rules! choice_greedy {
    ($($rest:expr),+ $(,)?) => {
        $crate::choice_generalised!(true, $($rest),+)
    };
}
