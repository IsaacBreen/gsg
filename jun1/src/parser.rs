use std::ops::{BitOr, BitAnd};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct BitSet {
    x: u32,
}

impl BitSet {
    fn is_set(&self, index: u8) -> bool {
        (self.x & (1 << index)) != 0
    }

    fn set_bit(&mut self, index: u8) {
        self.x |= 1 << index;
    }

    fn clear_bit(&mut self, index: u8) {
        self.x &= !(1 << index);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct U8Set {
    bitset: BitSet,
}

impl U8Set {
    fn insert(&mut self, value: u8) -> bool {
        if self.contains(value) {
            return false;
        }
        self.bitset.set_bit(value);
        true
    }

    fn remove(&mut self, value: u8) -> bool {
        if !self.contains(value) {
            return false;
        }
        self.bitset.clear_bit(value);
        true
    }

    fn update(&mut self, other: &U8Set) {
        self.bitset.x |= other.bitset.x;
    }

    fn contains(&self, value: u8) -> bool {
        self.bitset.is_set(value)
    }

    fn len(&self) -> u32 {
        self.bitset.x.count_ones()
    }

    fn is_empty(&self) -> bool {
        self.bitset.x == 0
    }

    fn clear(&mut self) {
        self.bitset.x = 0;
    }

    fn all() -> Self {
        Self { bitset: BitSet { x: u32::max_value() } }
    }

    fn none() -> Self {
        Self { bitset: BitSet { x: 0 } }
    }

    fn from_chars(chars: &str) -> Self {
        let mut result = Self::none();
        for char in chars.chars() {
            result.insert(char as u8);
        }
        result
    }

    fn from_match_fn<F>(mut fn_: F) -> Self
    where
        F: FnMut(u8) -> bool,
    {
        let mut result = Self::none();
        for i in 0..256 {
            if fn_(i) {
                result.insert(i);
            }
        }
        result
    }
}

impl BitOr for U8Set {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self {
            bitset: BitSet {
                x: self.bitset.x | other.bitset.x,
            },
        }
    }
}

impl BitAnd for U8Set {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Self {
            bitset: BitSet {
                x: self.bitset.x & other.bitset.x,
            },
        }
    }
}

impl IntoIterator for U8Set {
    type Item = u8;
    type IntoIter = U8SetIterator;

    fn into_iter(self) -> Self::IntoIter {
        U8SetIterator {
            set: self,
            index: 0,
        }
    }
}

struct U8SetIterator {
    set: U8Set,
    index: u8,
}

impl Iterator for U8SetIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 255 {
            self.index += 1;
            if self.set.contains(self.index - 1) {
                return Some(self.index - 1);
            }
        }
        None
    }
}

fn balanced_tree_reduce<T, F>(func: F, iterable: &[T], initial: Option<T>) -> T
where
    F: Fn(T, T) -> T,
    T: Clone,
{
    let mut items: Vec<T> = iterable.to_vec();

    if items.is_empty() {
        if let Some(value) = initial {
            return value;
        } else {
            panic!("balanced_tree_reduce() of empty sequence with no initial value");
        }
    }

    if let Some(value) = initial {
        items.insert(0, value);
    }

    while items.len() > 1 {
        let mut new_items: Vec<T> = Vec::new();
        for i in (0..items.len() - 1).step_by(2) {
            new_items.push(func(items[i].clone(), items[i + 1].clone()));
        }
        if items.len() % 2 != 0 {
            new_items.push(items.last().unwrap().clone());
        }
        items = new_items;
    }

    items.remove(0)
}

#[derive(Debug)]
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

impl BitAnd for ParserIterationResult {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Self {
            u8set: self.u8set & other.u8set,
            is_complete: self.is_complete & other.is_complete,
        }
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

trait Combinator<State = ()> {
    fn initial_state(&self, data: &Data) -> State;
    fn next_state(&self, state: &mut State, c: Option<char>) -> ParserIterationResult;
    fn clone_state(&self, state: &State) -> State
    where
        State: Clone,
    {
        state.clone()
    }
}

struct ActiveCombinator<'a, C: Combinator> {
    combinator: &'a C,
    data: Data,
    state: C::State,
}

