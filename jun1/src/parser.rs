use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use crate::u8set::U8Set;

#[derive(PartialEq, Debug)]
struct ParserIterationResult {
    u8set: U8Set,
    is_complete: bool,
}

impl ParserIterationResult {
    fn new(u8set: U8Set, is_complete: bool) -> Self {
        Self {
            u8set,
            is_complete,
        }
    }
}

impl BitOr for ParserIterationResult {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self {
            u8set: self.u8set | other.u8set,
            is_complete: self.is_complete | other.is_complete,
        }
    }
}

impl BitOrAssign for ParserIterationResult {
    fn bitor_assign(&mut self, other: Self) {
        self.u8set |= other.u8set;
        self.is_complete |= other.is_complete;
    }
}

impl BitAnd for ParserIterationResult {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Self {
            u8set: self.u8set & other.u8set,
            is_complete: self.is_complete & other.is_complete,
        }
    }
}

impl BitAndAssign for ParserIterationResult {
    fn bitand_assign(&mut self, other: Self) {
        self.u8set &= other.u8set;
        self.is_complete &= other.is_complete;
    }
}

impl Clone for ParserIterationResult {
    fn clone(&self) -> Self {
        Self {
            u8set: self.u8set.clone(),
            is_complete: self.is_complete,
        }
    }
}

type Data = ();

trait Combinator: Clone {
    type State: Clone;
    fn initial_state(&self, data: &Data) -> Self::State;
    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult;
}

#[derive(Clone)]
struct ActiveCombinator<C: Combinator> {
    combinator: C,
    data: Data,
    state: C::State,
}

impl<C: Combinator> ActiveCombinator<C> {
    fn new(combinator: C, data: Data) -> Self {
        let state = combinator.initial_state(&data);
        Self {
            combinator,
            data,
            state,
        }
    }

    fn send(&mut self, c: Option<char>) -> ParserIterationResult {
        self.combinator.next_state(&mut self.state, c)
    }
}

fn process<C: Combinator>(
    c: Option<char>,
    its: &mut Vec<ActiveCombinator<C>>,
) -> ParserIterationResult {
    let mut final_result = ParserIterationResult::new(U8Set::none(), false);
    let mut i = its.len();
    while i > 0 {
        i -= 1;
        let result = its[i].send(c);
        if result.is_complete && result.u8set.is_empty() {
            its.remove(i);
        }
        final_result |= result;
    }
    final_result
}

fn seq2_helper<B: Combinator>(
    b: B,
    d: &Data,
    a_result: &mut ParserIterationResult,
    b_its: &mut Vec<ActiveCombinator<B>>,
) {
    if a_result.is_complete {
        let b_it = ActiveCombinator::new(b, d.clone());
        b_its.push(b_it);
        let b_result = b_its.last_mut().unwrap().send(None);
        a_result.is_complete = b_result.is_complete;
        a_result.u8set |= b_result.u8set;
    }
}

#[derive(Clone)]
struct Seq2<A, B>
where
    A: Combinator,
    B: Combinator,
{
    a: A,
    b: B,
}

