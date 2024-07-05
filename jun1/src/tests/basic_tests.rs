#![feature(assert_matches)]

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use std::rc::Rc;
    use crate::{ActiveCombinator, choice, Combinator, CombinatorState, eat_string, eat_u8, opt, ParserIterationResult, repeat1, seq, U8Set};

    // Test cases remain the same, just update the combinator creation syntax
    #[test]
    fn test_eat_u8() {
        let c = eat_u8('a');
        let mut it = ActiveCombinator::new(c);
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("a"));
        let result = it.send(Some('a'));
        assert_matches!(result, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set.is_empty());
    }

    #[test]
    fn test_eat_string() {
        let mut it = ActiveCombinator::new(eat_string("abc"));
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("a"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("b"));
        let result2 = it.send(Some('b'));
        assert_matches!(result2, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("c"));
        let result3 = it.send(Some('c'));
        assert_matches!(result3, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set.is_empty());
    }

    #[test]
    fn test_seq() {
        let mut it = ActiveCombinator::new(seq!(eat_u8('a'), eat_u8('b')));
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("a"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("b"));
        let result2 = it.send(Some('b'));
        assert_matches!(result2, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set.is_empty());
    }

    #[test]
    fn test_repeat1() {
        let mut it = ActiveCombinator::new(repeat1(eat_u8('a')));
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("a"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set == &U8Set::from_chars("a"));
        let result2 = it.send(Some('a'));
        assert_matches!(result2, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set == &U8Set::from_chars("a"));
    }

    #[test]
    fn test_choice() {
        let mut it = ActiveCombinator::new(choice!(eat_u8('a'), eat_u8('b')));
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("ab"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set.is_empty());

        let mut it = ActiveCombinator::new(choice!(eat_u8('a'), eat_u8('b')));
        it.send(None);
        let result2 = it.send(Some('b'));
        assert_matches!(result2, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set.is_empty());
    }

    #[test]
    fn test_seq_choice_seq() {
        // Matches "ac" or "abc"
        let mut it = ActiveCombinator::new(
            seq!(
                choice!(eat_u8('a'), seq!(eat_u8('a'), eat_u8('b'))),
                eat_u8('c')
            ),
        );
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("a"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("bc"));
        let result2 = it.send(Some('b'));
        assert_matches!(result2, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("c"));
        let result3 = it.send(Some('c'));
        assert_matches!(result3, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set.is_empty());
    }

    #[test]
    fn test_seq_opt() {
        // Matches "ac" or "abc"
        let mut it = ActiveCombinator::new(
            seq!(
                opt(eat_u8('b')),
                eat_u8('c')
            ),
        );
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("bc"));
        let result1 = it.send(Some('b'));
        assert_matches!(result1, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("c"));
        let result2 = it.send(Some('c'));
        assert_matches!(result2, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set.is_empty());
    }
}
