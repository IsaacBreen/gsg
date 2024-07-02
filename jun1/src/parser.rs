use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};
use std::rc::Rc;
use crate::u8set::U8Set;

#[derive(Clone, PartialEq, Debug)]
struct ParserIterationResult {
    u8set: U8Set,
    is_complete: bool,
}

impl ParserIterationResult {
    fn new(u8set: U8Set, is_complete: bool) -> Self {
        Self { u8set, is_complete }
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

#[derive(Clone)]
enum Combinator {
    Call(Rc<dyn Fn() -> Combinator>),
    Choice(Rc<[Combinator]>),
    EatString(&'static str),
    EatU8Matching(U8Set),
    Eps,
    ForwardRef(Rc<RefCell<Option<Combinator>>>),
    Repeat1(Box<Combinator>),
    Seq(Rc<[Combinator]>),
}

impl Debug for Combinator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Combinator::Call(_) => write!(f, "Call"),
            Combinator::Choice(a) => write!(f, "Choice({:?})", a),
            Combinator::EatString(value) => write!(f, "EatString({:?})", value),
            Combinator::EatU8Matching(u8set) => write!(f, "EatU8Matching({:?})", u8set),
            Combinator::Eps => write!(f, "Eps"),
            Combinator::ForwardRef(c) => write!(f, "ForwardRef({:?})", c),
            Combinator::Repeat1(a) => write!(f, "Repeat1({:?})", a),
            Combinator::Seq(a) => write!(f, "Seq({:?})", a),
        }
    }
}

#[derive(Debug)]
enum CombinatorState {
    Call(Option<Box<CombinatorState>>),
    Choice(Vec<Vec<CombinatorState>>),
    EatString(usize),
    EatU8Matching(u8),
    Eps,
    ForwardRef(Box<CombinatorState>),
    Repeat1(Vec<CombinatorState>),
    Seq(Vec<Vec<CombinatorState>>),
}

impl Combinator {
    fn initial_state(&self) -> CombinatorState {
        match self {
            Combinator::Call(f) => CombinatorState::Call(Some(Box::new(f().initial_state()))),
            Combinator::Choice(a) => CombinatorState::Choice(a.iter().map(|a| vec![a.initial_state()]).collect()),
            Combinator::EatString(_) => CombinatorState::EatString(0),
            Combinator::EatU8Matching(_) => CombinatorState::EatU8Matching(0),
            Combinator::Eps => CombinatorState::Eps,
            Combinator::ForwardRef(c) => {
                match c.as_ref().borrow().as_ref() {
                    Some(c) => CombinatorState::ForwardRef(Box::new(c.initial_state())),
                    None => panic!("ForwardRef not set"),
                }
            }
            Combinator::Repeat1(a) => CombinatorState::Repeat1(vec![a.initial_state()]),
            Combinator::Seq(a) => {
                let mut its = Vec::with_capacity(a.len());
                its.push(vec![a[0].initial_state()]);
                for _ in 1..a.len() {
                    its.push(Vec::new());
                }
                CombinatorState::Seq(its)
            }
        }
    }

    fn next_state(&self, state: &mut CombinatorState, c: Option<char>) -> ParserIterationResult {
        match (self, state) {
            (Combinator::Call(f), CombinatorState::Call(inner_state)) => {
                let inner_state = inner_state.as_mut().unwrap();
                f().next_state(inner_state, c)
            }
            (Combinator::Choice(combinators), CombinatorState::Choice(its)) => {
                let mut final_result = ParserIterationResult::new(U8Set::none(), false);
                for (combinator, its) in combinators.iter().zip(its.iter_mut()) {
                    let result = process(combinator, c, its);
                    final_result |= result;
                }
                final_result
            }
            (Combinator::EatString(value), CombinatorState::EatString(index)) => {
                if *index > value.len() {
                    panic!("EatString: index out of bounds");
                }
                let mut u8set = U8Set::none();
                if *index < value.len() {
                    u8set.insert(value.chars().nth(*index).unwrap() as u8);
                }
                let is_complete = *index == value.len() && c.map(|c| c == value.chars().nth(*index - 1).unwrap()).unwrap_or(false);
                *index += 1;
                ParserIterationResult::new(u8set, is_complete)
            }
            (Combinator::EatU8Matching(u8set), CombinatorState::EatU8Matching(state)) => {
                match *state {
                    0 => {
                        *state = 1;
                        ParserIterationResult::new(u8set.clone(), false)
                    }
                    1 => {
                        *state = 2;
                        ParserIterationResult::new(
                            U8Set::none(),
                            c.map(|c| u8set.contains(c as u8)).unwrap_or(false),
                        )
                    }
                    _ => panic!("EatU8Matching: state out of bounds"),
                }
            }
            (Combinator::Eps, CombinatorState::Eps) => {
                ParserIterationResult::new(U8Set::none(), true)
            }
            (Combinator::ForwardRef(inner), CombinatorState::ForwardRef(inner_state)) => {
                match inner.as_ref().borrow().as_ref() {
                    Some(combinator) => {
                        let inner_state = inner_state.as_mut();
                        combinator.next_state(inner_state, c)
                    }
                    None => {
                        panic!("Forward reference not set before use");
                    }
                }
            }
            (Combinator::Repeat1(a), CombinatorState::Repeat1(a_its)) => {
                let mut a_result = process(a.as_ref(), c, a_its);
                let b_result = a_result.clone();
                seq2_helper(a, &mut a_result, a_its);
                a_result | b_result
            }
            (Combinator::Seq(a), CombinatorState::Seq(its)) => {
                let mut a_result = process(&a[0], c, &mut its[0]);
                for i in 1..its.len() {
                    let b_result = process(&a[i], c, &mut its[i]);
                    seq2_helper(&a[i], &mut a_result, &mut its[i]);
                    a_result |= b_result
                }
                a_result
            }
            (_self, _state) => panic!("Mismatched combinator and state types: {:?} vs {:?}", _self, _state),
        }
    }

