// TODO:
//  - remove WrappedCombinatorState
//  - remove Signals, replace it with Signals2
//  - make id_complete a boolean, call it is_complete
//  - remove Signals, replace it with Signals2
//  - remove node from ParserIterationResult
//  - replace stage in SignalWrap with a boolean
//  - rename signal_id to something else to distinguish it from signal_type_id (what is it?)
//  - rename signal_type_id to something more suitable (what is it?)
//  - clean up the parse result struct and its methods. Look at its construction and usage and redesign it to make it simpler.
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};
use std::rc::Rc;
use crate::gss::GSSNode;
use crate::parse_iteration_result::{ParserIterationResult, SignalAtom, Signals};
use crate::u8set::U8Set;

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
    SignalWrap(SignalAtom, SignalAtom, Box<Combinator>),
}


impl PartialEq for Combinator {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Combinator::Call(a), Combinator::Call(b)) => Rc::ptr_eq(a, b),
            (Combinator::Choice(a), Combinator::Choice(b)) => Rc::ptr_eq(a, b),
            (Combinator::EatString(a), Combinator::EatString(b)) => a == b,
            (Combinator::EatU8Matching(a), Combinator::EatU8Matching(b)) => a == b,
            (Combinator::Eps, Combinator::Eps) => true,
            (Combinator::ForwardRef(a), Combinator::ForwardRef(b)) => Rc::ptr_eq(a, b),
            (Combinator::Repeat1(a), Combinator::Repeat1(b)) => *a == *b,
            (Combinator::Seq(a), Combinator::Seq(b)) => Rc::ptr_eq(a, b),
            (Combinator::SignalWrap(_, _, _), Combinator::SignalWrap(_, _, _)) => self == other,
            _ => false,
        }
    }
}

impl Eq for Combinator {}

impl Hash for Combinator {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Combinator::Call(a) => {
                // Hash the function pointer, not the Rc
                let a = Rc::as_ptr(a);
                a.hash(state);
            }
            Combinator::Choice(a) => a.hash(state),
            Combinator::EatString(a) => a.hash(state),
            Combinator::EatU8Matching(a) => a.hash(state),
            Combinator::Eps => std::mem::discriminant(self).hash(state),
            Combinator::ForwardRef(a) => {
                // Hash the RefCell, not the Rc
                let a = Rc::as_ptr(a);
                a.hash(state);
            }
            Combinator::Repeat1(a) => a.hash(state),
            Combinator::Seq(a) => a.hash(state),
            Combinator::SignalWrap(start_signal_atom, end_signal_atom, a) => {
                start_signal_atom.hash(state);
                end_signal_atom.hash(state);
                a.hash(state);
            }
        }
    }
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
            Combinator::SignalWrap(signal_atom, end_signal_atom, a) => write!(f, "EmitSignal({:?}, {:?}, {:?})", signal_atom, end_signal_atom, a),
        }
    }
}

#[derive(Debug)]
enum CombinatorState {
    Call(Option<Box<CombinatorState>>),
    Choice(Vec<Vec<CombinatorState>>),
    EatString(usize, usize),
    EatU8Matching(u8, usize),
    Eps(usize),
    ForwardRef(Box<CombinatorState>),
    Repeat1(Vec<CombinatorState>),
    Seq(Vec<Vec<CombinatorState>>),
    SignalWrap(usize, usize, Box<CombinatorState>),
}

#[derive(Debug)]
struct WrappedCombinatorState {
    state: CombinatorState,
    signals: Signals,
}

