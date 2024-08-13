#![feature(assert_matches)]

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::combinators::*;
    use crate::combinators::cache_context;
    use crate::combinators::tag;
    use crate::fast_combinator::eat_char_fast;
    use crate::parse_state::{RightData};
    use crate::tests::utils::{assert_parses, assert_parses_default};
    use crate::unicode::get_unicode_general_category_combinator;
    use crate::unicode_categories::GeneralCategory;
    use crate::utils::{assert_fails, assert_fails_default, assert_fails_fast, assert_parses_fast};

    #[test]
    fn test_eat_u8() {
        assert_parses_default(&eat_char('a'), "a");
        assert_parses_fast(&eat_char('a'), "a");
    }

    #[test]
    fn test_eat_string() {
        assert_parses_default(&eat_string("abc"), "abc");
        assert_parses_fast(&eat_string("abc"), "abc");
    }

    #[test]
    fn test_seq() {
        assert_parses_default(&seq!(eat_char('a'), eat_char('b')), "ab");
        assert_parses_fast(&seq!(eat_char('a'), eat_char('b')), "ab");
    }

    #[test]
    fn test_repeat1() {
        assert_parses_default(&repeat1(eat_char('a')), "a");
        assert_parses_fast(&repeat1(eat_char('a')), "a");
        assert_parses_default(&repeat1(eat_char('a')), "aa");
        assert_parses_fast(&repeat1(eat_char('a')), "aa");
        assert_parses_default(&repeat1(eat_char('a')), "aaa");
        assert_parses_fast(&repeat1(eat_char('a')), "aaa");
    }

    #[test]
    fn test_choice() {
        assert_parses_default(&choice!(eat_char('a'), eat_char('b')), "a");
        assert_parses_fast(&choice!(eat_char('a'), eat_char('b')), "a");
        assert_parses_default(&choice!(eat_char('a'), eat_char('b')), "b");
        assert_parses_fast(&choice!(eat_char('a'), eat_char('b')), "b");
    }

    #[test]
    fn test_seq_choice_seq() {
        assert_parses_default(&seq!(choice!(eat_char('a'), seq!(eat_char('a'), eat_char('b'))), eat_char('c')), "ac");
        assert_parses_fast(&seq!(choice!(eat_char('a'), seq!(eat_char('a'), eat_char('b'))), eat_char('c')), "ac");
        assert_parses_default(&seq!(choice!(eat_char('a'), seq!(eat_char('a'), eat_char('b'))), eat_char('c')), "abc");
        assert_parses_fast(&seq!(choice!(eat_char('a'), seq!(eat_char('a'), eat_char('b'))), eat_char('c')), "abc");
    }

    #[test]
    fn test_seq_opt() {
        assert_parses_default(&seq!(opt(eat_char('a')), eat_char('b')), "ab");
        assert_parses_fast(&seq!(opt(eat_char('a')), eat_char('b')), "ab");
        assert_parses_default(&seq!(opt(eat_char('a')), eat_char('b')), "b");
        assert_parses_fast(&seq!(opt(eat_char('a')), eat_char('b')), "b");
    }

    #[test]
    fn test_strong_ref() {
        let mut combinator = strong_ref();
        combinator.set(choice!(seq!(eat_char('a'), &combinator), eat_char('b')));
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
    fn test_indents() {
        let combinator = seq!(
            eat_char('a'),
            eat_char('\n'),
            dent(),
            indent(),
            eat_char('b'),
            eat_char('\n'),
            dent(),
            dedent(),
            eat_char('c'),
        );
        // assert_parses_default(&combinator, "a\n b\nc");
        assert_parses_fast(&combinator, "a\n b\nc");
    }

    #[test]
    fn test_right_recursion_name_explosion() {
        let NAME = symbol(tag("repeat_a", seq!(forbid_follows(&[0]), repeat1(eat_char('a')))));

        let mut combinator_recursive = strong_ref();
        combinator_recursive.set(seq!(&NAME, &combinator_recursive));

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
        let a_combinator = symbol(cached(tag("A", eat_char('a'))));
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
        // assert_eq!(results.up_data_vec.len(), 0, "Expected no up data after the first step");
    }

    #[test]
    fn test_cache2() {
        let a_combinator = symbol(cached(tag("A", eat_char('a'))));
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
        // forward_decls!(A);
        // A.set(cached(seq!(eat_char('['), opt(&A), eat_char(']'))));
        let A = strong_ref();
        A.set(cached(seq!(eat_char('['), opt(&A), eat_char(']'))));
        let s_combinator = cache_context(&A);

        assert_parses_default(&s_combinator, "[]");
        assert_parses_fast(&s_combinator, "[]");
        assert_parses_default(&s_combinator, "[[]]");
        assert_parses_fast(&s_combinator, "[[]]");
    }

    #[test]
    fn test_cache_nested() {
        // It's useful to test both eat_char and eat_string here to make sure both work under a cache
        // A.set(tag("A", cached(seq!(eat_string("["), opt(seq!(&A, opt(&A))), eat_string("]")))));
        let A = strong_ref();
        A.set(cached(seq!(eat_string("["), opt(seq!(&A, opt(&A))), eat_string("]"))));
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
            choice!(seq!(eat_char('a'), &A), eat_char('b'))
        }

        let S: Combinator = From::from(&A);
        assert_parses_default(&S, "ab");
        assert_parses_fast(&S, "ab");
    }

    #[test]
    fn test_fast_parse() {
        let combinator = seq!(
            eat_char('a'),
            repeat0(eat_char('b')),
            eat_char('c'),
        );
        assert_parses_default(&combinator, "abc");
        assert_parses_fast(&combinator, "abc");
        assert_parses_default(&combinator, "abbbbbbbc");
        assert_parses_fast(&combinator, "abbbbbbbc");
    }

    #[test]
    fn test_fast_fail() {
        let combinator = seq!(
            eat_char('a'),
            repeat0(eat_char('b')),
            eat_char('c'),
        );
        assert_fails_fast(&combinator, "d");
    }

    #[test]
    fn test_ordered_choice() {
        let combinator = seq!(
            choice_greedy!(
                eat_char('a'),
                seq!(eat_char('a'), eat_char('a')),
            ),
            eat_char('b'),
        );
    }

    #[test]
    fn test_exclude_strings() {
        let combinator = seq!(
            exclude_strings(
                choice_greedy!(
                    eat('a'),
                    eat("aa"),
                ),
                vec!["a"]
            ),
            eat('b'),
        );
        assert_parses_default(&combinator, "aab");
        assert_parses_fast(&combinator, "aab");

        let combinator = seq!(
            choice_greedy!(
                eat('a'),
                eat("aa"),
            ),
            eat('b'),
        );
        assert_fails_default(&combinator, "aab");
        assert_fails_fast(&combinator, "aab");
    }

    #[test]
    fn test_eat_string_choice() {
        let combinator = eat_string_choice(&["ab", "cd"]);
        assert_parses_default(&combinator, "ab");
        assert_parses_fast(&combinator, "ab");
        assert_parses_default(&combinator, "cd");
        assert_parses_fast(&combinator, "cd");
        assert_fails_default(&combinator, "ac");
        assert_fails_fast(&combinator, "ac");
    }

    #[test]
    fn test_unicode() {
        // Test on numbers
        let combinator = get_unicode_general_category_combinator(GeneralCategory::Nd);
        assert_parses_default(&combinator, "1");
        assert_parses_fast(&combinator, "1");
    }

    #[test]
    fn brute_force_success_on_first_step() {
        let combinator = brute_force(|mut right_data, bytes| {
            if bytes == b"hello" {
                right_data.advance(5);
                parse_ok(right_data)
            } else if bytes.starts_with(b"hello") {
                right_data.advance(5);
                parse_incomplete()
            } else {
                parse_error()
            }
        });

        let (mut parser, parse_results) = combinator.parse(RightData::default(), b"hello");
        assert!(parse_results.done());
        assert_eq!(parse_results.right_data_vec.len(), 1);
        assert_eq!(parse_results.right_data_vec[0].right_data_inner.fields1.position, 5);
    }

    #[test]
    fn brute_force_success_on_second_step() {
        let combinator = brute_force(|mut right_data, bytes| {
            if bytes == b"hello" {
                right_data.advance(5);
                parse_ok(right_data)
            } else if bytes.starts_with(b"hello") {
                right_data.advance(5);
                parse_incomplete()
            } else {
                parse_error()
            }
        });

        let (mut parser, parse_results) = combinator.parse(RightData::default(), b"hell");
        assert!(!parse_results.done());
        assert_eq!(parse_results.right_data_vec.len(), 0);

        let parse_results = parser.parse(b"o");
        assert!(parse_results.done());
        assert_eq!(parse_results.right_data_vec.len(), 1);
        assert_eq!(parse_results.right_data_vec[0].right_data_inner.fields1.position, 5);
    }

    #[test]
    fn brute_force_failure() {
        let combinator = brute_force(|mut right_data, bytes| {
            if bytes == b"hello" {
                right_data.advance(5);
                parse_ok(right_data)
            } else if bytes.starts_with(b"hello") {
                right_data.advance(5);
                parse_incomplete()
            } else {
                parse_error()
            }
        });

        let (mut parser, parse_results) = combinator.parse(RightData::default(), b"hell");
        assert!(!parse_results.done());
        assert_eq!(parse_results.right_data_vec.len(), 0);

        let parse_results = parser.parse(b"p");
        assert!(parse_results.done());
        assert_eq!(parse_results.right_data_vec.len(), 0);
    }

    #[test]
    fn test_fast_combinator() {
        let combinator = fast_combinator(seq_fast!(
            choice_fast!(
                eat_char_fast('a'),
                eat_char_fast('b'),
            ),
            choice_fast!(
                eat_char_fast('c'),
                eat_char_fast('d'),
            ),
        ));

        assert_parses_default(&combinator, "ac");
        assert_parses_fast(&combinator, "ac");
        assert_parses_default(&combinator, "ad");
        assert_parses_fast(&combinator, "ad");
        assert_parses_default(&combinator, "bc");
        assert_parses_fast(&combinator, "bc");
        assert_parses_default(&combinator, "bd");
        assert_parses_fast(&combinator, "bd");
    }

    #[test]
    fn test_autoparse() {
        let combinator = choice!(
            eat_string("abcxx"),
            eat_string("abcyy"),
        );
        let (mut parser, _) = combinator.parser(RightData::default());
        let (prefix, parse_results) = parser.autoparse(10);
        assert_eq!(prefix, b"abc");
    }
}
