#![feature(assert_matches)]

#[cfg(test)]
mod tests {
    use crate::{choice, eat_chars, eat_string, forward_ref, frame_stack_contains, FrameStack, opt, ParseData, ParseResult, push_to_frame, repeat1, seq, U8Set};
    use crate::combinator::*;
    use crate::pop_from_frame;

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
    fn test_frame_stack_contains() {
        let mut frame_stack = FrameStack::default();
        frame_stack.push_name(b"a");
        let parse_data = ParseData::new(frame_stack.clone());
        let combinator = frame_stack_contains(eat_chars("a"));
        let (mut parser, result0) = combinator.parser(parse_data.clone());
        assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::none(), Some(parse_data.clone())));

        let combinator = frame_stack_contains(eat_chars("b"));
        let (mut parser, result0) = combinator.parser(parse_data);
        assert_eq!(result0, ParseResult::new(U8Set::none(), None));
    }

    #[test]
    fn test_frame_stack_push() {
        let mut frame_stack = FrameStack::default();
        let parse_data = ParseData::new(frame_stack.clone());
        let combinator = seq!(push_to_frame(eat_chars("a")), frame_stack_contains(choice!(eat_chars("b"), eat_chars("a"))));
        let (mut parser, result0) = combinator.parser(parse_data.clone());
        assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("a"), None));
        assert_eq!(parser.step('b' as u8), ParseResult::default());
    }

    #[test]
    fn test_frame_stack_pop() {
        let mut frame_stack = FrameStack::default();
        let parse_data = ParseData::new(frame_stack.clone());
        let combinator = seq!(
            push_to_frame(eat_chars("a")),
            frame_stack_contains(choice!(eat_chars("b"), eat_chars("a"))),
            pop_from_frame(eat_chars("a")),
            frame_stack_contains(eat_chars("a"))
        );
        let (mut parser, result0) = combinator.parser(parse_data.clone());
        assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        // Parsing goes like this:
        //
        // 1. "a" is pushed to the frame stack.
        // 2. the choice says the next character is "b" or "a", but the frame stack only contains "a", so it only allows "a".
        // 3. the pop_from_frame parser pops the "a" from the frame stack.
        // 4. eat_chars("a") says the next character is "a", but the frame stack is empty, so it doesn't allow anything, and parsing fails.
        //
        // i.e. "aaaa" should fail on the final "a".
        //
        // 1. "a" is pushed to the frame stack.
        assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("a"), None));
        // 2. the choice says the next character is "b" or "a", but the frame stack only contains "a", so it only allows "a".
        assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("a"), None));
        // 3. the pop_from_frame parser pops the "a" from the frame stack.
        assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::none(), None));
        // 4. eat_chars("a") says the next character is "a", but the frame stack is empty, so it doesn't allow anything, and parsing fails.
    }
}