impl Combinator {
    fn initial_state(&self, node: Option<&GSSNode<()>>, signal_id: &mut usize) -> CombinatorState {
        match self {
            Combinator::Call(f) => CombinatorState::Call(Some(Box::new(f().initial_state(node, signal_id)))),
            Combinator::Choice(a) => CombinatorState::Choice(a.iter().map(|a| vec![a.initial_state(node, signal_id)]).collect()),
            Combinator::EatString(_) => CombinatorState::EatString(0, { *signal_id += 1; *signal_id }),
            Combinator::EatU8Matching(_) => CombinatorState::EatU8Matching(0, { *signal_id += 1; *signal_id }),
            Combinator::Eps => CombinatorState::Eps({ *signal_id += 1; *signal_id }),
            Combinator::ForwardRef(c) => {
                match c.as_ref().borrow().as_ref() {
                    Some(c) => CombinatorState::ForwardRef(Box::new(c.initial_state(node, signal_id))),
                    None => panic!("ForwardRef not set"),
                }
            }
            Combinator::Repeat1(a) => CombinatorState::Repeat1(vec![a.initial_state(node, signal_id)]),
            Combinator::Seq(a) => {
                let mut its = Vec::with_capacity(a.len());
                its.push(vec![a[0].initial_state(node, signal_id)]);
                for _ in 1..a.len() {
                    its.push(Vec::new());
                }
                CombinatorState::Seq(its)
            }
            Combinator::SignalWrap(start_signal_atom, end_signal_atom, a) => CombinatorState::SignalWrap({ *signal_id += 1; *signal_id }, 0, Box::new(a.initial_state(node, signal_id))),
        }
    }

