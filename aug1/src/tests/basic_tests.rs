use crate::{choice, eat_u8, seq};
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