use std::rc::Rc;
use std::collections::BTreeMap;
use crate::{Combinator, CombinatorTrait, FailParser, Parser, ParseResults, ParserTrait, profile, RightData, RightDataSquasher, U8Set, VecY, vecx, Fail};
use crate::SeqParser;

#[macro_export]
macro_rules! define_seq {
    ($seq_name:ident, $first:ident, $($rest:ident),+) => {
        #[derive(Debug)]
        pub struct $seq_name<$first, $($rest),+>
        where
            $($rest: CombinatorTrait),+
        {
            pub(crate) $first: $first,
            $(pub(crate) $rest: Rc<$rest>),+
        }

        impl<$first, $($rest),+> CombinatorTrait for $seq_name<$first, $($rest),+>
        where
            $first: CombinatorTrait + 'static,
            $($rest: CombinatorTrait + 'static),+
        {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
                let start_position = right_data.right_data_inner.fields1.position;

                let first_combinator = &self.$first;
                let (first_parser, first_parse_results) = profile!(stringify!($seq_name, " first child parse"), {
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
                    let (parser, parse_results) = profile!(stringify!($seq_name, " child parse"), {
                        next_combinator.parse(right_data, &bytes[offset..])
                    });
                    if !parse_results.done() {
                        parsers.push((parsers.len(), parser));
                    }
                    parse_results.right_data_vec
                }

                $(
                    let mut next_next_right_data_vec = VecY::new();
                    for right_data in next_right_data_vec {
                        next_next_right_data_vec.extend(helper(right_data, &self.$rest, &bytes, start_position, &mut parsers));
                    }
                    next_right_data_vec = next_next_right_data_vec;
                )+

                final_right_data = next_right_data_vec;

                if parsers.is_empty() {
                    return (Parser::FailParser(FailParser), ParseResults::new(final_right_data, true));
                }

                let parser = Parser::SeqParser(SeqParser {
                    parsers,
                    combinators: Rc::new(vecx![Combinator::Fail(Fail), $(Combinator::DynRc(self.$rest.clone())),+]),
                    position: start_position + bytes.len(),
                });

                let parse_results = ParseResults::new(final_right_data, false);

                (parser.into(), parse_results)
            }
        }
    };
}

define_seq!(Seq2, c0, c1);
define_seq!(Seq3, c0, c1, c2);
define_seq!(Seq4, c0, c1, c2, c3);
define_seq!(Seq5, c0, c1, c2, c3, c4);
define_seq!(Seq6, c0, c1, c2, c3, c4, c5);
define_seq!(Seq7, c0, c1, c2, c3, c4, c5, c6);
define_seq!(Seq8, c0, c1, c2, c3, c4, c5, c6, c7);
define_seq!(Seq9, c0, c1, c2, c3, c4, c5, c6, c7, c8);

#[macro_export]
macro_rules! seqn {
    ($c0:expr, $c1:expr) => {
        $crate::Seq2 {
            c0: $c0,
            c1: Rc::new($c1),
        }
    };
    ($c0:expr, $c1:expr, $c2:expr) => {
        $crate::Seq3 {
            c0: $c0,
            c1: Rc::new($c1),
            c2: Rc::new($c2),
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr) => {
        $crate::Seq4 {
            c0: $c0,
            c1: Rc::new($c1),
            c2: Rc::new($c2),
            c3: Rc::new($c3),
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr) => {
        $crate::Seq5 {
            c0: $c0,
            c1: Rc::new($c1),
            c2: Rc::new($c2),
            c3: Rc::new($c3),
            c4: Rc::new($c4),
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr) => {
        $crate::Seq6 {
            c0: $c0,
            c1: Rc::new($c1),
            c2: Rc::new($c2),
            c3: Rc::new($c3),
            c4: Rc::new($c4),
            c5: Rc::new($c5),
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr) => {
        $crate::Seq7 {
            c0: $c0,
            c1: Rc::new($c1),
            c2: Rc::new($c2),
            c3: Rc::new($c3),
            c4: Rc::new($c4),
            c5: Rc::new($c5),
            c6: Rc::new($c6),
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr) => {
        $crate::Seq8 {
            c0: $c0,
            c1: Rc::new($c1),
            c2: Rc::new($c2),
            c3: Rc::new($c3),
            c4: Rc::new($c4),
            c5: Rc::new($c5),
            c6: Rc::new($c6),
            c7: Rc::new($c7),
        }
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr) => {
        $crate::Seq9 {
            c0: $c0,
            c1: Rc::new($c1),
            c2: Rc::new($c2),
            c3: Rc::new($c3),
            c4: Rc::new($c4),
            c5: Rc::new($c5),
            c6: Rc::new($c6),
            c7: Rc::new($c7),
            c8: Rc::new($c8),
        }
    };
}
