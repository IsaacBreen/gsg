use crate::*;
use crate::fast_combinator::*;

pub fn assert_fast_parser_parses(fast_parser: &(impl FastParserTrait + Clone), bytes: &[u8], expected_offset: usize) {
    let result = fast_parser.clone().parse(bytes);
    assert_eq!(result, FastParserResult::Success(expected_offset));
}

pub fn assert_fast_parser_fails(fast_parser: &(impl FastParserTrait + Clone), bytes: &[u8]) {
    let result = fast_parser.clone().parse(bytes);
    assert_eq!(result, FastParserResult::Failure);
}

pub fn assert_fast_parser_incomplete(fast_parser: &(impl FastParserTrait + Clone), bytes: &[u8]) {
    let result = fast_parser.clone().parse(bytes);
    assert_eq!(result, FastParserResult::Incomplete);
}

#[test]
fn test_eat_u8_parser() {
    let p = EatU8Parser { u8set: U8Set::from_char('a') };
    assert_fast_parser_parses(&p, b"a", 1);
    assert_fast_parser_fails(&p, b"b");
    assert_fast_parser_incomplete(&p, b"");
}

#[test]
fn test_seq() {
    let p = seq_fast!(eat_char_fast('a'), eat_char_fast('b'));
    assert_fast_parser_parses(&p, b"ab", 2);
    assert_fast_parser_fails(&p, b"ac");
    assert_fast_parser_incomplete(&p, b"a");
}

#[test]
fn test_seq_multiple() {
    let p = seq_fast!(eat_char_fast('a'), eat_char_fast('b'), eat_char_fast('c'));
    assert_fast_parser_parses(&p, b"abc", 3);
    assert_fast_parser_fails(&p, b"abd");
    assert_fast_parser_incomplete(&p, b"ab");
}

#[test]
fn test_choice() {
    let p = choice_fast!(eat_char_fast('a'), eat_char_fast('b'));
    assert_fast_parser_parses(&p, b"a", 1);
    assert_fast_parser_parses(&p, b"b", 1);
    assert_fast_parser_fails(&p, b"c");
    assert_fast_parser_incomplete(&p, b"");
}

#[test]
fn test_choice_multiple() {
    let p = choice_fast!(eat_char_fast('a'), eat_char_fast('b'), eat_char_fast('c'));
    assert_fast_parser_parses(&p, b"a", 1);
    assert_fast_parser_parses(&p, b"b", 1);
    assert_fast_parser_parses(&p, b"c", 1);
    assert_fast_parser_fails(&p, b"d");
    assert_fast_parser_incomplete(&p, b"");
}

#[test]
fn test_opt() {
    let p = opt_fast!(eat_char_fast('a'));
    assert_fast_parser_parses(&p, b"a", 1);
    assert_fast_parser_parses(&p, b"b", 0);
    assert_fast_parser_parses(&p, b"", 0);
}

#[test]
fn test_repeat1() {
    let p = repeat1_fast!(eat_char_fast('a'));
    assert_fast_parser_parses(&p, b"a", 1);
    assert_fast_parser_parses(&p, b"aa", 2);
    assert_fast_parser_parses(&p, b"aaa", 3);
    assert_fast_parser_fails(&p, b"b");
    assert_fast_parser_fails(&p, b"");
    assert_fast_parser_incomplete(&p, b"aaaa"); // Incomplete because we don't know if there are more 'a's
}

#[test]
fn test_repeat1_complex() {
    let p = repeat1_fast!(seq_fast!(eat_char_fast('a'), eat_char_fast('b')));
    assert_fast_parser_parses(&p, b"ab", 2);
    assert_fast_parser_parses(&p, b"abab", 4);
    assert_fast_parser_fails(&p, b"ac");
    assert_fast_parser_fails(&p, b"");
    assert_fast_parser_incomplete(&p, b"ababab"); // Incomplete because we don't know if there are more 'ab's
}

#[test]
fn test_complex_combinations() {
    let p = seq_fast!(
        eat_char_fast('a'),
        opt_fast!(eat_char_fast('b')),
        repeat1_fast!(choice_fast!(eat_char_fast('c'), eat_char_fast('d'))),
        eat_char_fast('e')
    );
    assert_fast_parser_parses(&p, b"ace", 3);
    assert_fast_parser_parses(&p, b"abce", 4);
    assert_fast_parser_parses(&p, b"acde", 4);
    assert_fast_parser_parses(&p, b"abcde", 5);
    assert_fast_parser_parses(&p, b"acccde", 6);
    assert_fast_parser_fails(&p, b"ac");
    assert_fast_parser_fails(&p, b"abceee");
    assert_fast_parser_incomplete(&p, b"acccd");
}