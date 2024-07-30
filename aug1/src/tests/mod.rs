use crate::{eat_u8, seq};

mod basic_tests;

#[test]
fn eat_seq() {
    let combinator = seq!(eat_u8(0), eat_u8(1));
}