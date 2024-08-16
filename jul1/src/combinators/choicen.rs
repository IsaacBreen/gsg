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

            fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
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
macro_rules! choice {
    ($c0:expr, $c1:expr $(,)?) => {
        $crate::Choice2 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: crate::choicen_helper($c1),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr $(,)?) => {
        $crate::Choice3 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr $(,)?) => {
        $crate::Choice4 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr $(,)?) => {
        $crate::Choice5 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr $(,)?) => {
        $crate::Choice6 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr $(,)?) => {
        $crate::Choice7 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr $(,)?) => {
        $crate::Choice8 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr $(,)?) => {
        $crate::Choice9 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            c8: $crate::choicen_helper($c8),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr $(,)?) => {
        $crate::Choice10 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            c8: $crate::choicen_helper($c8),
            c9: $crate::choicen_helper($c9),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr $(,)?) => {
        $crate::Choice11 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            c8: $crate::choicen_helper($c8),
            c9: $crate::choicen_helper($c9),
            c10: $crate::choicen_helper($c10),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr $(,)?) => {
        $crate::Choice12 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            c8: $crate::choicen_helper($c8),
            c9: $crate::choicen_helper($c9),
            c10: $crate::choicen_helper($c10),
            c11: $crate::choicen_helper($c11),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr $(,)?) => {
        $crate::Choice13 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            c8: $crate::choicen_helper($c8),
            c9: $crate::choicen_helper($c9),
            c10: $crate::choicen_helper($c10),
            c11: $crate::choicen_helper($c11),
            c12: $crate::choicen_helper($c12),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr, $c13:expr $(,)?) => {
        $crate::Choice14 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            c8: $crate::choicen_helper($c8),
            c9: $crate::choicen_helper($c9),
            c10: $crate::choicen_helper($c10),
            c11: $crate::choicen_helper($c11),
            c12: $crate::choicen_helper($c12),
            c13: $crate::choicen_helper($c13),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr, $c13:expr, $c14:expr $(,)?) => {
        $crate::Choice15 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            c8: $crate::choicen_helper($c8),
            c9: $crate::choicen_helper($c9),
            c10: $crate::choicen_helper($c10),
            c11: $crate::choicen_helper($c11),
            c12: $crate::choicen_helper($c12),
            c13: $crate::choicen_helper($c13),
            c14: $crate::choicen_helper($c14),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr, $c13:expr, $c14:expr, $c15:expr $(,)?) => {
        $crate::Choice16 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            c8: $crate::choicen_helper($c8),
            c9: $crate::choicen_helper($c9),
            c10: $crate::choicen_helper($c10),
            c11: $crate::choicen_helper($c11),
            c12: $crate::choicen_helper($c12),
            c13: $crate::choicen_helper($c13),
            c14: $crate::choicen_helper($c14),
            c15: $crate::choicen_helper($c15),
            greedy: false,
        }
    };
}

#[macro_export]
macro_rules! choice_greedy {
    ($c0:expr, $c1:expr $(,)?) => {
        $crate::Choice2 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr $(,)?) => {
        $crate::Choice3 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr $(,)?) => {
        $crate::Choice4 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr $(,)?) => {
        $crate::Choice5 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr $(,)?) => {
        $crate::Choice6 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr $(,)?) => {
        $crate::Choice7 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr $(,)?) => {
        $crate::Choice8 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr $(,)?) => {
        $crate::Choice9 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            c8: $crate::choicen_helper($c8),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr $(,)?) => {
        $crate::Choice10 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            c8: $crate::choicen_helper($c8),
            c9: $crate::choicen_helper($c9),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr $(,)?) => {
        $crate::Choice11 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            c8: $crate::choicen_helper($c8),
            c9: $crate::choicen_helper($c9),
            c10: $crate::choicen_helper($c10),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr $(,)?) => {
        $crate::Choice12 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            c8: $crate::choicen_helper($c8),
            c9: $crate::choicen_helper($c9),
            c10: $crate::choicen_helper($c10),
            c11: $crate::choicen_helper($c11),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr $(,)?) => {
        $crate::Choice13 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            c8: $crate::choicen_helper($c8),
            c9: $crate::choicen_helper($c9),
            c10: $crate::choicen_helper($c10),
            c11: $crate::choicen_helper($c11),
            c12: $crate::choicen_helper($c12),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr, $c13:expr $(,)?) => {
        $crate::Choice14 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            c8: $crate::choicen_helper($c8),
            c9: $crate::choicen_helper($c9),
            c10: $crate::choicen_helper($c10),
            c11: $crate::choicen_helper($c11),
            c12: $crate::choicen_helper($c12),
            c13: $crate::choicen_helper($c13),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr, $c13:expr, $c14:expr $(,)?) => {
        $crate::Choice15 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            c8: $crate::choicen_helper($c8),
            c9: $crate::choicen_helper($c9),
            c10: $crate::choicen_helper($c10),
            c11: $crate::choicen_helper($c11),
            c12: $crate::choicen_helper($c12),
            c13: $crate::choicen_helper($c13),
            c14: $crate::choicen_helper($c14),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr, $c13:expr, $c14:expr, $c15:expr $(,)?) => {
        $crate::Choice16 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: $crate::choicen_helper($c1),
            c2: $crate::choicen_helper($c2),
            c3: $crate::choicen_helper($c3),
            c4: $crate::choicen_helper($c4),
            c5: $crate::choicen_helper($c5),
            c6: $crate::choicen_helper($c6),
            c7: $crate::choicen_helper($c7),
            c8: $crate::choicen_helper($c8),
            c9: $crate::choicen_helper($c9),
            c10: $crate::choicen_helper($c10),
            c11: $crate::choicen_helper($c11),
            c12: $crate::choicen_helper($c12),
            c13: $crate::choicen_helper($c13),
            c14: $crate::choicen_helper($c14),
            c15: $crate::choicen_helper($c15),
            greedy: true,
        }
    };
}