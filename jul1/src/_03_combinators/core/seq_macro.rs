use crate::IntoCombinator;

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
        fn convert<T: $crate::IntoCombinator>(x: T) -> Box<dyn $crate::CombinatorTrait> where T::Output {
            Box::new(x.into_combinator())
        }

        $crate::_seq(vec![convert($c0), convert($c1), convert($c2), convert($c3), convert($c4), convert($c5), convert($c6), convert($c7), convert($c8), convert($c9), convert($c10), convert($c11), convert($c12), convert($c13), convert($c14), convert($c15), $(convert($rest)),+])
    }};
}
