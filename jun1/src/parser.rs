use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};
use std::rc::Rc;
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
    Call(Rc<dyn Fn() -> Combinator>),
    Choice(Vec<Combinator>),
    EatString(String),
    EatU8Matching(U8Set),
    Eps,
    Repeat1(Box<Combinator>),
    Seq2(Box<Combinator>, Box<Combinator>),
}

#[derive(Clone)]
enum CombinatorState {
    Call(Option<Box<CombinatorState>>),
    Choice(Vec<ActiveCombinator>),
    EatString(usize),
    EatU8Matching(u8),
    Eps,
    Repeat1(Vec<ActiveCombinator>),
    Seq2(Vec<ActiveCombinator>, Vec<ActiveCombinator>),
}

impl Combinator {
    fn initial_state(&self, data: &Data) -> CombinatorState {
        match self {
            Combinator::Call(f) => CombinatorState::Call(Some(Box::new(f().initial_state(data)))),
            Combinator::Choice(a) => CombinatorState::Choice(a.iter().map(|a| ActiveCombinator::new(a.clone(), data.clone())).collect()),
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
            (Combinator::Call(f), CombinatorState::Call(inner_state)) => {
                let inner_state = inner_state.as_mut().unwrap();
                f().next_state(inner_state, c)
            }
            (Combinator::Choice(a), CombinatorState::Choice(its)) => {
                process(c, its)
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
        if result.u8set.is_empty() {
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
    Combinator::Choice(vec![a, b])
}

fn choice(combinators: Vec<Combinator>) -> Combinator {
    Combinator::Choice(combinators)
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

fn call<F>(f: F) -> Combinator
where
    F: Fn() -> Combinator + 'static,
{
    Combinator::Call(Rc::new(f))
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
    fn test_eat_string() {
        let mut it = ActiveCombinator::new(eat_string("abc"), ());
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
        let mut it = ActiveCombinator::new(nested_brackets(), ());
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