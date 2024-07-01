use std::marker::PhantomData;
use crate::u8set::U8Set;

#[derive(Debug, Clone)]
pub struct ParserIterationResult {
    pub u8set: U8Set,
    pub is_complete: bool,
}

impl ParserIterationResult {
    pub fn new(u8set: U8Set, is_complete: bool) -> Self {
        Self { u8set, is_complete }
    }
}

impl std::ops::BitOr for ParserIterationResult {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self {
            u8set: &self.u8set | &other.u8set,
            is_complete: self.is_complete | other.is_complete,
        }
    }
}

impl std::ops::BitAnd for ParserIterationResult {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Self {
            u8set: &self.u8set & &other.u8set,
            is_complete: self.is_complete & other.is_complete,
        }
    }
}

pub trait Combinator {
    type State;

    fn initial_state(&self) -> Self::State;
    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult;
    fn clone_state(&self, state: &Self::State) -> Self::State;
}

pub struct ActiveCombinator<C: Combinator> {
    combinator: C,
    state: C::State,
}

impl<C: Combinator> ActiveCombinator<C> {
    pub fn new(combinator: C) -> Self {
        let state = combinator.initial_state();
        Self { combinator, state }
    }

    pub fn send(&mut self, c: Option<char>) -> ParserIterationResult {
        self.combinator.next_state(&mut self.state, c)
    }

    pub fn clone(&self) -> Self {
        Self {
            combinator: self.combinator.clone(),
            state: self.combinator.clone_state(&self.state),
        }
    }
}

#[derive(Clone)]
pub struct EatU8Matching<F: Fn(u8) -> bool + Clone> {
    f: F,
}

