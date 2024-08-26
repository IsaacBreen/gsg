// src/tests/basic_tests.rs
#![feature(assert_matches)]
use crate::*;
use crate::combinators::*;
use crate::fast_combinator::eat_char_fast;
use crate::parse_state::RightData;
use crate::tests::utils::{assert_fails_fast, assert_parses_default, assert_parses_fast, assert_parses_one_shot, assert_parses_one_shot_with_result};
use crate::unicode::get_unicode_general_category_combinator;
use crate::unicode_categories::GeneralCategory;

#[test]
fn test_eat_char() {
    let combinator = eat_char('a');
    assert_parses_default(&combinator, "a");
    assert_parses_fast(&combinator, "a");
    assert_parses_one_shot(&combinator, "a");
}

#[test]
fn test_eat_string() {
    let combinator = eat_string("abc");
    assert_parses_default(&combinator, "abc");
    assert_parses_fast(&combinator, "abc");
    assert_parses_one_shot(&combinator, "abc");
}

#[test]
fn test_seq() {
    let combinator = seq!(eat_char('a'), eat_char('b'));
    assert_parses_default(&combinator, "ab");
    assert_parses_fast(&combinator, "ab");
    assert_parses_one_shot(&combinator, "ab");
}

#[test]
fn test_repeat1() {
    let combinator = repeat1(eat_char('a'));
    assert_parses_default(&combinator, "a");
    assert_parses_fast(&combinator, "a");
    assert_parses_one_shot_with_result(&combinator, "a", Err(UnambiguousParseError::Incomplete));

    assert_parses_default(&combinator, "aa");
    assert_parses_fast(&combinator, "aa");
    assert_parses_one_shot_with_result(&combinator, "aa", Err(UnambiguousParseError::Ambiguous));

    let combinator = seq!(repeat1(eat_char('a')), eat_char('b'));
    assert_parses_default(&combinator, "aaab");
    assert_parses_fast(&combinator, "aaab");
    assert_parses_one_shot_with_result(&combinator, "aaab", Err(UnambiguousParseError::Ambiguous));
}

#[test]
fn test_repeat1_greedy() {
    let combinator = repeat1_greedy(eat_char('a'));
    assert_parses_default(&combinator, "a");
    assert_parses_fast(&combinator, "a");
    assert_parses_one_shot_with_result(&combinator, "a", Err(UnambiguousParseError::Incomplete));

    assert_parses_default(&combinator, "aa");
    assert_parses_fast(&combinator, "aa");
    assert_parses_one_shot_with_result(&combinator, "aa", Err(UnambiguousParseError::Incomplete));

    assert_parses_default(&combinator, "aaa");
    assert_parses_fast(&combinator, "aaa");
    assert_parses_one_shot_with_result(&combinator, "aaa", Err(UnambiguousParseError::Incomplete));

    let combinator = seq!(repeat1_greedy(eat_char('a')), eat_char('b'));
    assert_parses_default(&combinator, "aaab");
    assert_parses_fast(&combinator, "aaab");
    assert_parses_one_shot_with_result(&combinator, "aaab", Ok(RightData::default().with_position(4)));
}

#[test]
fn test_choice() {
    let combinator = choice!(eat_char('a'), eat_char('b'));
    assert_parses_default(&combinator, "a");
    assert_parses_fast(&combinator, "a");
    assert_parses_one_shot(&combinator, "a");

    assert_parses_default(&combinator, "b");
    assert_parses_fast(&combinator, "b");
    assert_parses_one_shot(&combinator, "b");
}

#[test]
fn test_seq_choice_seq() {
    let combinator = seq!(choice!(eat_char('a'), seq!(eat_char('a'), eat_char('b'))), eat_char('c'));
    assert_parses_default(&combinator, "ac");
    assert_parses_fast(&combinator, "ac");
    assert_parses_one_shot(&combinator, "ac");

    assert_parses_default(&combinator, "abc");
    assert_parses_fast(&combinator, "abc");
    assert_parses_one_shot(&combinator, "abc");
}

#[test]
fn test_seq_opt() {
    let combinator = seq!(opt(eat_char('a')), eat_char('b'));
    assert_parses_default(&combinator, "ab");
    assert_parses_fast(&combinator, "ab");
    // Even local ambiguity causes an ambiguous parse error.
    // In this case, opt(eat_char('a')) is ambiguous when parsed with 'a', (it can match either 'a' or the empty string).
    // So even though seq!(opt(eat_char('a')), eat_char('b')) is unambiguous, it still causes an ambiguous parse error.
    // To avoid it, use the greedy version of opt, opt_greedy.
    assert_parses_one_shot_with_result(&combinator, "ab", Err(UnambiguousParseError::Ambiguous));

    assert_parses_default(&combinator, "b");
    assert_parses_fast(&combinator, "b");
    assert_parses_one_shot(&combinator, "b");
}

#[test]
fn test_seq_opt_greedy() {
    let combinator = seq!(opt_greedy(eat_char('a')), eat_char('b'));
    assert_parses_default(&combinator, "ab");
    assert_parses_fast(&combinator, "ab");
    assert_parses_one_shot(&combinator, "ab");

    assert_parses_default(&combinator, "b");
    assert_parses_fast(&combinator, "b");
    assert_parses_one_shot(&combinator, "b");
}

#[test]
fn test_strong_ref() {
    let mut combinator = strong_ref();
    combinator.set(choice!(seq!(eat_char('a'), combinator.clone().into_dyn()), eat_char('b')));

    assert_parses_default(&combinator, "b");
    assert_parses_fast(&combinator, "b");
    assert_parses_one_shot(&combinator, "b");

    assert_parses_default(&combinator, "ab");
    assert_parses_fast(&combinator, "ab");
    assert_parses_one_shot(&combinator, "ab");

    assert_parses_default(&combinator, "aab");
    assert_parses_fast(&combinator, "aab");
    assert_parses_one_shot(&combinator, "aab");

    assert_parses_default(&combinator, "aaab");
    assert_parses_fast(&combinator, "aaab");
    assert_parses_one_shot(&combinator, "aaab");
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
    assert_parses_default(&combinator, "a\n b\nc");
    assert_parses_fast(&combinator, "a\n b\nc");
}

#[test]
fn test_exclude_strings() {
    let combinator = seq!(
        exclude_strings(
            choice!(
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
        choice!(
            eat('a'),
            eat("aa"),
        ),
        eat('b'),
    );
    assert_parses_default(&combinator, "ab");
    assert_parses_fast(&combinator, "ab");
    assert_parses_default(&combinator, "aab");
    assert_parses_fast(&combinator, "aab");
}

#[test]
fn test_eat_string_choice() {
    let combinator = eat_string_choice(&["ab", "cd"]);
    assert_parses_default(&combinator, "ab");
    assert_parses_fast(&combinator, "ab");
    assert_parses_default(&combinator, "cd");
    assert_parses_fast(&combinator, "cd");
}

#[test]
fn test_unicode() {
    // Test on numbers
    let combinator = get_unicode_general_category_combinator(GeneralCategory::Nd);
    assert_parses_default(&combinator, "1");
    assert_parses_fast(&combinator, "1");
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