impl<'a, C: Combinator> ActiveCombinator<'a, C> {
    fn new(combinator: &'a C, data: Data) -> Self {
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

    fn clone(&self) -> Self
    where
        C::State: Clone,
    {
        Self {
            combinator: self.combinator,
            data: self.data.clone(),
            state: self.combinator.clone_state(&self.state),
        }
    }
}

fn process<'a, C: Combinator>(
    c: Option<char>,
    its: &mut Vec<ActiveCombinator<'a, C>>,
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

fn seq2_helper<'a, B: Combinator>(
    b: &B,
    d: &Data,
    a_result: &mut ParserIterationResult,
    b_its: &mut Vec<ActiveCombinator<'a, B>>,
) {
    if a_result.is_complete {
        let b_it = ActiveCombinator::new(b, d.clone());
        b_its.push(b_it);
        let b_result = b_its.last_mut().unwrap().send(None);
        a_result.is_complete = b_result.is_complete;
        a_result.u8set |= b_result.u8set;
    }
}

struct Seq2<'a, A, B>
where
    A: Combinator,
    B: Combinator,
{
    a: &'a A,
    b: &'a B,
}

impl<'a, A, B> Seq2<'a, A, B>
where
    A: Combinator,
    B: Combinator,
{
    fn new(a: &'a A, b: &'a B) -> Self {
        Self { a, b }
    }
}

impl<'a, A, B> Combinator for Seq2<'a, A, B>
where
    A: Combinator,
    B: Combinator,
{
    type State = (Vec<ActiveCombinator<'a, A>>, Vec<ActiveCombinator<'a, B>>, Data);

    fn initial_state(&self, data: &Data) -> Self::State {
        (
            vec![ActiveCombinator::new(self.a, data.clone())],
            Vec::new(),
            data.clone(),
        )
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult {
        let (a_its, b_its, d) = state;
        let mut a_result = process(c, a_its);
        let b_result = process(c, b_its);
        seq2_helper(self.b, d, &mut a_result, b_its);
        a_result | b_result
    }

    fn clone_state(&self, state: &Self::State) -> Self::State
    where
        Self::State: Clone,
    {
        (
            state.0.iter().map(|it| it.clone()).collect(),
            state.1.iter().map(|it| it.clone()).collect(),
            state.2.clone(),
        )
    }
}

fn seq2<'a, A, B>(a: &'a A, b: &'a B) -> Seq2<'a, A, B>
where
    A: Combinator,
    B: Combinator,
{
    Seq2::new(a, b)
}

fn seq<'a, C: Combinator>(combinators: &'a [C]) -> Box<dyn Combinator<State = (Box<dyn Combinator<State = C::State> + 'a>, C::State)> + 'a>
where
    C::State: Clone,
{
    match combinators.len() {
        0 => panic!("seq() called with empty slice"),
        1 => Box::new(combinators[0]),
        _ => {
            let (left, right) = combinators.split_at(combinators.len() / 2);
            let left_combinator = seq(left);
            let right_combinator = seq(right);
            Box::new(seq2(&left_combinator, &right_combinator))
        }
    }
}

struct Repeat1<'a, A>(&'a A)
where
    A: Combinator;

impl<'a, A> Combinator for Repeat1<'a, A>
where
    A: Combinator,
{
    type State = (Vec<ActiveCombinator<'a, A>>, Data);

    fn initial_state(&self, data: &Data) -> Self::State {
        (vec![ActiveCombinator::new(self.0, data.clone())], data.clone())
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult {
        let (a_its, d) = state;
        let mut a_result = process(c, a_its);
        seq2_helper(self.0, d, &mut a_result.clone(), a_its);
        a_result | process(c, a_its)
    }

    fn clone_state(&self, state: &Self::State) -> Self::State
    where
        Self::State: Clone,
    {
        (
            state.0.iter().map(|it| it.clone()).collect(),
            state.1.clone(),
        )
    }
}

fn repeat1<'a, A>(a: &'a A) -> Repeat1<'a, A>
where
    A: Combinator,
{
    Repeat1(a)
}

struct Choice<'a, C>
where
    C: Combinator,
{
    parsers: &'a [C],
}

impl<'a, C> Choice<'a, C>
where
    C: Combinator,
{
    fn new(parsers: &'a [C]) -> Self {
        Self { parsers }
    }
}

impl<'a, C> Combinator for Choice<'a, C>
where
    C: Combinator,
{
    type State = Vec<ActiveCombinator<'a, C>>;

    fn initial_state(&self, data: &Data) -> Self::State {
        self.parsers
            .iter()
            .map(|parser| ActiveCombinator::new(parser, data.clone()))
            .collect()
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult {
        process(c, state)
    }

    fn clone_state(&self, state: &Self::State) -> Self::State
    where
        Self::State: Clone,
    {
        state.iter().map(|it| it.clone()).collect()
    }
}

fn choice<'a, C: Combinator>(parsers: &'a [C]) -> Choice<'a, C> {
    Choice::new(parsers)
}

struct EatU8Matching<F>
where
    F: Fn(u8) -> bool,
{
    fn_: F,
}