impl<F: Fn(u8) -> bool + Clone> EatU8Matching<F> {
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F: Fn(u8) -> bool + Clone> Combinator for EatU8Matching<F> {
    type State = u8;

    fn initial_state(&self) -> Self::State {
        0
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult {
        match *state {
            0 => {
                *state = 1;
                ParserIterationResult::new(U8Set::from_match_fn(&self.f), false)
            }
            1 => {
                *state = 2;
                ParserIterationResult::new(
                    U8Set::none(),
                    c.map(|ch| (self.f)(ch as u8)).unwrap_or(false),
                )
            }
            _ => ParserIterationResult::new(U8Set::none(), true),
        }
    }

    fn clone_state(&self, state: &Self::State) -> Self::State {
        *state
    }
}

pub fn eat_u8_matching<F: Fn(u8) -> bool + Clone>(f: F) -> EatU8Matching<F> {
    EatU8Matching::new(f)
}

pub fn eat_u8(value: char) -> impl Combinator {
    eat_u8_matching(move |c| c == value as u8)
}

pub fn eat_u8_range(start: char, end: char) -> impl Combinator {
    eat_u8_matching(move |c| (start as u8 <= c) && (c <= end as u8))
}

pub fn eat_u8_range_complement(start: char, end: char) -> impl Combinator {
    eat_u8_matching(move |c| (c < start as u8) || (end as u8 < c))
}

#[derive(Clone)]
pub struct EatString {
    value: String,
}

impl EatString {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Combinator for EatString {
    type State = usize;

    fn initial_state(&self) -> Self::State {
        0
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult {
        if *state < self.value.len() {
            if let Some(ch) = c {
                if ch == self.value.chars().nth(*state).unwrap() {
                    *state += 1;
                    let is_complete = *state == self.value.len();
                    ParserIterationResult::new(U8Set::none(), is_complete)
                } else {
                    ParserIterationResult::new(U8Set::none(), true)
                }
            } else {
                ParserIterationResult::new(U8Set::from_chars(&self.value[*state..(*state + 1)]), false)
            }
        } else {
            ParserIterationResult::new(U8Set::none(), true)
        }
    }

    fn clone_state(&self, state: &Self::State) -> Self::State {
        *state
    }
}

pub fn eat_string(value: &str) -> EatString {
    EatString::new(value.to_string())
}

#[derive(Clone)]
pub struct Eps;

impl Combinator for Eps {
    type State = ();

    fn initial_state(&self) -> Self::State {}

    fn next_state(&self, _state: &mut Self::State, _c: Option<char>) -> ParserIterationResult {
        ParserIterationResult::new(U8Set::none(), true)
    }

    fn clone_state(&self, _state: &Self::State) -> Self::State {}
}

pub fn eps() -> Eps {
    Eps
}


#[derive(Clone)]
pub struct Seq2<A: Combinator, B: Combinator> {
    a: A,
    b: B,
}

impl<A: Combinator, B: Combinator> Seq2<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A: Combinator, B: Combinator> Combinator for Seq2<A, B> {
    type State = (Vec<ActiveCombinator<A>>, Vec<ActiveCombinator<B>>);

    fn initial_state(&self) -> Self::State {
        (vec![ActiveCombinator::new(self.a.clone())], vec![])
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult {
        let (a_its, b_its) = state;
        let mut a_result = process(c, a_its);
        let b_result = process(c, b_its);

        if a_result.is_complete {
            let b_it = ActiveCombinator::new(self.b.clone());
            b_its.push(b_it);
            let new_b_result = b_its.last_mut().unwrap().send(None);
            a_result.is_complete = new_b_result.is_complete;
            a_result.u8set = &a_result.u8set | &new_b_result.u8set;
        }

        a_result | b_result
    }

    fn clone_state(&self, state: &Self::State) -> Self::State {
        (
            state.0.iter().map(|it| it.clone()).collect(),
            state.1.iter().map(|it| it.clone()).collect(),
        )
    }
}

pub fn seq2<A: Combinator, B: Combinator>(a: A, b: B) -> Seq2<A, B> {
    Seq2::new(a, b)
}

#[derive(Clone)]
pub struct Repeat1<A: Combinator> {
    a: A,
}

impl<A: Combinator> Repeat1<A> {
    pub fn new(a: A) -> Self {
        Self { a }
    }
}

impl<A: Combinator> Combinator for Repeat1<A> {
    type State = Vec<ActiveCombinator<A>>;

    fn initial_state(&self) -> Self::State {
        vec![ActiveCombinator::new(self.a.clone())]
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult {
        let mut a_result = process(c, state);
        if a_result.is_complete {
            let mut new_it = ActiveCombinator::new(self.a.clone());
            let new_result = new_it.send(None);
            state.push(new_it);
            a_result.is_complete = new_result.is_complete;
            a_result.u8set = &a_result.u8set | &new_result.u8set;
        }
        a_result
    }

    fn clone_state(&self, state: &Self::State) -> Self::State {
        state.iter().map(|it| it.clone()).collect()
    }
}

pub fn repeat1<A: Combinator>(a: A) -> Repeat1<A> {
    Repeat1::new(a)
}

#[derive(Clone)]
pub struct Choice<C: Combinator> {
    parsers: Vec<C>,
}

impl<C: Combinator> Choice<C> {
    pub fn new(parsers: Vec<C>) -> Self {
        Self { parsers }
    }
}

impl<C: Combinator> Combinator for Choice<C> {
    type State = Vec<ActiveCombinator<C>>;

    fn initial_state(&self) -> Self::State {
        self.parsers.iter().map(|p| ActiveCombinator::new(p.clone())).collect()
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult {
        process(c, state)
    }

    fn clone_state(&self, state: &Self::State) -> Self::State {
        state.iter().map(|it| it.clone()).collect()
    }
}

pub fn choice<C: Combinator>(parsers: Vec<C>) -> Choice<C> {
    Choice::new(parsers)
}

fn process<C: Combinator>(c: Option<char>, its: &mut Vec<ActiveCombinator<C>>) -> ParserIterationResult {
    let mut final_result = ParserIterationResult::new(U8Set::none(), false);
    its.retain_mut(|it| {
        let result = it.send(c);
        final_result = final_result | result.clone();
        !(result.is_complete && result.u8set.is_empty())
    });
    final_result
}

pub fn seq<C: Combinator>(parsers: Vec<C>) -> impl Combinator {
    parsers.into_iter().reduce(|a, b| Box::new(seq2(a, b)) as Box<dyn Combinator>).unwrap()
}

pub fn opt<C: Combinator>(a: C) -> impl Combinator {
    choice(vec![a, eps()])
}

pub fn repeat<C: Combinator>(a: C) -> impl Combinator {
    opt(repeat1(a))
}

