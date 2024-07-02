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

trait Combinator {
    fn initial_state(&self, data: &Data) -> CombinatorState;
    fn next_state(&self, state: &mut CombinatorState, c: Option<char>) -> ParserIterationResult;
}

#[derive(Clone)]
enum CombinatorState {
    Choice2(Box<CombinatorState>, Box<CombinatorState>),
    EatString(usize),
    EatU8Matching(u8),
    Eps,
    Repeat1(Box<CombinatorState>),
    Seq2(Box<CombinatorState>, Box<CombinatorState>),
}

#[derive(Clone)]
enum CombinatorEnum {
    Choice2(Box<CombinatorEnum>, Box<CombinatorEnum>),
    EatString(String),
    EatU8Matching(U8Set),
    Eps,
    Repeat1(Box<CombinatorEnum>),
    Seq2(Box<CombinatorEnum>, Box<CombinatorEnum>),
}

impl Combinator for CombinatorEnum {
    fn initial_state(&self, data: &Data) -> CombinatorState {
        match self {
            CombinatorEnum::Choice2(a, b) => CombinatorState::Choice2(
                Box::new(a.initial_state(data)),
                Box::new(b.initial_state(data)),
            ),
            CombinatorEnum::EatString(_) => CombinatorState::EatString(0),
            CombinatorEnum::EatU8Matching(_) => CombinatorState::EatU8Matching(0),
            CombinatorEnum::Eps => CombinatorState::Eps,
            CombinatorEnum::Repeat1(a) => CombinatorState::Repeat1(Box::new(a.initial_state(data))),
            CombinatorEnum::Seq2(a, b) => CombinatorState::Seq2(
                Box::new(a.initial_state(data)),
                Box::new(b.initial_state(data)),
            ),
        }
    }

    fn next_state(&self, state: &mut CombinatorState, c: Option<char>) -> ParserIterationResult {
        match (self, state) {
            (CombinatorEnum::Choice2(a, b), CombinatorState::Choice2(sa, sb)) => {
                let mut result_a = a.next_state(sa, c);
                let result_b = b.next_state(sb, c);
                result_a |= result_b;
                result_a
            }
            (CombinatorEnum::EatString(value), CombinatorState::EatString(idx)) => {
                if *idx < value.len() {
                    let expected = value.chars().nth(*idx).unwrap();
                    if c.map_or(false, |c| c == expected) {
                        *idx += 1;
                        let is_complete = *idx == value.len();
                        ParserIterationResult::new(U8Set::none(), is_complete)
                    } else {
                        ParserIterationResult::new(U8Set::none(), true)
                    }
                } else {
                    ParserIterationResult::new(U8Set::none(), true)
                }
            }
            (CombinatorEnum::EatU8Matching(u8set), CombinatorState::EatU8Matching(state)) => {
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
                    _ => ParserIterationResult::new(U8Set::none(), true),
                }
            }
            (CombinatorEnum::Eps, CombinatorState::Eps) => {
                ParserIterationResult::new(U8Set::none(), true)
            }
            (CombinatorEnum::Repeat1(a), CombinatorState::Repeat1(sa)) => {
                let mut a_result = a.next_state(sa, c);
                let b_result = a_result.clone();
                if a_result.is_complete {
                    *sa = a.initial_state(&()).into();
                }
                a_result | b_result
            }
            (CombinatorEnum::Seq2(a, b), CombinatorState::Seq2(sa, sb)) => {
                let mut a_result = a.next_state(sa, c);
                let b_result = b.next_state(sb, c);
                if a_result.is_complete {
                    *sb = b.initial_state(&()).into();
                    a_result.is_complete = false;
                }
                a_result | b_result
            }
            _ => ParserIterationResult::new(U8Set::none(), true),
        }
    }
}

#[derive(Clone)]
struct ActiveCombinator {
    combinator: CombinatorEnum,
    data: Data,
    state: CombinatorState,
}

impl ActiveCombinator {
    fn new(combinator: CombinatorEnum, data: Data) -> Self {
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

fn process(c: Option<char>, its: &mut Vec<ActiveCombinator>) -> ParserIterationResult {
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
    b: CombinatorEnum,
    d: &Data,
    a_result: &mut ParserIterationResult,
    b_its: &mut Vec<ActiveCombinator>,
) {
    if a_result.is_complete {
        let b_it = ActiveCombinator::new(b, d.clone());
        b_its.push(b_it);
        let b_result = b_its.last_mut().unwrap().send(None);
        a_result.is_complete = b_result.is_complete;
        a_result.u8set |= b_result.u8set;
    }
}

fn seq(a: CombinatorEnum, b: CombinatorEnum) -> CombinatorEnum {
    CombinatorEnum::Seq2(Box::new(a), Box::new(b))
}

fn repeat1(a: CombinatorEnum) -> CombinatorEnum {
    CombinatorEnum::Repeat1(Box::new(a))
}

fn choice2(a: CombinatorEnum, b: CombinatorEnum) -> CombinatorEnum {
    CombinatorEnum::Choice2(Box::new(a), Box::new(b))
}

fn eat_u8_matching<F>(fn_: F) -> CombinatorEnum
where
    F: Fn(u8) -> bool,
{
    CombinatorEnum::EatU8Matching(U8Set::from_match_fn(&fn_))
}

fn eat_u8(value: char) -> CombinatorEnum {
    eat_u8_matching(move |c: u8| c == value as u8)
}

fn eat_u8_range(start: char, end: char) -> CombinatorEnum {
    eat_u8_matching(move |c: u8| (start as u8..=end as u8).contains(&c))
}

fn eat_u8_range_complement(start: char, end: char) -> CombinatorEnum {
    eat_u8_matching(move |c: u8| !(start as u8..=end as u8).contains(&c))
}

fn eat_string(value: &str) -> CombinatorEnum {
    CombinatorEnum::EatString(value.to_string())
}

fn eps() -> CombinatorEnum {
    CombinatorEnum::Eps
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
                choice2(eat_u8('a'), seq(eat_u8('a'), eat_u8('b'))),
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
        // let a = |a: CombinatorType| {
        //     choice2(seq(eat_u8('['), seq(a, eat_u8(']'))), eat_u8('a'))
        // };
        // let a = a(a(a(eps())));
        // let mut it = ActiveCombinator::new(a, ());
    }
}