    fn set(&mut self, combinator: Combinator) {
        match self {
            Combinator::ForwardRef(inner) => {
                let option: &mut Option<Combinator> = &mut inner.as_ref().borrow_mut();
                option.replace(combinator);
            }
            _ => panic!("Combinator is not a ForwardRef"),
        }
    }
}

fn process(combinator: &Combinator, c: Option<char>, its: &mut Vec<CombinatorState>) -> ParserIterationResult {
    let mut final_result = ParserIterationResult::new(U8Set::none(), false);
    let mut i = its.len();
    while i > 0 {
        i -= 1;
        let result = combinator.next_state(&mut its[i], c);
        if result.u8set.is_empty() {
            its.swap_remove(i);
        }
        final_result |= result;
    }
    final_result
}

fn seq2_helper(
    b: &Combinator,
    a_result: &mut ParserIterationResult,
    b_its: &mut Vec<CombinatorState>,
) {
    if a_result.is_complete {
        let b_it = b.initial_state();
        b_its.push(b_it);
        let b_result = b.next_state(b_its.last_mut().unwrap(), None);
        a_result.is_complete = b_result.is_complete;
        a_result.u8set |= b_result.u8set;
    }
}

fn seq<Combinators>(combinators: Combinators) -> Combinator
where
    Combinators: Into<Rc<[Combinator]>>,
{
    Combinator::Seq(combinators.into())
}

fn repeat1<C>(a: C) -> Combinator
where
    C: Into<Combinator>,
{
    Combinator::Repeat1(Box::new(a.into()))
}

fn choice<Combinators>(combinators: Combinators) -> Combinator
where
    Combinators: Into<Rc<[Combinator]>>,
{
    Combinator::Choice(combinators.into())
}

fn eat_u8_matching<F>(fn_: F) -> Combinator
where
    F: Fn(u8) -> bool,
{
    Combinator::EatU8Matching(U8Set::from_match_fn(&fn_))
}

fn eat_u8(value: char) -> Combinator {
    eat_u8_matching(move |c: u8| c == value as u8)
}

fn eat_u8_range(start: char, end: char) -> Combinator {
    eat_u8_matching(move |c: u8| (start as u8..=end as u8).contains(&c))
}

fn eat_u8_range_complement(start: char, end: char) -> Combinator {
    eat_u8_matching(move |c: u8| !(start as u8..=end as u8).contains(&c))
}

fn eat_string(value: &'static str) -> Combinator {
    Combinator::EatString(value)
}

fn eps() -> Combinator {
    Combinator::Eps
}

fn opt<C>(a: C) -> Combinator
where
    C: Into<Combinator>,
{
    choice(vec![a.into(), eps()])
}

fn repeat<C>(a: C) -> Combinator
where
    C: Into<Combinator>,
{
    opt(repeat1(a))
}

fn call<F>(f: F) -> Combinator
where
    F: Fn() -> Combinator + 'static,
{
    Combinator::Call(Rc::new(f))
}

fn forward_ref() -> Combinator {
    Combinator::ForwardRef(Rc::new(RefCell::new(None)))
}

macro_rules! seq {
    ($($a:expr),+ $(,)?) => {
        seq(vec![$($a.clone()),+])
    }
}

macro_rules! choice {
    ($($a:expr),+ $(,)?) => {
        choice(vec![$($a.clone()),+])
    }
}

struct ActiveCombinator {
    combinator: Combinator,
    state: CombinatorState,
}

impl ActiveCombinator {
    fn new(combinator: Combinator) -> Self {
        let state = combinator.initial_state();
        Self {
            combinator,
            state,
        }
    }