impl<F> EatU8Matching<F>
where
    F: Fn(u8) -> bool,
{
    fn new(fn_: F) -> Self {
        Self { fn_ }
    }
}

impl<F> Combinator for EatU8Matching<F>
where
    F: Fn(u8) -> bool,
{
    type State = u8;

    fn initial_state(&self, _data: &Data) -> Self::State {
        0
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>) -> ParserIterationResult {
        match *state {
            0 => {
                *state = 1;
                ParserIterationResult::new(U8Set::from_match_fn(&self.fn_), false)
            }
            1 => {
                *state = 2;
                ParserIterationResult::new(
                    U8Set::none(),
                    c.map(|c| (self.fn_)(c as u8)).unwrap_or(false),
                )
            }
            _ => ParserIterationResult::new(U8Set::none(), true),
        }
    }
}

fn eat_u8_matching<F>(fn_: F) -> EatU8Matching<F>
where
    F: Fn(u8) -> bool,
{
    EatU8Matching::new(fn_)
}

fn eat_u8(value: char) -> EatU8Matching<impl Fn(u8) -> bool> {
    eat_u8_matching(move |c: u8| c == value as u8)
}

fn eat_u8_range(start: char, end: char) -> EatU8Matching<impl Fn(u8) -> bool> {
    eat_u8_matching(move |c: u8| (start as u8..=end as u8).contains(&c))
}

fn eat_u8_range_complement(start: char, end: char) -> EatU8Matching<impl Fn(u8) -> bool> {
    eat_u8_matching(move |c: u8| !(start as u8..=end as u8).contains(&c))
}

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

struct Eps;

impl Combinator for Eps {
    fn initial_state(&self, _data: &Data) -> Self::State {}
    fn next_state(&self, _state: &mut Self::State, _c: Option<char>) -> ParserIterationResult {
        ParserIterationResult::new(U8Set::none(), true)
    }
}

fn eps() -> Eps {
    Eps
}

fn opt<'a, A: Combinator>(a: &'a A) -> Choice<'a, A> {
    choice(&[a, &eps()])
}

fn repeat<'a, A: Combinator>(a: &'a A) -> Choice<'a, Repeat1<'a, A>> {
    opt(&repeat1(a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eat_u8() {
        let mut it = ActiveCombinator::new(&eat_u8('a'), ());
        let result0 = it.send(None);
        assert_eq!(result0, ParserIterationResult::new(U8Set::from_chars("a"), false));
        let result = it.send(Some('a'));
        assert_eq!(result, ParserIterationResult::new(U8Set::none(), true));
    }

    #[test]
    fn test_seq() {
        let mut it = ActiveCombinator::new(&seq(&[eat_u8('a'), eat_u8('b')]), ());
        let result0 = it.send(None);
        assert_eq!(result0, ParserIterationResult::new(U8Set::from_chars("a"), false));
        let result1 = it.send(Some('a'));
        assert_eq!(result1, ParserIterationResult::new(U8Set::from_chars("b"), false));
        let result2 = it.send(Some('b'));
        assert_eq!(result2, ParserIterationResult::new(U8Set::none(), true));
    }

    #[test]
    fn test_repeat1() {
        let mut it = ActiveCombinator::new(&repeat1(&eat_u8('a')), ());
        let result0 = it.send(None);
        assert_eq!(result0, ParserIterationResult::new(U8Set::from_chars("a"), false));
        let result1 = it.send(Some('a'));
        assert_eq!(result1, ParserIterationResult::new(U8Set::from_chars("a"), true));
        let result2 = it.send(Some('a'));
        assert_eq!(result2, ParserIterationResult::new(U8Set::from_chars("a"), true));
    }

    #[test]
    fn test_choice() {
        let mut it = ActiveCombinator::new(&choice(&[eat_u8('a'), eat_u8('b')]), ());
        let result0 = it.send(None);
        assert_eq!(result0, ParserIterationResult::new(U8Set::from_chars("ab"), false));
        let result1 = it.send(Some('a'));
        assert_eq!(result1, ParserIterationResult::new(U8Set::none(), true));

        let mut it = ActiveCombinator::new(&choice(&[eat_u8('a'), eat_u8('b')]), ());
        it.send(None);
        let result2 = it.send(Some('b'));
        assert_eq!(result2, ParserIterationResult::new(U8Set::none(), true));
    }

    #[test]
    fn test_seq_choice_seq() {
        // Matches "ac" or "abc"
        let mut it = ActiveCombinator::new(
            &seq(&[
                choice(&[eat_u8('a'), seq(&[eat_u8('a'), eat_u8('b')])]),
                eat_u8('c'),
            ]),
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
}