impl<A, B> Seq2<A, B>
where
    A: Combinator,
    B: Combinator,
{
    fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A, B> Combinator for Seq2<A, B>
where
    A: Combinator,
    B: Combinator,
{
    type State = (Vec<ActiveCombinator<A>>, Vec<ActiveCombinator<B>>, Data);

    fn initial_state(&self, data: &Data) -> Self::State {
        (
            vec![ActiveCombinator::new(self.a.clone(), data.clone())],
            Vec::new(),
            data.clone(),
        )
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult {
        let (a_its, b_its, d) = state;
        let mut a_result = process(c, a_its);
        let b_result = process(c, b_its);
        seq2_helper(self.b.clone(), d, &mut a_result, b_its);
        a_result | b_result
    }
}

fn seq2<A, B>(a: A, b: B) -> Seq2<A, B>
where
    A: Combinator,
    B: Combinator,
{
    Seq2::new(a, b)
}

// Box is used here to allow for recursive types
fn seq<A: Combinator, B: Combinator>(
    a: A,
    b: B
) -> Seq2<A, B>
where
    A: Combinator + Clone,
    A::State: Clone,
    B: Combinator + Clone,
    B::State: Clone,
{
    Seq2::new(a, b)
}


#[derive(Clone)]
struct Repeat1<A>(A)
where
    A: Combinator;

impl<A> Combinator for Repeat1<A>
where
    A: Combinator,
{
    type State = (Vec<ActiveCombinator<A>>, Data);

    fn initial_state(&self, data: &Data) -> Self::State {
        (vec![ActiveCombinator::new(self.0.clone(), data.clone())], data.clone())
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult {
        let (a_its, d) = state;
        let mut a_result = process(c, a_its);
        let b_result = a_result.clone();
        seq2_helper(self.0.clone(), d, &mut a_result, a_its);
        a_result | b_result
    }
}

fn repeat1<A>(a: A) -> Repeat1<A>
where
    A: Combinator,
{
    Repeat1(a)
}

#[derive(Clone)]
struct Choice2<A, B>
where
    A: Combinator,
    B: Combinator,
{
    a: A,
    b: B
}

impl<A, B> Choice2<A, B>
where
    A: Combinator,
    B: Combinator,
{
    fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A, B> Combinator for Choice2<A, B>
where
    A: Combinator,
    B: Combinator,
{
    type State = (A::State, B::State);

    fn initial_state(&self, data: &Data) -> Self::State {
        (self.a.initial_state(&data), self.b.initial_state(&data))
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult {
        let mut result_a = self.a.next_state(&mut state.0, c);
        let result_b = self.b.next_state(&mut state.1, c);
        result_a |= result_b;
        result_a
    }
}

fn choice2<A: Combinator, B: Combinator>(a: A, b: B) -> Choice2<A, B> {
    Choice2::new(a, b)
}

#[derive(Clone)]
struct EatU8Matching
{
    u8set: U8Set,
}

impl EatU8Matching
{
    fn new<F>(fn_: F) -> Self
    where
        F: Fn(u8) -> bool,
    {
        Self {
            u8set: U8Set::from_match_fn(&fn_),
        }
    }
}

impl Combinator for EatU8Matching
{
    type State = u8;

    fn initial_state(&self, _data: &Data) -> Self::State {
        0
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult {
        match *state {
            0 => {
                *state = 1;
                ParserIterationResult::new(self.u8set.clone(), false)
            }
            1 => {
                *state = 2;
                ParserIterationResult::new(
                    U8Set::none(), c.map(|c| self.u8set.contains(c as u8)).unwrap_or(false),
                )
            }
            _ => ParserIterationResult::new(U8Set::none(), true),
        }
    }
}

fn eat_u8_matching<F>(fn_: F) -> EatU8Matching
where
    F: Fn(u8) -> bool,
{
    EatU8Matching::new(fn_)
}

fn eat_u8(value: char) -> EatU8Matching {
    eat_u8_matching(move |c: u8| c == value as u8)
}

fn eat_u8_range(start: char, end: char) -> EatU8Matching {
    eat_u8_matching(move |c: u8| (start as u8..=end as u8).contains(&c))
}

fn eat_u8_range_complement(start: char, end: char) -> EatU8Matching {
    eat_u8_matching(move |c: u8| !(start as u8..=end as u8).contains(&c))
}

#[derive(Clone)]
struct EatString {
    value: String,
}

impl EatString {
    fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl Combinator for EatString {
    type State = usize;

    fn initial_state(&self, _data: &Data) -> Self::State {
        0
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult {
        if *state < self.value.len() {
            let expected = self.value.chars().nth(*state).unwrap();
            if c.map_or(false, |c| c == expected) {
                *state += 1;
                let is_complete = *state == self.value.len();
                ParserIterationResult::new(U8Set::none(), is_complete)
            } else {
                ParserIterationResult::new(U8Set::none(), true)
            }
        } else {
            ParserIterationResult::new(U8Set::none(), true)
        }
    }
}

fn eat_string(value: &str) -> EatString {
    EatString::new(value)
}

#[derive(Clone)]
struct Eps;

impl Combinator for Eps {
    type State = ();
    fn initial_state(&self, _data: &Data) -> Self::State {}
    fn next_state(&self, _state: &mut Self::State, _c: Option<char>) -> ParserIterationResult {
        ParserIterationResult::new(U8Set::none(), true)
    }
}

fn eps() -> Eps {
    Eps
}

fn opt<A: Combinator>(a: A) -> Choice2<A, Eps> {
    choice2(a, eps())
}

fn repeat<A: Combinator>(a: A) -> Choice2<Repeat1<A>, Eps> {
    opt(repeat1(a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eat_u8() {
        let mut it = ActiveCombinator::new(eat_u8('a'), ());
        let result0 = it.send(None);
        assert_eq!(result0, ParserIterationResult::new(U8Set::from_chars("a"), false));
        let result = it.send(Some('a'));
        assert_eq!(result, ParserIterationResult::new(U8Set::none(), true));
    }

    #[test]
    fn test_seq() {
        let mut it = ActiveCombinator::new(seq(eat_u8('a'), eat_u8('b')), ());
        let result0 = it.send(None);
        assert_eq!(result0, ParserIterationResult::new(U8Set::from_chars("a"), false));
        let result1 = it.send(Some('a'));
        assert_eq!(result1, ParserIterationResult::new(U8Set::from_chars("b"), false));
        let result2 = it.send(Some('b'));
        assert_eq!(result2, ParserIterationResult::new(U8Set::none(), true));
    }

    #[test]
    fn test_repeat1() {
        let mut it = ActiveCombinator::new(repeat1(eat_u8('a')), ());
        let result0 = it.send(None);
        assert_eq!(result0, ParserIterationResult::new(U8Set::from_chars("a"), false));
        let result1 = it.send(Some('a'));
        assert_eq!(result1, ParserIterationResult::new(U8Set::from_chars("a"), true));
        let result2 = it.send(Some('a'));
        assert_eq!(result2, ParserIterationResult::new(U8Set::from_chars("a"), true));
    }

    #[test]
    fn test_choice() {
        let mut it = ActiveCombinator::new(choice2(eat_u8('a'), eat_u8('b')), ());
        let result0 = it.send(None);
        assert_eq!(result0, ParserIterationResult::new(U8Set::from_chars("ab"), false));
        let result1 = it.send(Some('a'));
        assert_eq!(result1, ParserIterationResult::new(U8Set::none(), true));

        let mut it = ActiveCombinator::new(choice2(eat_u8('a'), eat_u8('b')), ());
        it.send(None);
        let result2 = it.send(Some('b'));
        assert_eq!(result2, ParserIterationResult::new(U8Set::none(), true));
    }

    #[test]
    fn test_seq_choice_seq() {
        // Matches "ac" or "abc"
        let mut it = ActiveCombinator::new(
            seq(
                choice2(eat_u8('a'), seq2(eat_u8('a'), eat_u8('b'))),
                eat_u8('c')
            ),
            (),
        );
        let result0 = it.send(None);
        assert_eq!(result0, ParserIterationResult::new(U8Set::from_chars("a"), false));
        let result1 = it.send(Some('a'));
        assert_eq!(result1, ParserIterationResult::new(U8Set::from_chars("bc"), false));
        let result2 = it.send(Some('b'));
        assert_eq!(result2, ParserIterationResult::new(U8Set::from_chars("c"), false));
        let result3 = it.send(Some('c'));
        assert_eq!(result3, ParserIterationResult::new(U8Set::none(), true));
    }

    #[test]
    fn test_json_parser() {
        // Helper combinators for JSON parsing
        let whitespace = repeat(choice2(eat_u8(' '), choice2(eat_u8('\t'), choice2(eat_u8('\n'), eat_u8('\r')))));
        let digit = eat_u8_range('0', '9');
        let digits = repeat(digit);
        let integer = seq(opt(choice2(eat_u8('-'), eat_u8('+'))), digits.clone());
        let fraction = seq(eat_u8('.'), digits.clone());
        let exponent = seq(choice2(eat_u8('e'), eat_u8('E')), seq(choice2(choice2(eat_u8('+'), eat_u8('-')), eps()), digits));
        let number = seq(integer, seq(opt(fraction), opt(exponent)));

        let string_char = choice2(
            eat_u8_range_complement('"', '"'),
            seq(
                eat_u8('\\'),
                choice2(
                    choice2(
                        choice2(
                            choice2(eat_u8('"'), eat_u8('\\')),
                            choice2(eat_u8('/'), eat_u8('b')),
                        ),
                        choice2(eat_u8('f'), eat_u8('n')),
                    ),
                    choice2(
                        choice2(eat_u8('r'), eat_u8('t')),
                        seq(
                            eat_u8('u'),
                            seq(
                                eat_u8_range('0', '9'),
                                seq(
                                    eat_u8_range('0', '9'),
                                    seq(eat_u8_range('0', '9'), eat_u8_range('0', '9')),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );
        let string = seq(eat_u8('"'), seq(repeat(string_char), eat_u8('"')));

        let json_array = seq(
            eat_u8('['),
            seq(
                whitespace.clone(),
                seq(
                    opt(seq(
                        json_value.clone(),
                        repeat(seq(seq(whitespace.clone(), eat_u8(',')), seq(whitespace.clone(), json_value.clone()))),
                    )),
                    seq(whitespace.clone(), eat_u8(']')),
                ),
            ),
        );

        let key_value_pair = seq(seq(whitespace.clone(), string.clone()), seq(whitespace.clone(), seq(eat_u8(':'), seq(whitespace.clone(), json_value.clone()))));

        let json_object = seq(
            eat_u8('{'),
            seq(
                whitespace.clone(),
                seq(
                    opt(seq(
                        key_value_pair.clone(),
                        repeat(seq(seq(whitespace.clone(), eat_u8(',')), key_value_pair)),
                    )),
                    seq(whitespace.clone(), eat_u8('}')),
                ),
            ),
        );

        let json_value = choice2(
            choice2(string, number),
            choice2(
                choice2(eat_string("true"), eat_string("false")),
                choice2(eat_string("null"), choice2(json_array, json_object)),
            ));

        // Test cases
        let json_parser = seq(whitespace.clone(), json_value.clone());

        let test_cases = [
            "42",
            r#"{"key": "value"}"#,
            "[1, 2, 3]",
            r#"{"nested": {"array": [1, 2, 3], "object": {"a": true, "b": false}}}"#,
            r#""Hello, world!""#,
            "null",
            "true",
            "false",
        ];

        for json_string in test_cases {
            let mut it = ActiveCombinator::new(json_parser.clone(), ());
            let result = it.send(None);
            for char in json_string.chars() {
                assert!(result.u8set.contains(char as u8), "Expected {} to be in {:?}", char, result.u8set);
                let result = it.send(Some(char));
                assert!(result.is_complete, "Failed to parse JSON string: {}", json_string);
            }
        }
    }
}