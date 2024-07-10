#![feature(assert_matches)]

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use crate::*;
    use crate::combinators::*;
    use crate::parse_state::{HorizontalData, VerticalData};
    // use crate::frame_stack::FrameStack;
    // use crate::pop_from_frame;

    #[test]
    fn test_eat_u8() {
        // let combinator = eat_chars("a");
        // let (mut parser, result0) = combinator.parser(ParseData::default());
        // assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        // assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::none(), Some(ParseData::default())));
        let combinator = eat_char_choice("a");
        let (mut parser, horizontal_data0, vertical_data0) = combinator.parser(HorizontalData::default());
        assert_eq!((horizontal_data0, vertical_data0), (vec![], vec![VerticalData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8), (vec![HorizontalData::default()], vec![]));
    }

    #[test]
    fn test_eat_string() {
        // let combinator = eat_string("abc");
        // let (mut parser, result0) = combinator.parser(ParseData::default());
        // assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        // assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("b"), None));
        // assert_eq!(parser.step('b' as u8), ParseResult::new(U8Set::from_chars("c"), None));
        // assert_eq!(parser.step('c' as u8), ParseResult::new(U8Set::none(), Some(ParseData::default())));
        let combinator = eat_string("abc");
        let (mut parser, horizontal_data0, vertical_data0) = combinator.parser(HorizontalData::default());
        assert_eq!((horizontal_data0, vertical_data0), (vec![], vec![VerticalData { u8set: U8Set::from_chars("a") }]));
        let (horizontal_data1, vertical_data1) = parser.step('a' as u8);
        assert_eq!((horizontal_data1, vertical_data1), (vec![], vec![VerticalData { u8set: U8Set::from_chars("b") }]));
        let (horizontal_data2, vertical_data2) = parser.step('b' as u8);
        assert_eq!((horizontal_data2, vertical_data2), (vec![], vec![VerticalData { u8set: U8Set::from_chars("c") }]));
        let (horizontal_data3, vertical_data3) = parser.step('c' as u8);
        assert_eq!((horizontal_data3, vertical_data3), (vec![HorizontalData::default()], vec![]));
    }

    #[test]
    fn test_seq() {
        // let combinator = seq!(eat_chars("a"), eat_chars("b"));
        // let (mut parser, result0) = combinator.parser(ParseData::default());
        // assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        // assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("b"), None));
        // assert_eq!(parser.step('b' as u8), ParseResult::new(U8Set::none(), Some(ParseData::default())));
        let combinator = seq!(eat_char_choice("a"), eat_char_choice("b"));
        let (mut parser, horizontal_data0, vertical_data0) = combinator.parser(HorizontalData::default());
        assert_eq!((horizontal_data0, vertical_data0), (vec![], vec![VerticalData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8), (vec![], vec![VerticalData { u8set: U8Set::from_chars("b") }]));
        assert_eq!(parser.step('b' as u8), (vec![HorizontalData::default()], vec![]));
    }

    #[test]
    fn test_repeat1() {
        // let combinator = repeat1(eat_chars("a"));
        // let (mut parser, result0) = combinator.parser(ParseData::default());
        // assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        // assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("a"), Some(ParseData::default())));
        // assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("a"), Some(ParseData::default())));
        let combinator = repeat1(eat_char_choice("a"));
        let (mut parser, horizontal_data0, vertical_data0) = combinator.parser(HorizontalData::default());
        assert_eq!((horizontal_data0, VerticalData::squash(vertical_data0)), (vec![], vec![VerticalData { u8set: U8Set::from_chars("a") }]));
        let (horizontal_data1, vertical_data1) = parser.step('a' as u8);
        assert_eq!((horizontal_data1, VerticalData::squash(vertical_data1)), (vec![HorizontalData::default()], vec![VerticalData { u8set: U8Set::from_chars("a") }]));
        let (horizontal_data2, vertical_data2) = parser.step('a' as u8);
        assert_eq!((horizontal_data2, VerticalData::squash(vertical_data2)), (vec![HorizontalData::default()], vec![VerticalData { u8set: U8Set::from_chars("a") }]));
    }

    #[test]
    fn test_choice() {
        // let combinator = choice!(eat_chars("a"), eat_chars("b"));
        // let (mut parser, result0) = combinator.parser(ParseData::default());
        // assert_eq!(result0, ParseResult::new(U8Set::from_chars("ab"), None));
        // assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars(""), Some(ParseData::default())));
        let combinator = choice!(eat_char_choice("a"), eat_char_choice("b"));
        let (mut parser, horizontal_data0, vertical_data0) = combinator.parser(HorizontalData::default());
        assert_eq!((horizontal_data0, vertical_data0), (vec![], vec![VerticalData { u8set: U8Set::from_chars("a") }, VerticalData { u8set: U8Set::from_chars("b") }]));
        assert_eq!(parser.step('b' as u8), (vec![HorizontalData::default()], vec![]));
    }

    #[test]
    fn test_seq_choice_seq() {
        // let combinator = seq!(choice!(eat_chars("a"), seq!(eat_chars("a"), eat_chars("b"))), eat_chars("c"));
        // let (mut parser, result0) = combinator.parser(ParseData::default());
        // assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        // assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("bc"), None));
        // assert_eq!(parser.step('b' as u8), ParseResult::new(U8Set::from_chars("c"), None));
        // assert_eq!(parser.step('c' as u8), ParseResult::new(U8Set::none(), Some(ParseData::default())));
        let combinator = seq!(choice!(eat_char_choice("a"), seq!(eat_char_choice("a"), eat_char_choice("b"))), eat_char_choice("c"));
        let (mut parser, horizontal_data0, vertical_data0) = combinator.parser(HorizontalData::default());
        assert_eq!((horizontal_data0, VerticalData::squash(vertical_data0)), (vec![], vec![VerticalData { u8set: U8Set::from_chars("a") }]));
        let (horizontal_data1, vertical_data1) = parser.step('a' as u8);
        assert_eq!((horizontal_data1, VerticalData::squash(vertical_data1)), (vec![], vec![VerticalData { u8set: U8Set::from_chars("bc") }]));
        let (horizontal_data2, vertical_data2) = parser.step('b' as u8);
        assert_eq!((horizontal_data2, VerticalData::squash(vertical_data2)), (vec![], vec![VerticalData { u8set: U8Set::from_chars("c") }]));
        let (horizontal_data3, vertical_data3) = parser.step('c' as u8);
        assert_eq!((horizontal_data3, vertical_data3), (vec![HorizontalData::default(), HorizontalData::default()], vec![]));
    }

    #[test]
    fn test_seq_opt() {
        // let combinator = seq!(opt(eat_chars("a")), eat_chars("b"));
        // let (mut parser, result0) = combinator.parser(ParseData::default());
        // assert_eq!(result0, ParseResult::new(U8Set::from_chars("ab"), None));
        // assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("b"), None));
        // assert_eq!(parser.step('b' as u8), ParseResult::new(U8Set::none(), Some(ParseData::default())));
        let combinator = seq!(opt(eat_char_choice("a")), eat_char_choice("b"));
        let (mut parser, horizontal_data0, vertical_data0) = combinator.parser(HorizontalData::default());
        assert_eq!((horizontal_data0, VerticalData::squash(vertical_data0)), (vec![], vec![VerticalData { u8set: U8Set::from_chars("ab") }]));
        let (horizontal_data1, vertical_data1) = parser.step('a' as u8);
        assert_eq!((horizontal_data1, VerticalData::squash(vertical_data1)), (vec![], vec![VerticalData { u8set: U8Set::from_chars("b") }]));
        let (horizontal_data2, vertical_data2) = parser.step('b' as u8);
        assert_eq!((horizontal_data2, vertical_data2), (vec![HorizontalData::default(), HorizontalData::default()], vec![]));
    }

    #[test]
    fn test_forward_ref() {
        // let mut A = forward_ref();
        // A.set(choice!(seq!(eat_chars("a"), A.clone()), eat_chars("b")));
        // let combinator = A.clone();
        // let (mut parser, result0) = combinator.parser(ParseData::default());
        // assert_eq!(result0, ParseResult::new(U8Set::from_chars("ab"), None));
        // assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("ab"), None));
        // assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("ab"), None));
        // assert_eq!(parser.step('b' as u8), ParseResult::new(U8Set::none(), Some(ParseData::default())));
        let mut combinator = forward_ref();
        combinator.set(choice!(seq!(eat_char_choice("a"), combinator.clone()), eat_char_choice("b")));
        let (mut parser, horizontal_data0, vertical_data0) = combinator.parser(HorizontalData::default());
        assert_eq!((horizontal_data0, VerticalData::squash(vertical_data0)), (vec![], vec![VerticalData { u8set: U8Set::from_chars("ab") }]));
        let (horizontal_data1, vertical_data1) = parser.step('a' as u8);
        assert_eq!((horizontal_data1, VerticalData::squash(vertical_data1)), (vec![], vec![VerticalData { u8set: U8Set::from_chars("ab") }]));
        let (horizontal_data2, vertical_data2) = parser.step('a' as u8);
        assert_eq!((horizontal_data2, VerticalData::squash(vertical_data2)), (vec![], vec![VerticalData { u8set: U8Set::from_chars("ab") }]));
        let (horizontal_data3, vertical_data3) = parser.step('b' as u8);
        assert_eq!((HorizontalData::squash(horizontal_data3), VerticalData::squash(vertical_data3)), (vec![HorizontalData::default()], vec![]));
    }

    #[test]
    fn test_frame_stack_contains() {
        //     let mut frame_stack = FrameStack::default();
        //     frame_stack.push_name(b"a");
        //     let parse_data = ParseData::new(frame_stack.clone());
        //     let combinator = frame_stack_contains(eat_chars("a"));
        //     let (mut parser, result0) = combinator.parser(parse_data.clone());
        //     assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        //     assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::none(), Some(parse_data.clone())));
        //
        //     let combinator = frame_stack_contains(eat_chars("b"));
        //     let (mut parser, result0) = combinator.parser(parse_data);
        //     assert_eq!(result0, ParseResult::new(U8Set::none(), None));

        // let mut frame_stack = FrameStack::default();
        // frame_stack.push_name(b"a");
        // let parse_data = ParseData::new(frame_stack.clone());
        // let combinator = frame_stack_contains(eat_char_choice("a"));
        // let (mut parser, horizontal_data0, vertical_data0) = combinator.parser(parse_data.clone());
        // assert_eq!((horizontal_data0, vertical_data0), (vec![], vec![VerticalData { u8set: U8Set::from_chars("a") }]));
        // assert_eq!(parser.step('a' as u8), (vec![HorizontalData::default()], vec![]));
        //
        // let combinator = frame_stack_contains(eat_char_choice("b"));
        // let (mut parser, horizontal_data0, vertical_data0) = combinator.parser(parse_data);
        // assert_eq!((horizontal_data0, vertical_data0), (vec![], vec![]));
    }

    #[test]
    fn test_frame_stack_push() {
        //     let mut frame_stack = FrameStack::default();
        //     let parse_data = ParseData::new(frame_stack.clone());
        //     let combinator = seq!(push_to_frame(eat_chars("a")), frame_stack_contains(choice!(eat_chars("b"), eat_chars("a"))));
        //     let (mut parser, result0) = combinator.parser(parse_data.clone());
        //     assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        //     assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("a"), None));
        //     assert_eq!(parser.step('b' as u8), ParseResult::default());

        // let mut frame_stack = FrameStack::default();
        // let parse_data = ParseData::new(frame_stack.clone());
        // let combinator = seq!(push_to_frame(eat_char_choice("a")), frame_stack_contains(choice!(eat_char_choice("b"), eat_char_choice("a"))));
        // let (mut parser, horizontal_data0, vertical_data0) = combinator.parser(parse_data.clone());
        // assert_eq!((horizontal_data0, vertical_data0), (vec![], vec![VerticalData { u8set: U8Set::from_chars("a") }]));
        // assert_eq!(parser.step('a' as u8), (vec![], vec![VerticalData { u8set: U8Set::from_chars("a") }]));
        // assert_eq!(parser.step('b' as u8), (vec![HorizontalData::default()], vec![]));
    }

    #[test]
    fn test_frame_stack_pop() {
        //     let mut frame_stack = FrameStack::default();
        //     let parse_data = ParseData::new(frame_stack.clone());
        //     let combinator = seq!(
        //         push_to_frame(eat_chars("a")),
        //         frame_stack_contains(choice!(eat_chars("b"), eat_chars("a"))),
        //         pop_from_frame(eat_chars("a")),
        //         frame_stack_contains(eat_chars("a"))
        //     );
        //     let (mut parser, result0) = combinator.parser(parse_data.clone());
        //     assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        //     // Parsing goes like this:
        //     //
        //     // 1. "a" is pushed to the frame stack.
        //     // 2. the choice says the next character is "b" or "a", but the frame stack only contains "a", so it only allows "a".
        //     // 3. the pop_from_frame parser pops the "a" from the frame stack.
        //     // 4. eat_chars("a") says the next character is "a", but the frame stack is empty, so it doesn't allow anything, and parsing fails.
        //     //
        //     // i.e. "aaaa" should fail on the final "a".
        //     //
        //     // 1. "a" is pushed to the frame stack.
        //     assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("a"), None));
        //     // 2. the choice says the next character is "b" or "a", but the frame stack only contains "a", so it only allows "a".
        //     assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("a"), None));
        //     // 3. the pop_from_frame parser pops the "a" from the frame stack.
        //     assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::none(), None));
        //     // 4. eat_chars("a") says the next character is "a", but the frame stack is empty, so it doesn't allow anything, and parsing fails.

        // let mut frame_stack = FrameStack::default();
        // let parse_data = ParseData::new(frame_stack.clone());
        // let combinator = seq!(
        //     push_to_frame(eat_char_choice("a")),
        //     frame_stack_contains(choice!(eat_char_choice("b"), eat_char_choice("a"))),
        //     pop_from_frame(eat_char_choice("a")),
        //     frame_stack_contains(eat_char_choice("a"))
        // );
        // let (mut parser, horizontal_data0, vertical_data0) = combinator.parser(parse_data.clone());
        // assert_eq!((horizontal_data0, vertical_data0), (vec![], vec![VerticalData { u8set: U8Set::from_chars("a") }]));
        // assert_eq!(parser.step('a' as u8), (vec![], vec![VerticalData { u8set: U8Set::from_chars("a") }]));
        // assert_eq!(parser.step('a' as u8), (vec![], vec![VerticalData { u8set: U8Set::none() }]));
        // assert_eq!(parser.step('a' as u8), (vec![], vec![]));
    }

    #[test]
    fn test_frame_stack_push_empty_frame() {
        //     let mut frame_stack = FrameStack::default();
        //     let parse_data = ParseData::new(frame_stack.clone());
        //     // Simulate declaring a new scope with curly braces, defining a variable, and then using it.
        //     let combinator = seq!(
        //         eat_chars("{"),
        //         with_new_frame(seq!(
        //                 push_to_frame(eat_chars("a")), eat_chars("="), eat_chars("b"), eat_chars(";"),
        //                 frame_stack_contains(eat_chars("a")),
        //         )),
        //         eat_chars("}"),
        //         frame_stack_contains(eat_chars("a")),
        //     );
        //     let (mut parser, result0) = combinator.parser(parse_data.clone());
        //     assert_eq!(result0, ParseResult::new(U8Set::from_chars("{"), None));
        //     assert_eq!(parser.step('{' as u8), ParseResult::new(U8Set::from_chars("a"), None));
        //     assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("="), None));
        //     assert_eq!(parser.step('=' as u8), ParseResult::new(U8Set::from_chars("b"), None));
        //     assert_eq!(parser.step('b' as u8), ParseResult::new(U8Set::from_chars(";"), None));
        //     assert_eq!(parser.step(';' as u8), ParseResult::new(U8Set::from_chars("a"), None));
        //     assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("}"), None));
        //     assert_eq!(parser.step('}' as u8), ParseResult::new(U8Set::none(), None));
        //     assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::none(), None));

        // let mut frame_stack = FrameStack::default();
        // let parse_data = ParseData::new(frame_stack.clone());
        // let combinator = seq!(
        //     eat_char_choice("{"),
        //     with_new_frame(seq!(
        //         push_to_frame(eat_char_choice("a")), eat_char_choice("="), eat_char_choice("b"), eat_char_choice(";"),
        //         frame_stack_contains(eat_char_choice("a")),
        //     )),
        //     eat_char_choice("}"),
        //     frame_stack_contains(eat_char_choice("a")),
        // );
        // let (mut parser, horizontal_data0, vertical_data0) = combinator.parser(parse_data.clone());
        // assert_eq!((horizontal_data0, vertical_data0), (vec![], vec![VerticalData { u8set: U8Set::from_chars("{") }]));
        // assert_eq!(parser.step('{' as u8), (vec![], vec![VerticalData { u8set: U8Set::from_chars("a") }]));
        // assert_eq!(parser.step('a' as u8), (vec![], vec![VerticalData { u8set: U8Set::from_chars("=") }]));
        // assert_eq!(parser.step('=' as u8), (vec![], vec![VerticalData { u8set: U8Set::from_chars("b") }]));
        // assert_eq!(parser.step('b' as u8), (vec![], vec![VerticalData { u8set: U8Set::from_chars(";") }]));
        // assert_eq!(parser.step(';' as u8), (vec![], vec![VerticalData { u8set: U8Set::from_chars("a") }]));
        // assert_eq!(parser.step('a' as u8), (vec![], vec![VerticalData { u8set: U8Set::from_chars("}") }]));
        // assert_eq!(parser.step('}' as u8), (vec![], vec![]));
        // assert_eq!(parser.step('a' as u8), (vec![], vec![]));
    }

    #[test]
    fn test_indents() {
        //     let mut frame_stack = FrameStack::default();
        //     let parse_data = ParseData::new(frame_stack.clone());
        //     let combinator = seq!(
        //         eat_chars("a"),
        //         newline(),
        //         with_indent(seq!(
        //             eat_chars("b"),
        //             newline(),
        //         )),
        //         eat_chars("c"),
        //     );
        //     // e.g. "a\n b\nc" should parse, but not "a\nb\nc" or "a\n b\n c".
        //     let (mut parser, result0) = combinator.parser(parse_data);
        //     assert_eq!(result0, ParseResult::new(U8Set::from_chars("a"), None));
        //     assert_eq!(parser.step('a' as u8), ParseResult::new(U8Set::from_chars("\n"), None));
        //     assert_eq!(parser.step('\n' as u8), ParseResult::new(U8Set::from_chars(" "), None));
        //     assert_eq!(parser.step(' ' as u8), ParseResult::new(U8Set::from_chars("b"), None));
        //     assert_eq!(parser.step('b' as u8), ParseResult::new(U8Set::from_chars("\n"), None));
        //     assert_eq!(parser.step('\n' as u8), ParseResult::new(U8Set::from_chars("c"), None));
        //     assert_eq!(parser.step('c' as u8), ParseResult::new(U8Set::none(), Some(ParseData::default())));
        let mut frame_stack = FrameStack::default();
        let parse_data = HorizontalData::default();
        let combinator = seq!(
            eat_char_choice("a"),
            newline(),
            with_indent(seq!(
                eat_char_choice("b"),
                newline(),
            )),
            eat_char_choice("c"),
        );
        let (mut parser, horizontal_data0, vertical_data0) = combinator.parser(parse_data);
        assert_eq!((horizontal_data0, vertical_data0), (vec![], vec![VerticalData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8), (vec![], vec![VerticalData { u8set: U8Set::from_chars("\n") }]));
        assert_eq!(parser.step('\n' as u8), (vec![], vec![VerticalData { u8set: U8Set::from_chars(" ") }]));
        assert_eq!(parser.step(' ' as u8), (vec![], vec![VerticalData { u8set: U8Set::from_chars("b") }]));
        assert_eq!(parser.step('b' as u8), (vec![], vec![VerticalData { u8set: U8Set::from_chars("\n") }]));
        assert_eq!(parser.step('\n' as u8), (vec![], vec![VerticalData { u8set: U8Set::from_chars("c") }]));
        assert_eq!(parser.step('c' as u8), (vec![HorizontalData::default()], vec![]));
    }
}
