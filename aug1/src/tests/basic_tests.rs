use crate::{eat_u8, seq};
use crate::tests::utils::assert_parses;

#[test]
fn eat_seq() {
    let combinator = seq!(eat_u8(0), eat_u8(1));
    assert_parses(combinator, "01");
}