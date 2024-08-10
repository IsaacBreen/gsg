use crate::*;
use crate::fast_combinator::*;

pub fn assert_fast_parser_parses(fast_parser: &(impl FastParserTrait + Clone), bytes: &[u8], expected_offset: usize) {
    let result = fast_parser.clone().parse(bytes);
    assert_eq!(result, FastParserResult::Success(expected_offset));
}


#[test]
fn test_seq() {
    let p = seq_fast!(eat_char_fast('a'), eat_char_fast('b'));
    assert_fast_parser_parses(&p, b"ab", 2);
}