#![feature(assert_matches)]

#[cfg(test)]
mod tests {
    use crate::{choice, eat_chars, eat_string, forward_ref, frame_stack_contains, FrameStack, opt, ParseData, ParseResult, repeat1, seq, U8Set};
    use crate::combinator::*;

    #[test]
    fn test_eat_u8() {
        let combinator = eat_chars("a");
        let (mut parser, result0) = combinator.parser(ParseData::default());
        assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::none(), Some(ParseData::default())));
    }

    #[test]
    fn test_eat_string() {
        let combinator = eat_string("abc");
        let (mut parser, result0) = combinator.parser(ParseData::default());
        assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("b"), None));
        assert_eq!(parser.step('b' as u8), ParseResult::new(U8Set::from_chars("c"), None));
        assert_eq!(parser.step('c' as u8), ParseResult::new(U8Set::none(), Some(ParseData::default())));
    }

    #[test]
    fn test_seq() {
        let combinator = seq!(eat_chars("a"), eat_chars("b"));
        let (mut parser, result0) = combinator.parser(ParseData::default());
        assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("b"), None));
        assert_eq!(parser.step('b' as u8), ParseResult::new(U8Set::none(), Some(ParseData::default())));
    }

    #[test]
    fn test_repeat1() {
        let combinator = repeat1(eat_chars("a"));
        let (mut parser, result0) = combinator.parser(ParseData::default());
        assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("a"), Some(ParseData::default())));
        assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("a"), Some(ParseData::default())));
    }

    #[test]
    fn test_choice() {
        let combinator = choice!(eat_chars("a"), eat_chars("b"));
        let (mut parser, result0) = combinator.parser(ParseData::default());
        assert_eq!(result0, ParseResult::new(U8Set::from_chars("ab"), None));
        assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars(""), Some(ParseData::default())));
    }

    #[test]
    fn test_seq_choice_seq() {
        let combinator = seq!(choice!(eat_chars("a"), seq!(eat_chars("a"), eat_chars("b"))), eat_chars("c"));
        let (mut parser, result0) = combinator.parser(ParseData::default());
        assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("bc"), None));
        assert_eq!(parser.step('b' as u8), ParseResult::new(U8Set::from_chars("c"), None));
        assert_eq!(parser.step('c' as u8), ParseResult::new(U8Set::none(), Some(ParseData::default())));
    }

    #[test]
    fn test_seq_opt() {
        let combinator = seq!(opt(eat_chars("a")), eat_chars("b"));
        let (mut parser, result0) = combinator.parser(ParseData::default());
        assert_eq!(result0, ParseResult::new(U8Set::from_chars("ab"), None));
        assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("b"), None));
        assert_eq!(parser.step('b' as u8), ParseResult::new(U8Set::none(), Some(ParseData::default())));
    }

    #[test]
    fn test_forward_ref() {
        let mut A = forward_ref();
        A.set(choice!(seq!(eat_chars("a"), A.clone()), eat_chars("b")));
        let combinator = A.clone();
        let (mut parser, result0) = combinator.parser(ParseData::default());
        assert_eq!(result0, ParseResult::new(U8Set::from_chars("ab"), None));
        assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("ab"), None));
        assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("ab"), None));
        assert_eq!(parser.step('b' as u8), ParseResult::new(U8Set::none(), Some(ParseData::default())));
    }

    #[test]
    fn test_frame_stack_ops() {
        let mut frame_stack = FrameStack::default();
        frame_stack.push_name(b"a");
        let mut parse_data = ParseData::new();
        parse_data.insert("frame_stack".to_string(), frame_stack.clone());
        let combinator = frame_stack_contains(eat_chars("a"));
        let (mut parser, result0) = combinator.parser(parse_data.clone());
        assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::none(), Some(parse_data.clone())));

        let combinator = frame_stack_contains(eat_chars("b"));
        let (mut parser, result0) = combinator.parser(parse_data);
        assert_eq!(result0, ParseResult::new(U8Set::none(), None));
    }
}