    fn next_state(&self, state: &mut CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        match (self, state) {
            (Combinator::Call(f), CombinatorState::Call(inner_state)) => {
                let inner_state = inner_state.as_mut().unwrap();
                f().next_state(inner_state, c, signal_id)
            }
            (Combinator::Choice(combinators), CombinatorState::Choice(its)) => {
                let mut final_result = ParserIterationResult::new(U8Set::none(), None, Signals::default());
                for (combinator, its) in combinators.iter().zip(its.iter_mut()) {
                    final_result.merge_assign(process(combinator, c, its, signal_id));
                }
                final_result
            }
            (Combinator::EatString(value), CombinatorState::EatString(index, signal_id)) => {
                if *index > value.len() {
                    return ParserIterationResult::new(U8Set::none(), None, Default::default());
                }
                if *index == value.len() {
                    let mut result = ParserIterationResult::new(U8Set::none(), Some(*signal_id), Default::default());
                    result.signals2.add_finished(*signal_id);
                    return result;
                }
                let u8set = U8Set::from_chars(&value[*index..=*index]);
                *index += 1;
                ParserIterationResult::new(u8set, None, Default::default())
            }
            (Combinator::EatU8Matching(u8set), CombinatorState::EatU8Matching(state, signal_id)) => {
                match *state {
                    0 => {
                        *state = 1;
                        ParserIterationResult::new(u8set.clone(), None, Default::default())
                    }
                    1 => {
                        *state = 2;
                        let finished = c.map(|c| u8set.contains(c as u8)).unwrap_or(false);
                        let mut result = ParserIterationResult::new(
                            U8Set::none(),
                            finished.then_some(*signal_id),
                            Default::default(),
                        );
                        if finished {
                            result.signals2.add_finished(*signal_id);
                        }
                        result
                    }
                    _ => panic!("EatU8Matching: state out of bounds"),
                }
            }
            (Combinator::Eps, CombinatorState::Eps(signal_id)) => {
                let mut result = ParserIterationResult::new(U8Set::none(), Some(*signal_id), Default::default());
                result.signals2.add_finished(*signal_id);
                result
            }
            (Combinator::ForwardRef(inner), CombinatorState::ForwardRef(inner_state)) => {
                match inner.as_ref().borrow().as_ref() {
                    Some(combinator) => {
                        combinator.next_state(inner_state, c, signal_id)
                    }
                    None => {
                        panic!("Forward reference not set before use");
                    }
                }
            }
            (Combinator::Repeat1(a), CombinatorState::Repeat1(a_its)) => {
                let mut a_result = process(a.as_ref(), c, a_its, signal_id);
                let b_result = a_result.clone();
                seq2_helper(a, &mut a_result, b_result, a_its, signal_id);
                a_result
            }
            (Combinator::Seq(a), CombinatorState::Seq(its)) => {
                let mut a_result = process(&a[0], c, &mut its[0], signal_id);
                for (combinator, its) in a.iter().zip(its.iter_mut()).skip(1) {
                    let b_result = process(combinator, c, its, signal_id);
                    seq2_helper(combinator, &mut a_result, b_result, its, signal_id);
                }
                a_result
            }
            (Combinator::SignalWrap(start_signal_atom, end_signal_atom, a), CombinatorState::SignalWrap(start_signal_id, stage, state)) => {
                let mut result = a.next_state(state, c, signal_id);
                if result.is_complete() {
                    let new_signal_id = { *signal_id += 1; *signal_id };
                    result.signals2.push_to_finished(new_signal_id, end_signal_atom.clone());
                }
                if *stage == 0 {
                    let new_signal_id = { *signal_id += 1; *signal_id };
                    let active_signal_ids = state.get_active_signal_ids();
                    result.signals2.push_to_many(active_signal_ids, new_signal_id, start_signal_atom.clone());
                    state.set_active_signal_ids(new_signal_id);
                    *stage += 1;
                }
                result
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

impl CombinatorState {
    fn state_iter(&self) -> StateIter {
        StateIter::new(self)
    }
}

struct StateIter<'a> {
    stack: Vec<&'a CombinatorState>,
}

impl<'a> StateIter<'a> {
    fn new(state: &'a CombinatorState) -> Self {
        Self {
            stack: vec![state],
        }
    }
}

impl<'a> Iterator for StateIter<'a> {
    type Item = &'a CombinatorState;

    fn next(&mut self) -> Option<Self::Item> {
        let state = self.stack.pop()?;
        match state {
            CombinatorState::Call(inner_state) => {
                if let Some(inner_state) = inner_state {
                    self.stack.push(inner_state);
                }
            }
            CombinatorState::Choice(states) => {
                for sub_states in states.iter() {
                    for state in sub_states.iter() {
                        self.stack.push(state);
                    }
                }
            }
            CombinatorState::Repeat1(states) => {
                for state in states.iter() {
                    self.stack.push(state);
                }
            }
            CombinatorState::Seq(states) => {
                for sub_states in states.iter() {
                    for state in sub_states.iter() {
                        self.stack.push(state);
                    };
                }
            }
            CombinatorState::ForwardRef(inner_state) => {
                self.stack.push(inner_state);
            }
            CombinatorState::SignalWrap(_, _, inner_state) => {
                self.stack.push(inner_state);
            }
            CombinatorState::EatString(..) | CombinatorState::EatU8Matching(..) | CombinatorState::Eps(..) => {}
        }
        Some(state)
    }
}

impl CombinatorState {
    fn state_iter_mut(&mut self) -> StateIterMut {
        StateIterMut::new(self)
    }
}

struct StateIterMut<'a> {
    stack: Vec<*mut CombinatorState>,
    _phantom: std::marker::PhantomData<&'a mut CombinatorState>,
}

impl<'a> StateIterMut<'a> {
    fn new(state: &'a mut CombinatorState) -> Self {
        Self {
            stack: vec![state as *mut CombinatorState],
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a> Iterator for StateIterMut<'a> {
    type Item = &'a mut CombinatorState;

    fn next(&mut self) -> Option<Self::Item> {
        let state_ptr = self.stack.pop()?;

        // SAFETY: We know this pointer is valid because it came from a &mut reference,
        // and we're careful not to create overlapping mutable references.
        let state = unsafe { &mut *state_ptr };

        match state {
            CombinatorState::Call(inner_state) => {
                if let Some(inner_state) = inner_state {
                    self.stack.push(inner_state.as_mut() as *mut CombinatorState);
                }
            }
            CombinatorState::Choice(states) => {
                for sub_states in states.iter_mut().rev() {
                    for state in sub_states.iter_mut().rev() {
                        self.stack.push(state as *mut CombinatorState);
                    }
                }
            }
            CombinatorState::Repeat1(states) => {
                for state in states.iter_mut().rev() {
                    self.stack.push(state as *mut CombinatorState);
                }
            }
            CombinatorState::Seq(states) => {
                for sub_states in states.iter_mut().rev() {
                    for state in sub_states.iter_mut().rev() {
                        self.stack.push(state as *mut CombinatorState);
                    }
                }
            }
            CombinatorState::ForwardRef(inner_state) => {
                self.stack.push(inner_state.as_mut() as *mut CombinatorState);
            }
            CombinatorState::SignalWrap(_, _, inner_state) => {
                self.stack.push(inner_state.as_mut() as *mut CombinatorState);
            }
            CombinatorState::EatString(..) | CombinatorState::EatU8Matching(..) | CombinatorState::Eps(..) => {}
        }

        Some(state)
    }
}

impl CombinatorState {
    fn get_active_signal_ids(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for state in self.state_iter() {
            match state {
                CombinatorState::EatU8Matching(_, signal_id) => result.push(*signal_id),
                CombinatorState::EatString(_, signal_id) => result.push(*signal_id),
                CombinatorState::Eps(signal_id) => result.push(*signal_id),
                _ => ()
            }
        }
        result
    }

    fn set_active_signal_ids(&mut self, signal_id: usize) {
        for state in self.state_iter_mut() {
            match state {
                CombinatorState::EatU8Matching(_, ref mut id) => *id = signal_id,
                CombinatorState::EatString(_, ref mut id) => *id = signal_id,
                CombinatorState::Eps(ref mut id) => *id = signal_id,
                _ => ()
            }
        }
    }
}

trait GetCombinatorState {
    fn get_combinator_state(&self) -> &CombinatorState;
    fn get_combinator_state_mut(&mut self) -> &mut CombinatorState;
}

impl GetCombinatorState for WrappedCombinatorState {
    fn get_combinator_state(&self) -> &CombinatorState {
        &self.state
    }

    fn get_combinator_state_mut(&mut self) -> &mut CombinatorState {
        &mut self.state
    }
}

impl GetCombinatorState for CombinatorState {
    fn get_combinator_state(&self) -> &CombinatorState {
        self
    }

    fn get_combinator_state_mut(&mut self) -> &mut CombinatorState {
        self
    }
}

fn process<C>(combinator: &Combinator, c: Option<char>, its: &mut Vec<C>, signal_id: &mut usize) -> ParserIterationResult
where C: GetCombinatorState {
    if its.len() > 100 {
        // Warn if there are too many states
        eprintln!("Warning: there are {} states (process)", its.len());
    }
    let mut final_result = ParserIterationResult::new(U8Set::none(), None, Default::default());
    its.retain_mut(|it| {
        let result = combinator.next_state(it.get_combinator_state_mut(), c, signal_id);
        let is_empty = result.u8set().is_empty();
        final_result.merge_assign(result);
        !is_empty
    });
    final_result
}

fn seq2_helper(
    b: &Combinator,
    a_result: &mut ParserIterationResult,
    b_result: ParserIterationResult,
    b_its: &mut Vec<CombinatorState>,
    signal_id: &mut usize,
) {
    if b_its.len() > 100 {
        // Warn if there are too many states
        eprintln!("Warning: there are {} states (seq2_helper)", b_its.len());
    }
    if a_result.id_complete.is_some() {
        let mut b_it = b.initial_state(a_result.node.as_ref(), signal_id);
        let b_result = b.next_state(&mut b_it, None, signal_id);
        b_its.push(b_it);
        a_result.forward_assign(b_result);
    }
    a_result.merge_assign(b_result);
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

fn signal_wrap(signal_type_id: usize, a: Combinator) -> Combinator {
    Combinator::SignalWrap(SignalAtom::Start(signal_type_id), SignalAtom::End(signal_type_id), Box::new(a))
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
    signal_id: usize,
}

impl ActiveCombinator {
    fn new(combinator: Combinator) -> Self {
        let mut signal_id = 0;
        let state = combinator.initial_state(None, &mut signal_id);
        Self {
            combinator,
            state,
            signal_id,
        }
    }

    fn send(&mut self, c: Option<char>) -> ParserIterationResult {
        self.combinator.next_state(&mut self.state, c, &mut self.signal_id)
    }
}

fn simplify_combinator(combinator: Combinator, seen_refs: &mut HashSet<*const Option<Combinator>>) -> Combinator {
    match combinator {
        Combinator::Choice(choices) => {
            // Simplify each choice
            let choices: Vec<_> = choices.into_iter().map(|choice| simplify_combinator(choice.clone(), seen_refs)).collect();

            // Combine any EatU8Matching combinators
            // Expand any choice combinators
            let mut eat_u8s = U8Set::none();
            let mut non_eat_u8s = Vec::new();
            for choice in &choices {
                match choice {
                    Combinator::EatU8Matching(u8set) => {
                        eat_u8s |= u8set.clone();
                    }
                    Combinator::Choice(choices) => {
                        for choice in choices.into_iter() {
                            non_eat_u8s.push(choice.clone());
                        }
                    }
                    _ => {
                        non_eat_u8s.push(choice.clone());
                    }
                }
            }
            let mut new_choices = Vec::with_capacity(non_eat_u8s.len() + 1);
            if !eat_u8s.is_empty() {
                new_choices.push(Combinator::EatU8Matching(eat_u8s));
            }
            new_choices.extend(non_eat_u8s);
            Combinator::Choice(new_choices.into())
        }
        Combinator::Seq(seq) => {
            // Simplify each sequent
            let seq: Vec<_> = seq.into_iter().map(|sequent| simplify_combinator(sequent.clone(), seen_refs)).collect();

            // Expand any sequence combinators
            let mut eat_u8s = U8Set::none();
            let mut non_eat_u8s = Vec::new();
            for sequent in &seq {
                match sequent {
                    Combinator::Seq(seq) => {
                        for sequent in seq.into_iter() {
                            non_eat_u8s.push(sequent.clone());
                        }
                    }
                    _ => {
                        non_eat_u8s.push(sequent.clone());
                    }
                }
            }
            let mut new_seq = Vec::with_capacity(non_eat_u8s.len() + 1);
            if !eat_u8s.is_empty() {
                new_seq.push(Combinator::EatU8Matching(eat_u8s));
            }
            new_seq.extend(non_eat_u8s);
            Combinator::Seq(new_seq.into())
        }
        Combinator::Repeat1(inner) => {
            Combinator::Repeat1(Box::new(simplify_combinator(*inner, seen_refs)))
        }
        Combinator::ForwardRef(ref inner) => {
            if seen_refs.contains(&(inner.as_ptr() as *const Option<Combinator>)) {
                return combinator;
            }
            seen_refs.insert(inner.as_ptr() as *const Option<Combinator>);
            Combinator::ForwardRef(Rc::new(std::cell::RefCell::new(
                inner.borrow().as_ref().map(|c| simplify_combinator(c.clone(), seen_refs))
            )))
        }
        // Other combinator types remain unchanged
        _ => combinator,
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use super::*;

    // Test cases remain the same, just update the combinator creation syntax
    #[test]
    fn test_eat_u8() {
        let mut it = ActiveCombinator::new(eat_u8('a').clone());
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("a"));
        let result = it.send(Some('a'));
        assert_matches!(result, ParserIterationResult { ref u8set, id_complete: Some(_), .. } if u8set.is_empty());
    }

    #[test]
    fn test_eat_string() {
        let mut it = ActiveCombinator::new(eat_string("abc").clone());
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("a"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("b"));
        let result2 = it.send(Some('b'));
        assert_matches!(result2, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("c"));
        let result3 = it.send(Some('c'));
        assert_matches!(result3, ParserIterationResult { ref u8set, id_complete: Some(_), .. } if u8set.is_empty());
    }

    #[test]
    fn test_seq() {
        let mut it = ActiveCombinator::new(seq!(eat_u8('a'), eat_u8('b')).clone());
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("a"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("b"));
        let result2 = it.send(Some('b'));
        assert_matches!(result2, ParserIterationResult { ref u8set, id_complete: Some(_), .. } if u8set.is_empty());
    }

    #[test]
    fn test_repeat1() {
        let mut it = ActiveCombinator::new(repeat1(eat_u8('a')));
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("a"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, id_complete: Some(_), .. } if u8set == &U8Set::from_chars("a"));
        let result2 = it.send(Some('a'));
        assert_matches!(result2, ParserIterationResult { ref u8set, id_complete: Some(_), .. } if u8set == &U8Set::from_chars("a"));
    }

    #[test]
    fn test_choice() {
        let mut it = ActiveCombinator::new(choice!(eat_u8('a'), eat_u8('b')));
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("ab"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, id_complete: Some(_), .. } if u8set.is_empty());

        let mut it = ActiveCombinator::new(choice!(eat_u8('a'), eat_u8('b')));
        it.send(None);
        let result2 = it.send(Some('b'));
        assert_matches!(result2, ParserIterationResult { ref u8set, id_complete: Some(_), .. } if u8set.is_empty());
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
        assert_matches!(result0, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("a"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("bc"));
        let result2 = it.send(Some('b'));
        assert_matches!(result2, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("c"));
        let result3 = it.send(Some('c'));
        assert_matches!(result3, ParserIterationResult { ref u8set, id_complete: Some(_), .. } if u8set.is_empty());
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
        assert_matches!(result0, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("[a"));
        let result1 = it.send(Some('['));
        assert_matches!(result1, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("[a"));
        let result2 = it.send(Some('a'));
        assert_matches!(result2, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("]"));
        let result3 = it.send(Some(']'));
        assert_matches!(result3, ParserIterationResult { ref u8set, id_complete: Some(_), .. } if u8set.is_empty());
    }

    #[test]
    fn test_signal_wrap()
    {
        let signal_type_id = 0;
        let mut it = ActiveCombinator::new(seq!(
            eat_u8('a'),
            signal_wrap(signal_type_id, eat_string("bcd")),
            eat_u8('e')
        ));
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("a"));
        assert!(result0.signals2.is_empty());
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("b"));
        assert!(!result1.signals2.is_empty());
        // There should be a SignalAtom::Start(signal_type_id) signal
        assert!(result1.signals2.signals.iter().any(|(_, (_, signal_atom))| matches!(signal_atom, SignalAtom::Start(signal_type_id))));
        let result2 = it.send(Some('b'));
        assert_matches!(result2, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("c"));
        assert!(result2.signals2.is_empty());
        let result3 = it.send(Some('c'));
        assert_matches!(result3, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("d"));
        assert!(result3.signals2.is_empty());
        let result4 = it.send(Some('d'));
        assert_matches!(result4, ParserIterationResult { ref u8set, id_complete: None, .. } if u8set == &U8Set::from_chars("e"));
        assert!(!result4.signals2.is_empty());
        // There should be a SignalAtom::End(signal_type_id) signal
        assert!(result4.signals2.signals.iter().any(|(_, (_, signal_atom))| matches!(signal_atom, SignalAtom::End(signal_type_id))));
        let result5 = it.send(Some('e'));
        assert_matches!(result5, ParserIterationResult { ref u8set, id_complete: Some(_), .. } if u8set.is_empty());
        assert!(result5.signals2.is_empty());
    }
}

#[cfg(test)]
mod json_parser {
    use super::*;

    #[test]
    fn test_json_parser() {
        // Helper combinators for JSON parsing
        let whitespace = repeat(choice!(eat_u8(' '), eat_u8('\t'), eat_u8('\n'), eat_u8('\r')));
        let digit = eat_u8_range('0', '9');
        let digits = repeat(digit);
        let integer = seq!(opt(choice!(eat_u8('-'), eat_u8('+'))), digits);
        let fraction = seq!(eat_u8('.'), digits);
        let exponent = seq!(choice!(eat_u8('e'), eat_u8('E')), seq!(choice!(eat_u8('+'), eat_u8('-')), digits));
        let number = seq!(integer, opt(fraction), opt(exponent));

        let string_char = eat_u8_range_complement('"', '"');
        let string = seq!(eat_u8('"'), repeat(string_char), eat_u8('"'));

        let mut json_value = forward_ref();

        let json_array = seq!(
            eat_u8('['),
            whitespace,
            opt(seq!(
                json_value,
                repeat(seq!(whitespace, eat_u8(','), whitespace, json_value)),
                whitespace,
            )),
            eat_u8(']'),
        );

        let key_value_pair = seq!(string, whitespace, eat_u8(':'), whitespace, json_value);

        let json_object = seq!(
            eat_u8('{'),
            whitespace,
            opt(seq!(
                key_value_pair,
                whitespace,
                repeat(seq!(eat_u8(','), whitespace, key_value_pair)),
                whitespace,
            )),
            eat_u8('}'),
        );

        json_value.set(
            choice!(
                string, number,
                eat_string("true"), eat_string("false"),
                eat_string("null"), json_array, json_object,
            )
        );

        // Test cases
        let json_parser = seq!(whitespace, json_value);
        let json_parser = simplify_combinator(json_parser, &mut HashSet::new());

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
            for char in json_string.chars() {
                assert!(result.u8set().contains(char as u8), "Expected {} to be in {:?}", char, result.u8set());
                result = it.send(Some(char));
            }
            result.is_complete()
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
            "GeneratedCSV_mini.json",
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
        for i in vec![1_000, 10_000, 100_000] {
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