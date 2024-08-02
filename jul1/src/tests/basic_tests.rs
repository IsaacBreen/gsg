#![feature(assert_matches)]

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::combinators::*;
    use crate::combinators::cache_context;
    use crate::combinators::tag;
    use crate::parse_state::{RightData, UpData};
    use crate::tests::utils::{assert_parses, assert_parses_default};
    use crate::utils::{assert_fails, assert_fails_default, assert_fails_fast, assert_parses_fast};

    #[test]
    fn test_eat_u8() {
        assert_parses_default(&eat_char_choice("a"), "a");
        assert_parses_fast(&eat_char_choice("a"), "a");
    }

    #[test]
    fn test_eat_string() {
        assert_parses_default(&eat_string("abc"), "abc");
        assert_parses_fast(&eat_string("abc"), "abc");
    }

    #[test]
    fn test_seq() {
        assert_parses_default(&seq!(eat_char_choice("a"), eat_char_choice("b")), "ab");
        assert_parses_fast(&seq!(eat_char_choice("a"), eat_char_choice("b")), "ab");
    }

    #[test]
    fn test_repeat1() {
        assert_parses_default(&repeat1(eat_char_choice("a")), "a");
        assert_parses_fast(&repeat1(eat_char_choice("a")), "a");
        assert_parses_default(&repeat1(eat_char_choice("a")), "aa");
        assert_parses_fast(&repeat1(eat_char_choice("a")), "aa");
        assert_parses_default(&repeat1(eat_char_choice("a")), "aaa");
        assert_parses_fast(&repeat1(eat_char_choice("a")), "aaa");
    }

    #[test]
    fn test_choice() {
        assert_parses_default(&choice!(eat_char_choice("a"), eat_char_choice("b")), "a");
        assert_parses_fast(&choice!(eat_char_choice("a"), eat_char_choice("b")), "a");
        assert_parses_default(&choice!(eat_char_choice("a"), eat_char_choice("b")), "b");
        assert_parses_fast(&choice!(eat_char_choice("a"), eat_char_choice("b")), "b");
    }

    #[test]
    fn test_seq_choice_seq() {
        assert_parses_default(&seq!(choice!(eat_char_choice("a"), seq!(eat_char_choice("a"), eat_char_choice("b"))), eat_char_choice("c")), "ac");
        assert_parses_fast(&seq!(choice!(eat_char_choice("a"), seq!(eat_char_choice("a"), eat_char_choice("b"))), eat_char_choice("c")), "ac");
        assert_parses_default(&seq!(choice!(eat_char_choice("a"), seq!(eat_char_choice("a"), eat_char_choice("b"))), eat_char_choice("c")), "abc");
        assert_parses_fast(&seq!(choice!(eat_char_choice("a"), seq!(eat_char_choice("a"), eat_char_choice("b"))), eat_char_choice("c")), "abc");
    }

    #[test]
    fn test_seq_opt() {
        assert_parses_default(&seq!(opt(eat_char_choice("a")), eat_char_choice("b")), "ab");
        assert_parses_fast(&seq!(opt(eat_char_choice("a")), eat_char_choice("b")), "ab");
        assert_parses_default(&seq!(opt(eat_char_choice("a")), eat_char_choice("b")), "b");
        assert_parses_fast(&seq!(opt(eat_char_choice("a")), eat_char_choice("b")), "b");
    }

    #[test]
    fn test_forward_ref() {
        let mut combinator = forward_ref();
        combinator.set(choice!(seq!(eat_char_choice("a"), &combinator), eat_char_choice("b")));
        assert_parses_default(&combinator, "b");
        assert_parses_fast(&combinator, "b");
        assert_parses_fast(&combinator, "b");
        assert_parses_default(&combinator, "ab");
        assert_parses_fast(&combinator, "ab");
        assert_parses_fast(&combinator, "ab");
        assert_parses_default(&combinator, "aab");
        assert_parses_fast(&combinator, "aab");
        assert_parses_default(&combinator, "aaab");
        assert_parses_fast(&combinator, "aaab");
    }

    #[test]
    #[ignore]
    fn test_frame_stack_contains() {
        let mut right_data = RightData::default();
        right_data.frame_stack.as_mut().unwrap().push_name(b"a");
        let combinator = frame_stack_contains(eat_char_choice("a"));
        assert_parses(&combinator, "a", "Frame stack contains 'a'");
        assert_parses_fast(&combinator, "a");

        let combinator = frame_stack_contains(eat_char_choice("b"));
        assert_fails(&combinator, "b", "Frame stack does not contain 'b'");
        assert_parses_fast(&combinator, "b");
    }

    #[test]
    #[ignore]
    fn test_frame_stack_push() {
        let combinator = seq!(push_to_frame(eat_char_choice("a")), frame_stack_contains(choice!(eat_char_choice("b"), eat_char_choice("a"))));
        assert_parses_default(&combinator, "ab");
    }

    #[test]
    #[ignore]
    fn test_frame_stack_pop() {
        let combinator = seq!(
            push_to_frame(eat_char_choice("a")),
            frame_stack_contains(choice!(eat_char_choice("b"), eat_char_choice("a"))),
            pop_from_frame(eat_char_choice("a")),
            frame_stack_contains(eat_char_choice("a"))
        );
        assert_fails_default(&combinator, "aaa");
        assert_parses_fast(&combinator, "aaa");
    }

    #[test]
    #[ignore]
    fn test_frame_stack_push_empty_frame() {
        let combinator = seq!(
            eat_char_choice("{"),
            with_new_frame(seq!(
                push_to_frame(eat_char_choice("a")), eat_char_choice("="), eat_char_choice("b"), eat_char_choice(";"),
                frame_stack_contains(eat_char_choice("a")),
            )),
            eat_char_choice("}"),
            frame_stack_contains(eat_char_choice("a")),
        );
        assert_fails_default(&combinator, "{a=b;}a");
        assert_parses_fast(&combinator, "{a=b;}a");
    }

    #[test]
    fn test_indents() {
        let combinator = seq!(
            eat_char_choice("a"),
            eat_char('\n'),
            dent(),
            indent(),
            eat_char_choice("b"),
            eat_char('\n'),
            dent(),
            dedent(),
            eat_char_choice("c"),
        );
        // assert_parses_default(&combinator, "a\n b\nc");
        assert_parses_fast(&combinator, "a\n b\nc");
    }

    #[test]
    fn test_right_recursion_name_explosion() {
        let NAME = symbol(tag("repeat_a", seq!(forbid_follows(&[0]), repeat1(eat_char('a')))));

        let mut combinator_recursive = forward_ref();
        let combinator_recursive = combinator_recursive.set(seq!(&NAME, &combinator_recursive));

        let combinator_repeat1 = repeat1(&NAME);

        let (mut parser_recursive, _) = combinator_recursive.parser(RightData::default());
        let (mut parser_repeat1, _) = combinator_repeat1.parser(RightData::default());

        for i in 0..10 {
            parser_recursive.step('a' as u8);
            parser_repeat1.step('a' as u8);
            let stats_recursive = parser_recursive.stats();
            let stats_repeat1 = parser_repeat1.stats();
            if i > 5 {
                assert!(stats_recursive.total_active_tags() > stats_repeat1.total_active_tags(), 
                    "Expected recursive parser to have more active tags than repeat1 parser, but found {} > {}", 
                    stats_recursive.total_active_tags(), stats_repeat1.total_active_tags());
            }
        }
    }

    #[test]
    fn test_cache() {
        let a_combinator = symbol(cached(tag("A", eat_char_choice("a"))));
        let s_combinator = cache_context(choice!(&a_combinator, &a_combinator));

        assert_parses_default(&s_combinator, "a");
        assert_parses_fast(&s_combinator, "a");

        let (mut parser, _) = s_combinator.parser(RightData::default());

        let stats = parser.stats();
        assert_eq!(stats.active_tags["A"], 1, "Expected one tag 'A' to be active initially");

        let results = parser.step('a' as u8).squashed();

        let stats = parser.stats();
        assert_eq!(stats.active_tags.len(), 0, "Expected no active tags after the first step");

        assert_eq!(results.right_data_vec.len(), 1, "Expected one right data after the first step");
        assert_eq!(results.up_data_vec.len(), 0, "Expected no up data after the first step");
    }

    #[test]
    fn test_cache2() {
        let a_combinator = symbol(cached(tag("A", eat_char_choice("a"))));
        let s_combinator = cache_context(choice!(&a_combinator, &a_combinator));

        assert_parses_default(&s_combinator, "a");
        assert_parses_fast(&s_combinator, "a");
    }

    #[test]
    fn test_cache3() {
        let a_combinator = symbol(cached(tag("A", eat_string("aa"))));
        let s_combinator = cache_context(seq!(eat_char('b'), choice!(&a_combinator, &a_combinator), eat_char('c')));

        // assert_parses_default(&s_combinator, "baac");
        assert_parses_fast(&s_combinator, "baac");
    }

    #[test]
    fn test_cache_nested_simple() {
        forward_decls!(A);
        A.set(cached(seq!(eat_char('['), opt(&A), eat_char(']'))));
        let s_combinator = cache_context(&A);

        assert_parses_default(&s_combinator, "[]");
        assert_parses_fast(&s_combinator, "[]");
        assert_parses_default(&s_combinator, "[[]]");
        assert_parses_fast(&s_combinator, "[[]]");
    }

    #[test]
    fn test_cache_nested() {
        forward_decls!(A);
        // It's useful to test both eat_char and eat_string here to make sure both work under a cache
        // A.set(tag("A", cached(seq!(eat_char('['), opt(seq!(&A, opt(&A))), eat_char(']')))));
        A.set(tag("A", cached(seq!(eat_string("["), opt(seq!(&A, opt(&A))), eat_string("]")))));
        let s_combinator = cache_context(&A);

        assert_parses_default(&s_combinator, "[]");
        assert_parses_fast(&s_combinator, "[]");
        assert_parses_default(&s_combinator, "[[]]");
        assert_parses_fast(&s_combinator, "[[]]");
        assert_parses_default(&s_combinator, "[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]");
        assert_parses_fast(&s_combinator, "[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]");
        assert_parses_default(&s_combinator, "[[][]]");
        assert_parses_fast(&s_combinator, "[[][]]");
        assert_parses_default(&s_combinator, "[[][[][]]]");
        assert_parses_fast(&s_combinator, "[[][[][]]]");
    }

    #[test]
    fn test_from_fn() {
        fn A() -> Combinator {
            choice!(seq!(eat_char('a'), &A as &dyn Fn() -> Combinator), eat_char('b'))
        }

        let S: Combinator = From::<&dyn Fn() -> Combinator>::from(&A);
        assert_parses_default(&S, "ab");
        assert_parses_fast(&S, "ab");
    }

    #[test]
    fn test_fast_parse() {
        let combinator = seq!(
            eat_char_choice("a"),
            repeat0(eat_char_choice("b")),
            eat_char_choice("c"),
        );
        assert_parses_default(&combinator, "abc");
        assert_parses_fast(&combinator, "abc");
        assert_parses_default(&combinator, "abbbbbbbc");
        assert_parses_fast(&combinator, "abbbbbbbc");
    }

    #[test]
    fn test_fast_fail() {
        let combinator = seq!(
            eat_char_choice("a"),
            repeat0(eat_char_choice("b")),
            eat_char_choice("c"),
        );
        assert_fails_fast(&combinator, "d");
    }
}
