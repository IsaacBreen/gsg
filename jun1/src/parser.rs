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

#[derive(Clone)]
enum Combinator {
    Choice2(Box<Combinator>, Box<Combinator>),
    EatString(String),
    EatU8Matching(U8Set),
    Eps,
    Repeat1(Box<Combinator>),
    Seq2(Box<Combinator>, Box<Combinator>),
}

#[derive(Clone)]
enum CombinatorState {
    Choice2(Box<CombinatorState>, Box<CombinatorState>),
    EatString(usize),
    EatU8Matching(u8),
    Eps,
    Repeat1(Vec<ActiveCombinator>),
    Seq2(Vec<ActiveCombinator>, Vec<ActiveCombinator>),
}

impl Combinator {
    fn initial_state(&self, data: &Data) -> CombinatorState {
        match self {
            Combinator::Choice2(a, b) => CombinatorState::Choice2(
                Box::new(a.initial_state(data)),
                Box::new(b.initial_state(data)),
            ),
            Combinator::EatString(_) => CombinatorState::EatString(0),
            Combinator::EatU8Matching(_) => CombinatorState::EatU8Matching(0),
            Combinator::Eps => CombinatorState::Eps,
            Combinator::Repeat1(a) => CombinatorState::Repeat1(vec![ActiveCombinator::new((**a).clone(), data.clone())]),
            Combinator::Seq2(a, b) => CombinatorState::Seq2(
                vec![ActiveCombinator::new((**a).clone(), data.clone())],
                Vec::new(),
            ),
        }
    }

    fn next_state(&self, state: &mut CombinatorState, c: Option<char>) -> ParserIterationResult {
        match (self, state) {
            (Combinator::Choice2(a, b), CombinatorState::Choice2(state_a, state_b)) => {
                let mut result_a = a.next_state(state_a, c);
                let result_b = b.next_state(state_b, c);
                result_a |= result_b;
                result_a
            }
            (Combinator::EatString(value), CombinatorState::EatString(index)) => {
                if *index < value.len() {
                    let expected = value.chars().nth(*index).unwrap();
                    if c.map_or(false, |c| c == expected) {
                        *index += 1;
                        let is_complete = *index == value.len();
                        ParserIterationResult::new(U8Set::none(), is_complete)
                    } else {
                        ParserIterationResult::new(U8Set::none(), true)
                    }
                } else {
                    ParserIterationResult::new(U8Set::none(), true)
                }
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
                    _ => ParserIterationResult::new(U8Set::none(), true),
                }
            }
            (Combinator::Eps, CombinatorState::Eps) => {
                ParserIterationResult::new(U8Set::none(), true)
            }
            (Combinator::Repeat1(a), CombinatorState::Repeat1(a_its)) => {
                let mut a_result = process(c, a_its);
                let b_result = a_result.clone();
                seq2_helper((**a).clone(), &(), &mut a_result, a_its);
                a_result | b_result
            }
            (Combinator::Seq2(a, b), CombinatorState::Seq2(a_its, b_its)) => {
                let mut a_result = process(c, a_its);
                let b_result = process(c, b_its);
                seq2_helper((**b).clone(), &(), &mut a_result, b_its);
                a_result | b_result
            }
            _ => panic!("Mismatched combinator and state types"),
        }
    }
}

#[derive(Clone)]
struct ActiveCombinator {
    combinator: Combinator,
    data: Data,
    state: CombinatorState,
}

impl ActiveCombinator {
    fn new(combinator: Combinator, data: Data) -> Self {
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
    b: Combinator,
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

fn seq2(a: Combinator, b: Combinator) -> Combinator {
    Combinator::Seq2(Box::new(a), Box::new(b))
}

fn seq(a: Combinator, b: Combinator) -> Combinator {
    seq2(a, b)
}

fn repeat1(a: Combinator) -> Combinator {
    Combinator::Repeat1(Box::new(a))
}

fn choice2(a: Combinator, b: Combinator) -> Combinator {
    Combinator::Choice2(Box::new(a), Box::new(b))
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

fn eat_string(value: &str) -> Combinator {
    Combinator::EatString(value.to_string())
}

fn eps() -> Combinator {
    Combinator::Eps
}

fn opt(a: Combinator) -> Combinator {
    choice2(a, eps())
}

fn repeat(a: Combinator) -> Combinator {
    opt(repeat1(a))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test cases remain the same, just update the combinator creation syntax
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
        fn nested_brackets() -> Combinator {
            choice2(
                seq(eat_u8('['), seq(call(nested_brackets), eat_u8(']'))),
                eat_u8('a')
            )
        }
        let _A = nested_brackets();
    }
}