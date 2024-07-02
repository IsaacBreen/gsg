use std::marker::PhantomData;
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
struct ActiveCombinator {
    combinator: CombinatorType,
    data: Data,
    state: CombinatorState,
}

impl ActiveCombinator {
    fn new(combinator: CombinatorType, data: Data) -> Self {
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

fn process(
    c: Option<char>,
    its: &mut Vec<ActiveCombinator>,
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

fn seq2_helper(
    combinator: CombinatorType,
    d: &Data,
    a_result: &mut ParserIterationResult,
    b_its: &mut Vec<ActiveCombinator>,
) {
    if a_result.is_complete {
        let b_it = ActiveCombinator::new(combinator, d.clone());
        b_its.push(b_it);
        let b_result = b_its.last_mut().unwrap().send(None);
        a_result.is_complete = b_result.is_complete;
        a_result.u8set |= b_result.u8set;
    }
}

#[derive(Clone)]
enum CombinatorType {
    Choice2(Choice2),
    EatString(EatString),
    EatU8Matching(EatU8Matching),
    Eps(Eps),
    Repeat1(Box<CombinatorType>),
    Seq2(Seq2),
}

#[derive(Clone)]
enum CombinatorState {
    Choice2((Choice2State, Choice2State)),
    EatString(usize),
    EatU8Matching(u8),
    Eps(()),
    Repeat1((Vec<ActiveCombinator>, Data)),
    Seq2((Vec<ActiveCombinator>, Vec<ActiveCombinator>, Data)),
}

impl Combinator for CombinatorType {
    type State = CombinatorState;

    fn initial_state(&self, data: &Data) -> Self::State {
        match self {
            CombinatorType::Choice2(c) => CombinatorState::Choice2(c.initial_state(data)),
            CombinatorType::EatString(c) => CombinatorState::EatString(c.initial_state(data)),
            CombinatorType::EatU8Matching(c) => {
                CombinatorState::EatU8Matching(c.initial_state(data))
            }
            CombinatorType::Eps(c) => CombinatorState::Eps(c.initial_state(data)),
            CombinatorType::Repeat1(c) => {
                CombinatorState::Repeat1(c.initial_state(data))
            }
            CombinatorType::Seq2(c) => CombinatorState::Seq2(c.initial_state(data)),
        }
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult {
        match (self, state) {
            (CombinatorType::Choice2(c), CombinatorState::Choice2(s)) => {
                c.next_state(s, c)
            }
            (CombinatorType::EatString(c), CombinatorState::EatString(s)) => {
                c.next_state(s, c)
            }
            (
                CombinatorType::EatU8Matching(c),
                CombinatorState::EatU8Matching(s),
            ) => c.next_state(s, c),
            (CombinatorType::Eps(c), CombinatorState::Eps(s)) => c.next_state(s, c),
            (CombinatorType::Repeat1(c), CombinatorState::Repeat1(s)) => {
                c.next_state(s, c)
            }
            (CombinatorType::Seq2(c), CombinatorState::Seq2(s)) => c.next_state(s, c),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct Seq2 {
    a: CombinatorType,
    b: CombinatorType,
}

impl Seq2 {
    fn new(a: CombinatorType, b: CombinatorType) -> Self {
        Self { a, b }
    }
}

impl Combinator for Seq2 {
    type State = (Vec<ActiveCombinator>, Vec<ActiveCombinator>, Data);

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

fn seq2(a: CombinatorType, b: CombinatorType) -> CombinatorType {
    CombinatorType::Seq2(Seq2::new(a, b))
}

// Box is used here to allow for recursive types
fn seq(
    a: CombinatorType,
    b: CombinatorType
) -> CombinatorType {
    seq2(a, b)
}


#[derive(Clone)]
struct Repeat1(CombinatorType);

impl Combinator for Repeat1 {
    type State = (Vec<ActiveCombinator>, Data);

    fn initial_state(&self, data: &Data) -> Self::State {
        (
            vec![ActiveCombinator::new(self.0.clone(), data.clone())],
            data.clone(),
        )
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult {
        let (a_its, d) = state;
        let mut a_result = process(c, a_its);
        let b_result = a_result.clone();
        seq2_helper(self.0.clone(), d, &mut a_result, a_its);
        a_result | b_result
    }
}

fn repeat1(a: CombinatorType) -> CombinatorType {
    CombinatorType::Repeat1(Box::new(a))
}

#[derive(Clone)]
struct Choice2 {
    a: CombinatorType,
    b: CombinatorType,
}

#[derive(Clone)]
struct Choice2State(CombinatorState, CombinatorState);

impl Choice2 {
    fn new(a: CombinatorType, b: CombinatorType) -> Self {
        Self { a, b }
    }
}

impl Combinator for Choice2 {
    type State = (CombinatorState, CombinatorState);

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

fn choice2(a: CombinatorType, b: CombinatorType) -> CombinatorType {
    CombinatorType::Choice2(Choice2::new(a, b))
}

#[derive(Clone)]
struct EatU8Matching {
    u8set: U8Set,
}

impl EatU8Matching {
    fn new<F>(fn_: F) -> Self
    where
        F: Fn(u8) -> bool,
    {
        Self {
            u8set: U8Set::from_match_fn(&fn_),
        }
    }
}

impl Combinator for EatU8Matching {
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
                    U8Set::none(),
                    c.map(|c| self.u8set.contains(c as u8)).unwrap_or(false),
                )
            }
            _ => ParserIterationResult::new(U8Set::none(), true),
        }
    }
}

fn eat_u8_matching<F>(fn_: F) -> CombinatorType
where
    F: Fn(u8) -> bool,
{
    CombinatorType::EatU8Matching(EatU8Matching::new(fn_))
}

fn eat_u8(value: char) -> CombinatorType {
    eat_u8_matching(move |c: u8| c == value as u8)
}

fn eat_u8_range(start: char, end: char) -> CombinatorType {
    eat_u8_matching(move |c: u8| (start as u8..=end as u8).contains(&c))
}

fn eat_u8_range_complement(start: char, end: char) -> CombinatorType {
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

fn eat_string(value: &str) -> CombinatorType {
    CombinatorType::EatString(EatString::new(value))
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

fn eps() -> CombinatorType {
    CombinatorType::Eps(Eps)
}

fn opt(a: CombinatorType) -> CombinatorType {
    choice2(a, eps())
}

fn repeat(a: CombinatorType) -> CombinatorType {
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
    fn test_nested_brackets() {
        let a = |a: CombinatorType| {
            choice2(seq(eat_u8('['), seq(a, eat_u8(']'))), eat_u8('a'))
        };
        let a = a(a(a(eps())));
        let mut it = ActiveCombinator::new(a, ());
    }
}