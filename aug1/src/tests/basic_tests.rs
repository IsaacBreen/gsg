use crate::{choice, eat_u8, seq, repeat1, eps, done, eat_string};
use crate::tests::utils::assert_parses;

#[test]
fn test_eat_u8() {
    let combinator = eat_u8(0);
    assert_parses(&combinator, &[0]);
}

#[test]
fn test_seq() {
    let combinator = seq!(eat_u8(0), eat_u8(1));
    assert_parses(&combinator, &[0, 1]);
}

#[test]
fn test_choice() {
    let combinator = choice!(eat_u8(0), eat_u8(1));
    assert_parses(&combinator, &[0]);
    assert_parses(&combinator, &[1]);
}

#[test]
fn test_repeat1() {
    let combinator = repeat1(eat_u8(0));
    assert_parses(&combinator, &[0]);
    assert_parses(&combinator, &[0, 0]);
    assert_parses(&combinator, &[0, 0, 0]);
}

#[test]
fn test_eps() {
    let combinator = eps();
    assert_parses(&combinator, &[]);
}

#[test]
fn test_done() {
    let combinator = done();
    assert_parses(&combinator, &[]);
}

#[test]
fn test_eat_string() {
    let combinator = eat_string(vec![0, 1, 2]);
    assert_parses(&combinator, &[0, 1, 2]);
}
