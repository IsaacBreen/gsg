#![feature(assert_matches)]


#[cfg(test)]
mod tests {
    use crate::*;
    use crate::combinators::*;
    use crate::parse_state::{RightData, UpData};

    #[test]
    fn test_eat_u8() {
        let combinator = eat_char_choice("a");
        let (mut parser, right_data0, up_data0) = combinator.parser(RightData::default());
        assert_eq!((right_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8), (vec![RightData::default()], vec![]));
    }

    #[test]
    fn test_eat_string() {
        let combinator = eat_string("abc");
        let (mut parser, right_data0, up_data0) = combinator.parser(RightData::default());
        assert_eq!((right_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("b") }]));
        assert_eq!(parser.step('b' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("c") }]));
        assert_eq!(parser.step('c' as u8).squashed(), (vec![RightData::default()], vec![]));
    }

    #[test]
    fn test_seq() {
        let combinator = seq!(eat_char_choice("a"), eat_char_choice("b"));
        let (mut parser, right_data0, up_data0) = combinator.parser(RightData::default());
        assert_eq!((right_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8), (vec![], vec![UpData { u8set: U8Set::from_chars("b") }]));
        assert_eq!(parser.step('b' as u8), (vec![RightData::default()], vec![]));
    }

    #[test]
    fn test_repeat1() {
        let combinator = repeat1(eat_char_choice("a"));
        let (mut parser, right_data0, up_data0) = combinator.parser(RightData::default());
        assert_eq!(Squash::squashed((right_data0, up_data0)), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![RightData::default()], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![RightData::default()], vec![UpData { u8set: U8Set::from_chars("a") }]));
    }

    #[test]
    fn test_choice() {
        let combinator = choice!(eat_char_choice("a"), eat_char_choice("b"));
        let (mut parser, right_data0, up_data0) = combinator.parser(RightData::default());
        assert_eq!((right_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }, UpData { u8set: U8Set::from_chars("b") }]));
        assert_eq!(parser.step('b' as u8), (vec![RightData::default()], vec![]));
    }

    #[test]
    fn test_seq_choice_seq() {
        let combinator = seq!(choice!(eat_char_choice("a"), seq!(eat_char_choice("a"), eat_char_choice("b"))), eat_char_choice("c"));
        let (mut parser, right_data0, up_data0) = combinator.parser(RightData::default());
        assert_eq!(Squash::squashed((right_data0, up_data0)), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("bc") }]));
        assert_eq!(parser.step('b' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("c") }]));
        assert_eq!(parser.step('c' as u8), (vec![RightData::default(), RightData::default()], vec![]));
    }

    #[test]
    fn test_seq_opt() {
        let combinator = seq!(opt(eat_char_choice("a")), eat_char_choice("b"));
        let (mut parser, right_data0, up_data0) = combinator.parser(RightData::default());
        assert_eq!(Squash::squashed((right_data0, up_data0)), (vec![], vec![UpData { u8set: U8Set::from_chars("ab") }]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("b") }]));
        assert_eq!(parser.step('b' as u8), (vec![RightData::default(), RightData::default()], vec![]));
    }

    #[test]
    fn test_forward_ref() {
        let mut combinator = forward_ref();
        combinator.set(choice!(seq!(eat_char_choice("a"), &combinator), eat_char_choice("b")));
        let (mut parser, right_data0, up_data0) = combinator.parser(RightData::default());
        assert_eq!(Squash::squashed((right_data0, up_data0)), (vec![], vec![UpData { u8set: U8Set::from_chars("ab") }]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("ab") }]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("ab") }]));
        assert_eq!(parser.step('b' as u8).squashed(), (vec![RightData::default()], vec![]));
    }

    #[test]
    fn test_left_recursion_guard_explicit() {
        let eat_char_choice2 = |s| left_recursion_guard_terminal(eat_char_choice(s));
        let mut A = forward_ref();
        A.set(left_recursion_guard(choice!(seq!(&A, eat_char_choice2("a")), eat_char_choice2("b"))));
        let (mut parser, right_data0, up_data0) = A.parser(RightData::default());
        assert_eq!(Squash::squashed((right_data0, up_data0)), (vec![], vec![UpData { u8set: U8Set::from_chars("b") }]));
        assert_eq!(parser.step('b' as u8).squashed(), (vec![RightData::default()], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![RightData::default()], vec![UpData { u8set: U8Set::from_chars("a") }]));
        let (right_data3, up_data3) = parser.step('a' as u8);
    }

    #[test]
    fn test_left_recursion_guard_implicit() {
        let eat_char_choice2 = |s| left_recursion_guard_terminal(eat_char_choice(s));
        let mut A = forward_ref();
        A.set(choice!(seq!(&A, eat_char_choice2("a")), eat_char_choice2("b")));
        let (mut parser, right_data0, up_data0) = A.parser(RightData::default());
        assert_eq!(Squash::squashed((right_data0, up_data0)), (vec![], vec![UpData { u8set: U8Set::from_chars("b") }]));
        assert_eq!(parser.step('b' as u8).squashed(), (vec![RightData::default()], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![RightData::default()], vec![UpData { u8set: U8Set::from_chars("a") }]));
        let (right_data3, up_data3) = parser.step('a' as u8);
    }

    #[test]
    fn test_left_recursion_guard_empty() {
        let mut A = forward_ref();
        A.set(left_recursion_guard(choice!(&A, eps())));
        let (mut parser, right_data0, up_data0) = A.parser(RightData::default());
        assert_eq!(Squash::squashed((right_data0, up_data0)), (vec![RightData::default()], vec![]));
    }

    #[test]
    fn test_left_recursion_backtrack() {
        let eat_char_choice2 = |s| left_recursion_guard_terminal(eat_char_choice(s));
        let mut A = forward_ref();
        A.set(choice!(seq!(choice!(&A, seq!(&A, eat_char_choice2("c"))), eat_char_choice2("a")), eat_char_choice2("b")));
        let (mut parser, right_data0, up_data0) = A.parser(RightData::default());
        assert_eq!(Squash::squashed((right_data0, up_data0)), (vec![], vec![UpData { u8set: U8Set::from_chars("b") }]));
        assert_eq!(parser.step('b' as u8).squashed(), (vec![RightData::default()], vec![UpData { u8set: U8Set::from_chars("ca") }]));
        assert_eq!(parser.step('c' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![RightData::default()], vec![UpData { u8set: U8Set::from_chars("ac") }]));
    }

    #[test]
    fn test_left_recursion_guard_double_consecutive() {
        let eat_char_choice2 = |s| left_recursion_guard_terminal(eat_char_choice(s));
        let mut A = forward_ref();
        A.set(choice!(seq!(&A, &A, eat_char_choice2("a")), eat_char_choice2("b")));
        let (mut parser, right_data0, up_data0) = A.parser(RightData::default());
        assert_eq!(Squash::squashed((right_data0, up_data0)), (vec![], vec![UpData { u8set: U8Set::from_chars("b") }]));
        assert_eq!(parser.step('b' as u8).squashed(), (vec![RightData::default()], vec![UpData { u8set: U8Set::from_chars("b") }]));
        assert_eq!(parser.step('b' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("ba") }]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![RightData::default()], vec![UpData { u8set: U8Set::from_chars("b") }]));
        assert_eq!(parser.step('b' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("ba") }]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![RightData::default()], vec![UpData { u8set: U8Set::from_chars("ba") }]));
    }

    #[test]
    fn test_frame_stack_contains() {
        let mut frame_stack = FrameStack::default();
        frame_stack.push_name(b"a");
        let mut right_data = RightData::default();
        right_data.frame_stack.as_mut().unwrap().push_name(b"a");
        let combinator = frame_stack_contains(eat_char_choice("a"));
        let (mut parser, right_data0, up_data0) = combinator.parser(right_data.clone());
        assert_eq!((right_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8), (vec![right_data.clone()], vec![]));

        let combinator = frame_stack_contains(eat_char_choice("b"));
        let (mut parser, right_data0, up_data0) = combinator.parser(right_data);
        assert_eq!((right_data0, up_data0).squashed(), (vec![], vec![]));
    }

    #[test]
    fn test_frame_stack_push() {
        let mut frame_stack = FrameStack::default();
        let right_data = RightData::default();
        let combinator = seq!(push_to_frame(eat_char_choice("a")), frame_stack_contains(choice!(eat_char_choice("b"), eat_char_choice("a"))));
        let (mut parser, right_data0, up_data0) = combinator.parser(right_data.clone());
        assert_eq!((right_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('b' as u8).squashed(), (vec![], vec![]));
    }

    #[test]
    fn test_frame_stack_pop() {
        let mut frame_stack = FrameStack::default();
        let right_data = RightData::default();
        let combinator = seq!(
            push_to_frame(eat_char_choice("a")),
            frame_stack_contains(choice!(eat_char_choice("b"), eat_char_choice("a"))),
            pop_from_frame(eat_char_choice("a")),
            frame_stack_contains(eat_char_choice("a"))
        );
        let (mut parser, right_data0, up_data0) = combinator.parser(right_data);
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
        assert_eq!((right_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
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
        let right_data = RightData::default();
        let combinator = seq!(
            eat_char_choice("{"),
            with_new_frame(seq!(
                push_to_frame(eat_char_choice("a")), eat_char_choice("="), eat_char_choice("b"), eat_char_choice(";"),
                frame_stack_contains(eat_char_choice("a")),
            )),
            eat_char_choice("}"),
            frame_stack_contains(eat_char_choice("a")),
        );
        let (mut parser, right_data0, up_data0) = combinator.parser(right_data);
        assert_eq!((right_data0, up_data0), (vec![], vec![UpData { u8set: U8Set::from_chars("{") }]));
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
        let parse_data = RightData::default();
        let combinator = seq!(
            eat_char_choice("a"),
            python_newline(),
            with_indent(seq!(
                eat_char_choice("b"),
                python_newline(),
            )),
            eat_char_choice("c"),
        );
        let (mut parser, right_data0, up_data0) = combinator.parser(parse_data);
        assert_eq!((right_data0, up_data0).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("a") }]));
        assert_eq!(parser.step('a' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("\n ") }]));
        assert_eq!(parser.step('\n' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("\n ") }]));
        assert_eq!(parser.step(' ' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("\n b") }]));
        assert_eq!(parser.step('b' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("\n ") }]));
        assert_eq!(parser.step('\n' as u8).squashed(), (vec![], vec![UpData { u8set: U8Set::from_chars("\n c") }]));
        assert_eq!(parser.step('c' as u8).squashed(), (vec![RightData::default()], vec![]));
    }
}