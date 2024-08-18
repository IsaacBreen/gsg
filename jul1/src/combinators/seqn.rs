use std::rc::Rc;
use std::collections::BTreeMap;
use crate::{Combinator, CombinatorTrait, FailParser, Parser, ParseResults, ParserTrait, profile, RightData, RightDataSquasher, U8Set, VecY, vecx, Fail, IntoCombinator};

macro_rules! profile {
    ($name:expr, $body:expr) => {
        $body
    };
}

#[macro_export]
macro_rules! define_seq {
    ($seq_name:ident, $seq_parser_name:ident, $first:ident, $($rest:ident),+ $(,)?) => {
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
            $($rest: CombinatorTrait),+
        {
            pub(crate) combinator: &'a $seq_name<$first, $($rest),+>,
            pub(crate) $first: Vec<Parser<'a>>,
            $(pub(crate) $rest: Vec<Parser<'a>>,)+
            pub(crate) position: usize,
        }

        impl<$first, $($rest),+> CombinatorTrait for $seq_name<$first, $($rest),+>
        where
            $first: CombinatorTrait + 'static,
            $($rest: CombinatorTrait + 'static),+
        {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
                f(&self.$first);
                $(f(&self.$rest);)+
            }

            fn parse<'a>(&'a self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'a>, ParseResults) {
                let start_position = right_data.right_data_inner.fields1.position;

                let first_combinator = &self.$first;
                let (first_parser, first_parse_results) = profile!(stringify!($seq_name, " first child parse"), {
                    first_combinator.parse(right_data, &bytes)
                });

                let mut all_done = first_parse_results.done();
                if all_done && first_parse_results.right_data_vec.is_empty() {
                    // Shortcut
                    return (Parser::FailParser(FailParser), first_parse_results);
                }

                let mut parser = $seq_parser_name {
                    combinator: self,
                    $first: if all_done { vec![] } else { vec![first_parser] },
                    $($rest: vec![],)+
                    position: start_position + bytes.len(),
                };

                let mut next_right_data_vec = first_parse_results.right_data_vec;

                fn helper<'a, T: CombinatorTrait>(right_data: RightData, next_combinator: &'a T, bytes: &[u8], start_position: usize) -> (Parser<'a>, ParseResults) {
                    let offset = right_data.right_data_inner.fields1.position - start_position;
                    profile!(stringify!($seq_name, " child parse"), {
                        next_combinator.parse(right_data, &bytes[offset..])
                    })
                }

                // Macro to process each child combinator
                $(
                    if next_right_data_vec.is_empty() {
                        return (Parser::DynParser(Box::new(parser)), ParseResults::empty(all_done));
                    }

                    let mut next_next_right_data_vec = VecY::new();
                    let mut $rest = Vec::new();
                    for right_data in next_right_data_vec {
                        let (parser, parse_results) = helper(right_data, &self.$rest, &bytes, start_position);
                        all_done &= parse_results.done();
                        if !parse_results.done() {
                            $rest.push(parser);
                        }
                        next_next_right_data_vec.extend(parse_results.right_data_vec);
                    }
                    next_right_data_vec = next_next_right_data_vec;

                    // Update the parser with the new parsers for this child
                    parser.$rest = $rest;
                )+

                let parse_results = ParseResults::new(next_right_data_vec, all_done);

                (Parser::DynParser(Box::new(parser)), parse_results)
            }
        }

        impl<'a, $first, $($rest),+> ParserTrait for $seq_parser_name<'a, $first, $($rest),+>
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
                profile!(stringify!($seq_parser_name, "::parse"), {
                    let mut new_right_data: VecY<RightData> = VecY::new();

                    // First child
                    self.$first.retain_mut(|parser| {
                        let parse_results = profile!(stringify!($seq_parser_name, "::parse child Parser::parse"), {
                            parser.parse(bytes)
                        });
                        let done = parse_results.done();
                        if !done {
                            return true;
                        }
                        if self.combinator.children().len() == 1 {
                            new_right_data.extend(parse_results.right_data_vec);
                        }
                        false
                    });

                    $(
                        // Subsequent children
                        for right_data in std::mem::take(&mut new_right_data) {
                            let offset = right_data.right_data_inner.fields1.position - self.position;
                            let combinator = &self.combinator.$rest;
                            let (parser, parse_results) = profile!(stringify!($seq_parser_name, "::parse child Combinator::parse"), {
                                combinator.parse(right_data, &bytes[offset..])
                            });
                            if !parse_results.done() {
                                self.$rest.push(parser);
                            }
                            if self.combinator.children().len() == 2 {
                                new_right_data.extend(parse_results.right_data_vec);
                            }
                        }

                        self.$rest.retain_mut(|parser| {
                            let parse_results = profile!(stringify!($seq_parser_name, "::parse child Parser::parse"), {
                                parser.parse(bytes)
                            });
                            let done = parse_results.done();
                            if !done {
                                return true;
                            }
                            if self.combinator.children().len() == 2 {
                                new_right_data.extend(parse_results.right_data_vec);
                            }
                            false
                        });
                    )+

                    self.position += bytes.len();

                    ParseResults::new(new_right_data, self.$first.is_empty() $( && self.$rest.is_empty() )+)
                })
            }
        }

        impl<$first, $($rest),+> $seq_name<$first, $($rest),+>
        where
            $first: CombinatorTrait,
            $($rest: CombinatorTrait),+
        {
            fn children(&self) -> Vec<&dyn CombinatorTrait> {
                vec![&self.$first, $( &self.$rest ),+]
            }
        }
    };
}