    fn send(&mut self, c: Option<char>) -> ParserIterationResult {
        self.combinator.next_state(&mut self.state, c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test cases remain the same, just update the combinator creation syntax
    #[test]
    fn test_eat_u8() {
        let mut it = ActiveCombinator::new(eat_u8('a').clone());
        let result0 = it.send(None);
        assert_eq!(result0, ParserIterationResult::new(U8Set::from_chars("a"), false));
        let result = it.send(Some('a'));
        assert_eq!(result, ParserIterationResult::new(U8Set::none(), true));
    }

    #[test]
    fn test_eat_string() {
        let mut it = ActiveCombinator::new(eat_string("abc").clone());
        let result0 = it.send(None);
        assert_eq!(result0, ParserIterationResult::new(U8Set::from_chars("a"), false));
        let result1 = it.send(Some('a'));
        assert_eq!(result1, ParserIterationResult::new(U8Set::from_chars("b"), false));
        let result2 = it.send(Some('b'));
        assert_eq!(result2, ParserIterationResult::new(U8Set::from_chars("c"), false));
        let result3 = it.send(Some('c'));
        assert_eq!(result3, ParserIterationResult::new(U8Set::none(), true));
    }

    #[test]
    fn test_seq() {
        let mut it = ActiveCombinator::new(seq!(eat_u8('a'), eat_u8('b')).clone());
        let result0 = it.send(None);
        assert_eq!(result0, ParserIterationResult::new(U8Set::from_chars("a"), false));
        let result1 = it.send(Some('a'));
        assert_eq!(result1, ParserIterationResult::new(U8Set::from_chars("b"), false));
        let result2 = it.send(Some('b'));
        assert_eq!(result2, ParserIterationResult::new(U8Set::none(), true));
    }

    #[test]
    fn test_repeat1() {
        let mut it = ActiveCombinator::new(repeat1(eat_u8('a')));
        let result0 = it.send(None);
        assert_eq!(result0, ParserIterationResult::new(U8Set::from_chars("a"), false));
        let result1 = it.send(Some('a'));
        assert_eq!(result1, ParserIterationResult::new(U8Set::from_chars("a"), true));
        let result2 = it.send(Some('a'));
        assert_eq!(result2, ParserIterationResult::new(U8Set::from_chars("a"), true));
    }

    #[test]
    fn test_choice() {
        let mut it = ActiveCombinator::new(choice!(eat_u8('a'), eat_u8('b')));
        let result0 = it.send(None);
        assert_eq!(result0, ParserIterationResult::new(U8Set::from_chars("ab"), false));
        let result1 = it.send(Some('a'));
        assert_eq!(result1, ParserIterationResult::new(U8Set::none(), true));

        let mut it = ActiveCombinator::new(choice!(eat_u8('a'), eat_u8('b')));
        it.send(None);
        let result2 = it.send(Some('b'));
        assert_eq!(result2, ParserIterationResult::new(U8Set::none(), true));
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
        assert_eq!(result0, ParserIterationResult::new(U8Set::from_chars("a"), false));
        let result1 = it.send(Some('a'));
        assert_eq!(result1, ParserIterationResult::new(U8Set::from_chars("bc"), false));
        let result2 = it.send(Some('b'));
        assert_eq!(result2, ParserIterationResult::new(U8Set::from_chars("c"), false));
        let result3 = it.send(Some('c'));
        assert_eq!(result3, ParserIterationResult::new(U8Set::none(), true));
    }

    #[test]
    fn test_nested_brackets() {
        fn nested_brackets() -> Combinator {
            choice!(
                seq!(eat_u8('['), seq!(call(nested_brackets), eat_u8(']'))),
                eat_u8('a')
            )
        }
        let mut it = ActiveCombinator::new(nested_brackets());
        let result0 = it.send(None);
        assert_eq!(result0, ParserIterationResult::new(U8Set::from_chars("[a"), false));
        let result1 = it.send(Some('['));
        assert_eq!(result1, ParserIterationResult::new(U8Set::from_chars("[a"), false));
        let result2 = it.send(Some('a'));
        assert_eq!(result2, ParserIterationResult::new(U8Set::from_chars("]"), false));
        let result3 = it.send(Some(']'));
        assert_eq!(result3, ParserIterationResult::new(U8Set::none(), true));
    }
}

#[cfg(test)]
mod json_parser {
    use super::*;

    #[test]
    fn test_json_parser() {
        // Helper combinators for JSON parsing
        let whitespace = repeat(choice!(eat_u8(' '), choice!(eat_u8('\t'), choice!(eat_u8('\n'), eat_u8('\r')))));
        let digit = eat_u8_range('0', '9');
        let digits = repeat(digit);
        let integer = seq!(opt(choice!(eat_u8('-'), eat_u8('+'))), digits);
        let fraction = seq!(eat_u8('.'), digits);
        let exponent = seq!(choice!(eat_u8('e'), eat_u8('E')), seq!(choice!(choice!(eat_u8('+'), eat_u8('-')), eps()), digits));
        let number = seq!(integer, seq!(opt(fraction), opt(exponent)));

        let string_char = choice!(
            eat_u8_range_complement('"', '"'),
            seq!(
                eat_u8('\\'),
                choice!(
                    choice!(
                        choice!(
                            choice!(eat_u8('"'), eat_u8('\\')),
                            choice!(eat_u8('/'), eat_u8('b')),
                        ),
                        choice!(eat_u8('f'), eat_u8('n')),
                    ),
                    choice!(
                        choice!(eat_u8('r'), eat_u8('t')),
                        seq!(
                            eat_u8('u'),
                            seq!(
                                eat_u8_range('0', '9'),
                                seq!(
                                    eat_u8_range('0', '9'),
                                    seq!(eat_u8_range('0', '9'), eat_u8_range('0', '9')),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );
        let string = seq!(eat_u8('"'), seq!(repeat(string_char), eat_u8('"')));

        let mut json_value = forward_ref();

        let json_array = seq!(
            eat_u8('['),
            seq!(
                whitespace,
                seq!(
                    opt(seq!(
                        json_value,
                        repeat(seq!(seq!(whitespace, eat_u8(',')), seq!(whitespace, json_value))),
                    )),
                    seq!(whitespace, eat_u8(']')),
                ),
            ),
        );

        let key_value_pair = seq!(seq!(whitespace, string), seq!(whitespace, seq!(eat_u8(':'), seq!(whitespace, json_value))));

        let json_object = seq!(
            eat_u8('{'),
            seq!(
                whitespace,
                seq!(
                    opt(seq!(
                        key_value_pair,
                        repeat(seq!(seq!(whitespace, eat_u8(',')), key_value_pair)),
                    )),
                    seq!(whitespace, eat_u8('}')),
                ),
            ),
        );

        json_value.set(
            choice!(
                choice!(string, number),
                choice!(
                    choice!(eat_string("true"), eat_string("false")),
                    choice!(eat_string("null"), choice!(json_array, json_object)),
                ),
            ),
        );

        // Test cases
        let json_parser = seq!(whitespace, json_value);

        let test_cases = [
            "null",
            "true",
            "false",
            "42",
            r#"{"key": "value"}"#,
            "[1, 2, 3]",
            r#"{"nested": {"array": [1, 2, 3], "object": {"a": true, "b": false}}}"#,
            r#""Hello, world!""#,
        ];

        let parse_json = |json_string: &str| -> bool {
            let mut it = ActiveCombinator::new(json_parser.clone());
            let mut result = it.send(None);
            for i in 0..json_string.len() {
                let char = json_string.chars().nth(i).unwrap();
                assert!(result.u8set.contains(char as u8), "Expected {} to be in {:?}", char, result.u8set);
                result = it.send(Some(char));
            }
            result.is_complete
        };

        for json_string in test_cases {
            assert!(parse_json(json_string), "Failed to parse JSON string: {}", json_string);
        }

        let invalid_json_strings = [
            r#"{"unclosed": "object""#,
            "[1, 2, 3",
            r#"{"invalid": "json","#,
        ];

        for json_string in invalid_json_strings {
            assert!(!parse_json(json_string), "Incorrectly parsed invalid JSON string: {}", json_string);
        }

        let filenames: Vec<&str> = vec![
            // "GeneratedCSV_mini.json",
            "GeneratedCSV_1.json",
            "GeneratedCSV_2.json",
            // "GeneratedCSV_10.json",
            // "GeneratedCSV_20.json",
            // "GeneratedCSV_100.json",
            // "GeneratedCSV_200.json",
        ];

        // Print execution times for each parser
        for filename in filenames {
            let json_string = std::fs::read_to_string(format!("static/{}", filename)).unwrap();
            let start = std::time::Instant::now();
            let result = parse_json(&json_string);
            let end = std::time::Instant::now();
            println!("{}: {} ms", filename, end.duration_since(start).as_millis());
            assert!(result, "Failed to parse JSON string: {}", json_string);
        }

        // Test with a string of 'a's
        println!("Testing with a string of 'a's of length 100 and length 200");
        for i in vec![1_000, 2_000, 1_000, 2_000] {
            let json_string = std::iter::repeat('a').take(i).collect::<String>();
            let json_string = format!(r#"{{"a": "{}"}}"#, json_string);
            let start = std::time::Instant::now();
            let result = parse_json(&json_string);
            let end = std::time::Instant::now();
            println!("{}: {} ms", i, end.duration_since(start).as_millis());
            assert!(result, "Failed to parse JSON string: {}", json_string);
        }
    }
}