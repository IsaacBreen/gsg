#![feature(assert_matches)]


#[cfg(test)]
mod tests {
    use crate::*;
    use crate::combinators::*;
    use crate::parse_state::{HorizontalData, UpData};

    #[test]
    fn test_eat_u8() {
        let combinator = eat_char_choice("a");
        let (mut parser, horizontal_data0, up_data0) = combinator.parser(HorizontalData::default());
        assert_eq!((horizontal_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8), (vec![HorizontalData::default()], vec![]));
    }

    #[test]
    fn test_eat_string() {
        let combinator = eat_string("abc");
        let (mut parser, horizontal_data0, up_data0) = combinator.parser(HorizontalData::default());
        assert_eq!((horizontal_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        let (horizontal_data1, up_data1) = parser.step('a' as u8);
        assert_eq!((horizontal_data1, up_data1), (vec![], vec![UpData { u8set: U8Set::from_chars("b") }]));
        let (horizontal_data2, up_data2) = parser.step('b' as u8);
        assert_eq!((horizontal_data2, up_data2), (vec![], vec![UpData { u8set: U8Set::from_chars("c") }]));
        let (horizontal_data3, up_data3) = parser.step('c' as u8);
        assert_eq!((horizontal_data3, up_data3), (vec![HorizontalData::default()], vec![]));
    }

    #[test]
    fn test_seq() {
        let combinator = seq!(eat_char_choice("a"), eat_char_choice("b"));
        let (mut parser, horizontal_data0, up_data0) = combinator.parser(HorizontalData::default());
        assert_eq!((horizontal_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8), (vec![], vec![UpData { u8set: U8Set::from_chars("b") }]));
        assert_eq!(parser.step('b' as u8), (vec![HorizontalData::default()], vec![]));
    }

    #[test]
    fn test_repeat1() {
        let combinator = repeat1(eat_char_choice("a"));
        let (mut parser, horizontal_data0, up_data0) = combinator.parser(HorizontalData::default());
        assert_eq!((horizontal_data0, Squash::squashed(up_data0)), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        let (horizontal_data1, up_data1) = parser.step('a' as u8);
        assert_eq!((horizontal_data1, Squash::squashed(up_data1)), (vec![HorizontalData::default()], vec![UpData { u8set: U8Set::from_chars("a") }]));
        let (horizontal_data2, up_data2) = parser.step('a' as u8);
        assert_eq!((horizontal_data2, Squash::squashed(up_data2)), (vec![HorizontalData::default()], vec![UpData { u8set: U8Set::from_chars("a") }]));
    }

    #[test]
    fn test_choice() {
        let combinator = choice!(eat_char_choice("a"), eat_char_choice("b"));
        let (mut parser, horizontal_data0, up_data0) = combinator.parser(HorizontalData::default());
        assert_eq!((horizontal_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }, UpData { u8set: U8Set::from_chars("b") }]));
        assert_eq!(parser.step('b' as u8), (vec![HorizontalData::default()], vec![]));
    }

    #[test]
    fn test_seq_choice_seq() {
        let combinator = seq!(choice!(eat_char_choice("a"), seq!(eat_char_choice("a"), eat_char_choice("b"))), eat_char_choice("c"));
        let (mut parser, horizontal_data0, up_data0) = combinator.parser(HorizontalData::default());
        assert_eq!((horizontal_data0, Squash::squashed(up_data0)), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        let (horizontal_data1, up_data1) = parser.step('a' as u8);
        assert_eq!((horizontal_data1, Squash::squashed(up_data1)), (vec![], vec![UpData { u8set: U8Set::from_chars("bc") }]));
        let (horizontal_data2, up_data2) = parser.step('b' as u8);
        assert_eq!((horizontal_data2, Squash::squashed(up_data2)), (vec![], vec![UpData { u8set: U8Set::from_chars("c") }]));
        let (horizontal_data3, up_data3) = parser.step('c' as u8);
        assert_eq!((horizontal_data3, up_data3), (vec![HorizontalData::default(), HorizontalData::default()], vec![]));
    }

    #[test]
    fn test_seq_opt() {
        let combinator = seq!(opt(eat_char_choice("a")), eat_char_choice("b"));
        let (mut parser, horizontal_data0, up_data0) = combinator.parser(HorizontalData::default());
        assert_eq!((horizontal_data0, Squash::squashed(up_data0)), (vec![], vec![UpData { u8set: U8Set::from_chars("ab") }]));
        let (horizontal_data1, up_data1) = parser.step('a' as u8);
        assert_eq!((horizontal_data1, Squash::squashed(up_data1)), (vec![], vec![UpData { u8set: U8Set::from_chars("b") }]));
        let (horizontal_data2, up_data2) = parser.step('b' as u8);
        assert_eq!((horizontal_data2, up_data2), (vec![HorizontalData::default(), HorizontalData::default()], vec![]));
    }

    #[test]
    fn test_forward_ref() {
        let mut combinator = forward_ref();
        combinator.set(choice!(seq!(eat_char_choice("a"), combinator.clone()), eat_char_choice("b")));
        let (mut parser, horizontal_data0, up_data0) = combinator.parser(HorizontalData::default());
        assert_eq!((horizontal_data0, Squash::squashed(up_data0)), (vec![], vec![UpData { u8set: U8Set::from_chars("ab") }]));
        let (horizontal_data1, up_data1) = parser.step('a' as u8);
        assert_eq!((horizontal_data1, Squash::squashed(up_data1)), (vec![], vec![UpData { u8set: U8Set::from_chars("ab") }]));
        let (horizontal_data2, up_data2) = parser.step('a' as u8);
        assert_eq!((horizontal_data2, Squash::squashed(up_data2)), (vec![], vec![UpData { u8set: U8Set::from_chars("ab") }]));
        let (horizontal_data3, up_data3) = parser.step('b' as u8);
        assert_eq!((Squash::squashed(horizontal_data3), Squash::squashed(up_data3)), (vec![HorizontalData::default()], vec![]));
    }

    #[test]
    fn test_frame_stack_contains() {
        let mut frame_stack = FrameStack::default();
        frame_stack.push_name(b"a");
        let mut horizontal_data = HorizontalData::default();
        horizontal_data.frame_stack.as_mut().unwrap().push_name(b"a");
        let combinator = frame_stack_contains(eat_char_choice("a"));
        let (mut parser, horizontal_data0, up_data0) = combinator.parser(horizontal_data.clone());
        assert_eq!((horizontal_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8), (vec![horizontal_data.clone()], vec![]));

        let combinator = frame_stack_contains(eat_char_choice("b"));
        let (mut parser, horizontal_data0, up_data0) = combinator.parser(horizontal_data);
        assert_eq!((horizontal_data0, up_data0).squashed(), (vec![], vec![]));
    }

    #[test]
    fn test_frame_stack_push() {
        let mut frame_stack = FrameStack::default();
        let horizontal_data = HorizontalData::default();
        let combinator = seq!(push_to_frame(eat_char_choice("a")), frame_stack_contains(choice!(eat_char_choice("b"), eat_char_choice("a"))));
        let (mut parser, horizontal_data0, up_data0) = combinator.parser(horizontal_data.clone());
        assert_eq!((horizontal_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('b' as u8).squashed(), (vec![], vec![]));
    }

    #[test]
    fn test_frame_stack_pop() {
        let mut frame_stack = FrameStack::default();
        let horizontal_data = HorizontalData::default();
        let combinator = seq!(
            push_to_frame(eat_char_choice("a")),
            frame_stack_contains(choice!(eat_char_choice("b"), eat_char_choice("a"))),
            pop_from_frame(eat_char_choice("a")),
            frame_stack_contains(eat_char_choice("a"))
        );
        let (mut parser, horizontal_data0, up_data0) = combinator.parser(horizontal_data);
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
        assert_eq!((horizontal_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        //     // 2. the choice says the next character is "b" or "a", but the frame stack only contains "a", so it only allows "a".
        assert_eq!(parser.step('a' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        //     // 3. the pop_from_frame parser pops the "a" from the frame stack.
        assert_eq!(parser.step('a' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        //     // 4. eat_chars("a") says the next character is "a", but the frame stack is empty, so it doesn't allow anything, and parsing fails.
        assert_eq!(parser.step('a' as u8).squashed(), (vec![], vec![]));
    }

    #[test]
    fn test_frame_stack_push_empty_frame() {
        let mut frame_stack = FrameStack::default();
        let horizontal_data = HorizontalData::default();
        let combinator = seq!(
            eat_char_choice("{"),
            with_new_frame(seq!(
                push_to_frame(eat_char_choice("a")), eat_char_choice("="), eat_char_choice("b"), eat_char_choice(";"),
                frame_stack_contains(eat_char_choice("a")),
            )),
            eat_char_choice("}"),
            frame_stack_contains(eat_char_choice("a")),
        );
        let (mut parser, horizontal_data0, up_data0) = combinator.parser(horizontal_data);
        assert_eq!((horizontal_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("{") }]));
        assert_eq!(parser.step('{' as u8), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8), (vec![], vec![UpData { u8set: U8Set::from_chars("=") }]));
        assert_eq!(parser.step('=' as u8), (vec![], vec![UpData { u8set: U8Set::from_chars("b") }]));
        assert_eq!(parser.step('b' as u8), (vec![], vec![UpData { u8set: U8Set::from_chars(";") }]));
        assert_eq!(parser.step(';' as u8), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8), (vec![], vec![UpData { u8set: U8Set::from_chars("}") }]));
        assert_eq!(parser.step('}' as u8).squashed(), (vec![], vec![]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![], vec![]));
    }

    #[test]
    fn test_indents() {
        let mut frame_stack = FrameStack::default();
        let parse_data = HorizontalData::default();
        let combinator = seq!(
            eat_char_choice("a"),
            python_newline(),
            with_indent(seq!(
                eat_char_choice("b"),
                python_newline(),
            )),
            eat_char_choice("c"),
        );
        let (mut parser, horizontal_data0, up_data0) = combinator.parser(parse_data);
        assert_eq!((horizontal_data0, up_data0).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("\n ") }]));
        assert_eq!(parser.step('\n' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("\n ") }]));
        assert_eq!(parser.step(' ' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("\n b") }]));
        assert_eq!(parser.step('b' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("\n ") }]));
        assert_eq!(parser.step('\n' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("\n c") }]));
        assert_eq!(parser.step('c' as u8).squashed(), (vec![HorizontalData::default()], vec![]));
    }
}