define_seq!(Seq2, Seq2Parser, c0, c1);
define_seq!(Seq3, Seq3Parser, c0, c1, c2);
define_seq!(Seq4, Seq4Parser, c0, c1, c2, c3);
define_seq!(Seq5, Seq5Parser, c0, c1, c2, c3, c4);
define_seq!(Seq6, Seq6Parser, c0, c1, c2, c3, c4, c5);
define_seq!(Seq7, Seq7Parser, c0, c1, c2, c3, c4, c5, c6);
define_seq!(Seq8, Seq8Parser, c0, c1, c2, c3, c4, c5, c6, c7);
define_seq!(Seq9, Seq9Parser, c0, c1, c2, c3, c4, c5, c6, c7, c8);
define_seq!(Seq10, Seq10Parser, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
define_seq!(Seq11, Seq11Parser, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10);
define_seq!(Seq12, Seq12Parser, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11);
define_seq!(Seq13, Seq13Parser, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12);
define_seq!(Seq14, Seq14Parser, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13);
define_seq!(Seq15, Seq15Parser, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14);
define_seq!(Seq16, Seq16Parser, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15);

pub fn seqn_helper<T: IntoCombinator>(x: T) -> T::Output {
    IntoCombinator::into_combinator(x)
}

#[macro_export]
macro_rules! seq {
    ($c0:expr $(,)?) => {{
        $c0
    }};
    ($c0:expr, $c1:expr $(,)?) => {{
        use $crate::seqn_helper as h;
        $crate::Seq2 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
        }
    }};
    ($c0:expr, $c1:expr, $c2:expr $(,)?) => {{
        use $crate::seqn_helper as h;
        $crate::Seq3 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
        }
    }};
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr $(,)?) => {{
        use $crate::seqn_helper as h;
        $crate::Seq4 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
        }
    }};
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr $(,)?) => {{
        use $crate::seqn_helper as h;
        $crate::Seq5 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
        }
    }};
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr $(,)?) => {{
        use $crate::seqn_helper as h;
        $crate::Seq6 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
            c5: h($c5),
        }
    }};
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr $(,)?) => {{
        use $crate::seqn_helper as h;
        $crate::Seq7 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
            c5: h($c5),
            c6: h($c6),
        }
    }};
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr $(,)?) => {{
        use $crate::seqn_helper as h;
        $crate::Seq8 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
            c5: h($c5),
            c6: h($c6),
            c7: h($c7),
        }
    }};
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr $(,)?) => {{
        use $crate::seqn_helper as h;
        $crate::Seq9 {
            c0: $crate::IntoCombinator::into_combinator($c0),
            c1: h($c1),
            c2: h($c2),
            c3: h($c3),
            c4: h($c4),
            c5: h($c5),
            c6: h($c6),
            c7: h($c7),
            c8: h($c8),
        }
    }};
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr $(,)?) => {{
        use $crate::seqn_helper as h;
        $crate::Seq10 {
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
        }
    }};
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr $(,)?) => {{
        use $crate::seqn_helper as h;
        $crate::Seq11 {
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
        }
    }};
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr $(,)?) => {{
        use $crate::seqn_helper as h;
        $crate::Seq12 {
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
        }
    }};
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr $(,)?) => {{
        use $crate::seqn_helper as h;
        $crate::Seq13 {
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
        }
    }};
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr, $c13:expr $(,)?) => {{
        use $crate::seqn_helper as h;
        $crate::Seq14 {
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
        }
    }};
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr, $c13:expr, $c14:expr $(,)?) => {{
        use $crate::seqn_helper as h;
        $crate::Seq15 {
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
        }
    }};
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr, $c13:expr, $c14:expr, $c15:expr $(,)?) => {{
        use $crate::seqn_helper as h;
        $crate::Seq16 {
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
        }
    }};
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr, $c8:expr, $c9:expr, $c10:expr, $c11:expr, $c12:expr, $c13:expr, $c14:expr, $c15:expr, $($rest:expr),+ $(,)?) => {{
        fn convert(x: impl $crate::IntoCombinator + 'static) -> Box<dyn $crate::CombinatorTrait> {
            Box::new(x.into_combinator())
        }
        $crate::_seq(vec![convert($c0), convert($c1), convert($c2), convert($c3), convert($c4), convert($c5), convert($c6), convert($c7), convert($c8), convert($c9), convert($c10), convert($c11), convert($c12), convert($c13), convert($c14), convert($c15), $(convert($rest)),+])
    }};
}