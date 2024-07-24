#![feature(assert_matches)]


#[cfg(test)]
mod tests {
    use crate::*;
    use crate::combinators::*;
    use crate::combinators::cache_context;
    use crate::combinators::tag;
    use crate::parse_state::{RightData, UpData};
    use crate::tests::utils::assert_parses;

    #[test]
    fn test_eat_u8() {
        let combinator = eat_char_choice("a");
        let (mut parser, ParseResults { right_data_vec: right_data0, up_data_vec: up_data0, cut } ) = combinator.parser(RightData::default());
        assert_eq!((right_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8), ParseResults {
            right_data_vec: vec![RightData::default()],
            up_data_vec: vec![],
            cut: false,
        });
    }

    #[test]
    fn test_eat_string() {
        let combinator = eat_string("abc");
        let (mut parser, ParseResults { right_data_vec: right_data0, up_data_vec: up_data0, cut } ) = combinator.parser(RightData::default());
        assert_eq!((right_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("b") }],
            cut: false,
        });
        assert_eq!(parser.step('b' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("c") }],
            cut: false,
        });
        assert_eq!(parser.step('c' as u8).squashed(), ParseResults {
            right_data_vec: vec![RightData::default()],
            up_data_vec: vec![],
            cut: false,
        });
    }

    #[test]
    fn test_seq() {
        let combinator = seq!(eat_char_choice("a"), eat_char_choice("b"));
        let (mut parser, ParseResults { right_data_vec: right_data0, up_data_vec: up_data0, cut } ) = combinator.parser(RightData::default());
        assert_eq!((right_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("b") }],
            cut: false,
        });
        assert_eq!(parser.step('b' as u8), ParseResults {
            right_data_vec: vec![RightData::default()],
            up_data_vec: vec![],
            cut: false,
        });
    }

    #[test]
    fn test_repeat1() {
        let combinator = repeat1(eat_char_choice("a"));
        let (mut parser, ParseResults { right_data_vec: right_data0, up_data_vec: up_data0, cut } ) = combinator.parser(RightData::default());
        assert_eq!(Squash::squashed(ParseResults {
            right_data_vec: right_data0,
            up_data_vec: up_data0,
            cut: false,
        }), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("a") }],
            cut: false,
        });
        assert_eq!(parser.step('a' as u8).squashed(), ParseResults {
            right_data_vec: vec![RightData::default()],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("a") }],
            cut: false,
        });
        assert_eq!(parser.step('a' as u8).squashed(), ParseResults {
            right_data_vec: vec![RightData::default()],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("a") }],
            cut: false,
        });
    }

    #[test]
    fn test_choice() {
        let combinator = choice!(eat_char_choice("a"), eat_char_choice("b"));
        let (mut parser, ParseResults { right_data_vec: right_data0, up_data_vec: up_data0, cut } ) = combinator.parser(RightData::default());
        assert_eq!((right_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }, UpData { u8set: U8Set::from_chars("b") }].squashed()));
        assert_eq!(parser.step('b' as u8), ParseResults {
            right_data_vec: vec![RightData::default()],
            up_data_vec: vec![],
            cut: false,
        });
    }

    #[test]
    fn test_seq_choice_seq() {
        let combinator = seq!(choice!(eat_char_choice("a"), seq!(eat_char_choice("a"), eat_char_choice("b"))), eat_char_choice("c"));
        let (mut parser, ParseResults { right_data_vec: right_data0, up_data_vec: up_data0, cut } ) = combinator.parser(RightData::default());
        assert_eq!(Squash::squashed(ParseResults {
            right_data_vec: right_data0,
            up_data_vec: up_data0,
            cut: false,
        }), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("a") }],
            cut: false,
        });
        assert_eq!(parser.step('a' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("bc") }],
            cut: false,
        });
        assert_eq!(parser.step('b' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("c") }],
            cut: false,
        });
        assert_eq!(parser.step('c' as u8).squashed(), ParseResults {
            right_data_vec: vec![RightData::default()],
            up_data_vec: vec![],
            cut: false,
        });
    }

    #[test]
    fn test_seq_opt() {
        let combinator = seq!(opt(eat_char_choice("a")), eat_char_choice("b"));
        let (mut parser, ParseResults { right_data_vec: right_data0, up_data_vec: up_data0, cut } ) = combinator.parser(RightData::default());
        assert_eq!(Squash::squashed(ParseResults {
            right_data_vec: right_data0,
            up_data_vec: up_data0,
            cut: false,
        }), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("ab") }],
            cut: false,
        });
        assert_eq!(parser.step('a' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("b") }],
            cut: false,
        });
        assert_eq!(parser.step('b' as u8).squashed(), ParseResults {
            right_data_vec: vec![RightData::default()],
            up_data_vec: vec![],
            cut: false,
        });
    }

    #[test]
    fn test_forward_ref() {
        let mut combinator = forward_ref();
        combinator.set(choice!(seq!(eat_char_choice("a"), &combinator), eat_char_choice("b")));
        let (mut parser, ParseResults { right_data_vec: right_data0, up_data_vec: up_data0, cut } ) = combinator.parser(RightData::default());
        assert_eq!(Squash::squashed(ParseResults {
            right_data_vec: right_data0,
            up_data_vec: up_data0,
            cut: false,
        }), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("ab") }],
            cut: false,
        });
        assert_eq!(parser.step('a' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("ab") }],
            cut: false,
        });
        assert_eq!(parser.step('a' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("ab") }],
            cut: false,
        });
        assert_eq!(parser.step('b' as u8).squashed(), ParseResults {
            right_data_vec: vec![RightData::default()],
            up_data_vec: vec![],
            cut: false,
        });
    }

    #[test]
    fn test_frame_stack_contains() {
        let mut frame_stack = FrameStack::default();
        frame_stack.push_name(b"a");
        let mut right_data = RightData::default();
        right_data.frame_stack.as_mut().unwrap().push_name(b"a");
        let combinator = frame_stack_contains(eat_char_choice("a"));
        let (mut parser, ParseResults { right_data_vec: right_data0, up_data_vec: up_data0, cut }) = combinator.parser(right_data.clone());
        assert_eq!((right_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8), ParseResults {
            right_data_vec: vec![right_data.clone()],
            up_data_vec: vec![],
            cut: false,
        });

        let combinator = frame_stack_contains(eat_char_choice("b"));
        let (mut parser, ParseResults { right_data_vec: right_data0, up_data_vec: up_data0, cut }) = combinator.parser(right_data);
        assert_eq!(ParseResults {
            right_data_vec: right_data0,
            up_data_vec: up_data0,
            cut: false,
        }.squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            cut: false,
        });
    }

    #[test]
    fn test_frame_stack_push() {
        let mut frame_stack = FrameStack::default();
        let right_data = RightData::default();
        let combinator = seq!(push_to_frame(eat_char_choice("a")), frame_stack_contains(choice!(eat_char_choice("b"), eat_char_choice("a"))));
        let (mut parser, ParseResults { right_data_vec: right_data0, up_data_vec: up_data0, cut }) = combinator.parser(right_data.clone());
        assert_eq!((right_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("a") }],
            cut: false,
        });
        assert_eq!(parser.step('b' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            cut: false,
        });
    }

    #[test]
    fn test_frame_stack_pop() {
        let mut frame_stack = FrameStack::default();
        let right_data = RightData::default();
        let combinator = seq!(
            push_to_frame(eat_char_choice("a")),
            frame_stack_contains(choice!(eat_char_choice("b"), eat_char_choice("a"))),
            pop_from_frame(eat_char_choice("a")),
            frame_stack_contains(eat_char_choice("a"))
        );
        let (mut parser, ParseResults { right_data_vec: right_data0, up_data_vec: up_data0, cut }) = combinator.parser(right_data);
        //     // Parsing goes like this:
        //     //
        //     // 1. "a" is pushed to the frame stack.
        //     // 2. the choice says the next character is "b" or "a", but the frame stack only contains "a", so it only allows "a".
        //     // 3. the pop_from_frame parser pops the "a" from the frame stack.
        //     // 4. eat_chars("a") says the next character is "a", but the frame stack is empty, so it doesn't allow anything, and parsing fails.
        //     //
        //     // i.e. "aaaa" should fail on the final "a".
        //     //
        //     // 1. "a" is pushed to the frame stack.
        assert_eq!((right_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        //     // 2. the choice says the next character is "b" or "a", but the frame stack only contains "a", so it only allows "a".
        assert_eq!(parser.step('a' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("a") }],
            cut: false,
        });
        //     // 3. the pop_from_frame parser pops the "a" from the frame stack.
        assert_eq!(parser.step('a' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("a") }],
            cut: false,
        });
        //     // 4. eat_chars("a") says the next character is "a", but the frame stack is empty, so it doesn't allow anything, and parsing fails.
        assert_eq!(parser.step('a' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            cut: false,
        });
    }

    #[test]
    fn test_frame_stack_push_empty_frame() {
        let mut frame_stack = FrameStack::default();
        let right_data = RightData::default();
        let combinator = seq!(
            eat_char_choice("{"),
            with_new_frame(seq!(
                push_to_frame(eat_char_choice("a")), eat_char_choice("="), eat_char_choice("b"), eat_char_choice(";"),
                frame_stack_contains(eat_char_choice("a")),
            )),
            eat_char_choice("}"),
            frame_stack_contains(eat_char_choice("a")),
        );
        let (mut parser, ParseResults { right_data_vec: right_data0, up_data_vec: up_data0, cut }) = combinator.parser(right_data);
        assert_eq!((right_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("{") }]));
        assert_eq!(parser.step('{' as u8), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("a") }],
            cut: false,
        });
        assert_eq!(parser.step('a' as u8), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("=") }],
            cut: false,
        });
        assert_eq!(parser.step('=' as u8), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("b") }],
            cut: false,
        });
        assert_eq!(parser.step('b' as u8), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars(";") }],
            cut: false,
        });
        assert_eq!(parser.step(';' as u8), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("a") }],
            cut: false,
        });
        assert_eq!(parser.step('a' as u8), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("}") }],
            cut: false,
        });
        assert_eq!(parser.step('}' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            cut: false,
        });
        assert_eq!(parser.step('a' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            cut: false,
        });
    }

    #[test]
    fn test_indents() {
        pub fn newline() -> Seq2<Choice2<Repeat1<EatU8>, Eps>, EatU8> {
            seq!(repeat0(eat_char_choice(" ")), eat_char_choice("\n"))
        }

        pub fn python_newline() -> Seq2<Repeat1<Seq2<Choice2<Repeat1<EatU8>, Eps>, EatU8>>, IndentCombinator> {
            seq!(repeat1(newline()), dent())
        }

        let mut frame_stack = FrameStack::default();
        let parse_data = RightData::default();
        let combinator = seq!(
            eat_char_choice("a"),
            python_newline(),
            with_indent(seq!(
                eat_char_choice("b"),
                python_newline(),
            )),
            eat_char_choice("c"),
        );
        let (mut parser, ParseResults { right_data_vec: right_data0, up_data_vec: up_data0, cut }) = combinator.parser(parse_data);
        assert_eq!(ParseResults {
            right_data_vec: right_data0,
            up_data_vec: up_data0,
            cut: false,
        }.squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("a") }],
            cut: false,
        });
        assert_eq!(parser.step('a' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("\n ") }],
            cut: false,
        });
        assert_eq!(parser.step('\n' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("\n ") }],
            cut: false,
        });
        assert_eq!(parser.step(' ' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("\n b") }],
            cut: false,
        });
        assert_eq!(parser.step('b' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("\n ") }],
            cut: false,
        });
        assert_eq!(parser.step('\n' as u8).squashed(), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData { u8set: U8Set::from_chars("\n c") }],
            cut: false,
        });
        assert_eq!(parser.step('c' as u8).squashed(), ParseResults {
            right_data_vec: vec![RightData::default()],
            up_data_vec: vec![],
            cut: false,
        });
    }

    #[test]
    fn test_right_recursion_name_explosion() {
        // Based on a Python slowdown issue.
        let NAME = tag("repeat_a", seq!(prevent_consecutive_matches("repeat_a"), repeat1(eat_char('a')))).into_rc_dyn();

        let mut combinator_recursive = forward_ref();
        let combinator_recursive = combinator_recursive.set(seq!(&NAME, &combinator_recursive));

        let combinator_repeat1 = repeat1(&NAME);

        let (mut parser_recursive, parse_results0_recursive) = combinator_recursive.parser(RightData::default());
        let (mut parser_repeat1, parse_results0_repeat1) = combinator_repeat1.parser(RightData::default());

        // Repeat "a" 10 times.
        for i in 0..10 {
            let parser_recursive_results = parser_recursive.step('a' as u8);
            let parser_repeat1_results = parser_repeat1.step('a' as u8);
            let stats_recursive = parser_recursive.stats();
            let stats_repeat1 = parser_repeat1.stats();
            println!("stats_recursive:{}", stats_recursive);
            println!("stats_repeat1:{}", stats_repeat1);
            if i > 5 {
                assert!(stats_recursive.total_active_tags() > stats_repeat1.total_active_tags());
            }
        }
    }

    #[test]
    fn test_cache() {
        // Define the grammar
        let a_combinator = cached(tag("A", eat_char_choice("a")));
        let s_combinator = cache_context(choice!(&a_combinator, &a_combinator));

        assert_parses(&s_combinator, "a", "Test input");

        // Initialize the parser
        let (mut parser, ParseResults { right_data_vec: _, up_data_vec: _, cut }) = s_combinator.parser(RightData::default());

        {
            // Check stats
            let stats = parser.stats();
            assert_eq!(stats.active_tags["A"], 1, "Expected one tag 'A' to be active initially");

            // Check initial cache state
            let initial_cache_state = parser.cache_data_inner.borrow();
            assert_eq!(initial_cache_state.new_parsers.len(), 1, "Expected one tag 'A' to be active initially");
            assert_eq!(initial_cache_state.existing_parsers.len(), 0, "Expected no existing parsers initially");
        }

        // Perform the first parsing step
        let results = parser.step('a' as u8).squashed();

        {
            // Check stats
            let stats = parser.stats();
            assert_eq!(stats.active_tags["A"], 1, "Expected one tag 'A' to be active after the first step");

            // Check the cache state after the first step
            let cache_state_after_step = parser.cache_data_inner.borrow();
            assert_eq!(cache_state_after_step.new_parsers.len(), 0, "Expected no new parsers after the first step");
            assert_eq!(cache_state_after_step.existing_parsers.len(), 1, "Expected one existing parser after the first step");
            assert_eq!(results.right_data_vec.len(), 1, "Expected one right data after the first step");
            assert_eq!(results.up_data_vec.len(), 0, "Expected no up data after the first step");
        }
    }

    #[test]
    fn test_cache2() {
        // Define the grammar
        let a_combinator = cached(tag("A", eat_char_choice("a")));
        let s_combinator = cache_context(choice!(&a_combinator, &a_combinator));

        assert_parses(&s_combinator, "a", "Test input");
    }

    #[test]
    fn test_cache_nested() {
        // Define the grammar
        forward_decls!(A);
        A.set(tag("A", cached(choice!(seq!(eat_string("["), opt(seq!(&A, opt(&A))), eat_string("]"))))));
        let s_combinator = cache_context(A);

        // let s = "[]";
        // let s = "[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]";
        // let s = "[[][]]";
        let s = "[[][[][]]]";
        assert_parses(&s_combinator, s, "Test input");
    }
}