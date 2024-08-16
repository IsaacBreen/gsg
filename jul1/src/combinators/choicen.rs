use std::rc::Rc;
use std::collections::BTreeMap;
use crate::{CombinatorTrait, FailParser, Parser, ParseResults, ParserTrait, profile, RightData, RightDataSquasher, U8Set, VecY, vecx, Fail};
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

#[macro_export]
macro_rules! choice {
    ($c0:expr, $c1:expr $(,)?) => {
        $crate::Choice2 {
            c0: crate::IntoCombinator::into_combinator($c0),
            c1: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c1)),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr $(,)?) => {
        $crate::Choice3 {
            c0: crate::IntoCombinator::into_combinator($c0),
            c1: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c1)),
            c2: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c2)),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr $(,)?) => {
        $crate::Choice4 {
            c0: crate::IntoCombinator::into_combinator($c0),
            c1: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c1)),
            c2: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c2)),
            c3: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c3)),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr $(,)?) => {
        $crate::Choice5 {
            c0: crate::IntoCombinator::into_combinator($c0),
            c1: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c1)),
            c2: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c2)),
            c3: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c3)),
            c4: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c4)),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr $(,)?) => {
        $crate::Choice6 {
            c0: crate::IntoCombinator::into_combinator($c0),
            c1: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c1)),
            c2: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c2)),
            c3: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c3)),
            c4: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c4)),
            c5: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c5)),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr $(,)?) => {
        $crate::Choice7 {
            c0: crate::IntoCombinator::into_combinator($c0),
            c1: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c1)),
            c2: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c2)),
            c3: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c3)),
            c4: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c4)),
            c5: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c5)),
            c6: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c6)),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr $(,)?) => {
        $crate::Choice8 {
            c0: crate::IntoCombinator::into_combinator($c0),
            c1: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c1)),
            c2: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c2)),
            c3: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c3)),
            c4: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c4)),
            c5: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c5)),
            c6: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c6)),
            c7: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c7)),
            greedy: false,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr $(,)?) => {
        $crate::Choice9 {
            c0: crate::IntoCombinator::into_combinator($c0),
            c1: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c1)),
            c2: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c2)),
            c3: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c3)),
            c4: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c4)),
            c5: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c5)),
            c6: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c6)),
            c7: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c7)),
            c8: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c8)),
            greedy: false,
        }
    };
}

#[macro_export]
macro_rules! choice_greedy {
    ($c0:expr, $c1:expr $(,)?) => {
        $crate::Choice2 {
            c0: crate::IntoCombinator::into_combinator($c0),
            c1: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c1)),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr $(,)?) => {
        $crate::Choice3 {
            c0: crate::IntoCombinator::into_combinator($c0),
            c1: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c1)),
            c2: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c2)),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr $(,)?) => {
        $crate::Choice4 {
            c0: crate::IntoCombinator::into_combinator($c0),
            c1: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c1)),
            c2: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c2)),
            c3: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c3)),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr $(,)?) => {
        $crate::Choice5 {
            c0: crate::IntoCombinator::into_combinator($c0),
            c1: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c1)),
            c2: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c2)),
            c3: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c3)),
            c4: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c4)),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr $(,)?) => {
        $crate::Choice6 {
            c0: crate::IntoCombinator::into_combinator($c0),
            c1: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c1)),
            c2: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c2)),
            c3: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c3)),
            c4: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c4)),
            c5: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c5)),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr $(,)?) => {
        $crate::Choice7 {
            c0: crate::IntoCombinator::into_combinator($c0),
            c1: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c1)),
            c2: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c2)),
            c3: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c3)),
            c4: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c4)),
            c5: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c5)),
            c6: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c6)),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr $(,)?) => {
        $crate::Choice8 {
            c0: crate::IntoCombinator::into_combinator($c0),
            c1: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c1)),
            c2: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c2)),
            c3: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c3)),
            c4: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c4)),
            c5: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c5)),
            c6: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c6)),
            c7: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c7)),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr $(,)?) => {
        $crate::Choice9 {
            c0: crate::IntoCombinator::into_combinator($c0),
            c1: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c1)),
            c2: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c2)),
            c3: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c3)),
            c4: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c4)),
            c5: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c5)),
            c6: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c6)),
            c7: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c7)),
            c8: std::rc::Rc::new(crate::IntoCombinator::into_combinator($c8)),
            greedy: true,
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $($rest:expr),+) => {{
        fn convert(x: impl $crate::IntoCombinator + 'static) -> Box<dyn $crate::CombinatorTrait> {
            Box::new(x.into_combinator())
        }
        $crate::_choice(vec![convert($c0), convert($c1), convert($c2), convert($c3), convert($c4), convert($c5), convert($c6), convert($c7), convert($c8), $(convert($rest)),+])
    }};
}