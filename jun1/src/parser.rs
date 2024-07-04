use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};
use std::rc::Rc;

use crate::parse_iteration_result::{Frame, FrameStack, ParserIterationResult};
use crate::u8set::U8Set;

// Trait for all combinators
trait Combinator: Clone + PartialEq + Eq + Hash + Debug {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState>;
    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult;
}

// Trait for all combinator states
trait CombinatorState: Debug {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

// Macro to simplify combinator state implementation
macro_rules! impl_combinator_state {
    ($state_type:ty) => {
        impl CombinatorState for $state_type {
            fn as_any(&self) -> &dyn std::any::Any { self }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
        }
    };
}

// Call Combinator
#[derive(Clone)]
struct Call<F: Fn() -> Box<dyn Combinator> + 'static>(Rc<F>);

impl<F: Fn() -> Box<dyn Combinator> + 'static> PartialEq for Call<F> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<F: Fn() -> Box<dyn Combinator> + 'static> Eq for Call<F> {}

impl<F: Fn() -> Box<dyn Combinator> + 'static> Hash for Call<F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the function pointer, not the Rc
        let ptr = Rc::as_ptr(&self.0);
        ptr.hash(state);
    }
}

impl<F: Fn() -> Box<dyn Combinator> + 'static> Debug for Call<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Call")
    }
}

#[derive(Clone, Debug)]
struct CallState {
    inner_state: Box<dyn CombinatorState>,
}

impl_combinator_state!(CallState);

impl<F: Fn() -> Box<dyn Combinator> + 'static> Combinator for Call<F> {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let inner_combinator = (self.0)();
        Box::new(CallState {
            inner_state: inner_combinator.initial_state(signal_id, frame_stack),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<CallState>().unwrap();
        (self.0)().next_state(&mut *state.inner_state, c, signal_id)
    }
}

// Choice Combinator
#[derive(Clone)]
struct Choice(Rc<Vec<Box<dyn Combinator>>>);

impl PartialEq for Choice {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Choice {}

impl Hash for Choice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Debug for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Choice({:?})", self.0)
    }
}

#[derive(Clone, Debug)]
struct ChoiceState {
    inner_states: Vec<Vec<Box<dyn CombinatorState>>>,
}

impl_combinator_state!(ChoiceState);

impl Combinator for Choice {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(ChoiceState {
            inner_states: self.0.iter().map(|c| vec![c.initial_state(signal_id, frame_stack.clone())]).collect(),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<ChoiceState>().unwrap();
        let mut final_result = ParserIterationResult::new(U8Set::none(), false, FrameStack::default());
        for (combinator, its) in self.0.iter().zip(state.inner_states.iter_mut()) {
            final_result.merge_assign(process(combinator, c, its, signal_id));
        }
        final_result
    }
}

// EatString Combinator
#[derive(Clone)]
struct EatString(&'static str);

impl PartialEq for EatString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for EatString {}

impl Hash for EatString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Debug for EatString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EatString({:?})", self.0)
    }
}

#[derive(Clone, Debug)]
struct EatStringState {
    index: usize,
    frame_stack: FrameStack,
}

impl_combinator_state!(EatStringState);

impl Combinator for EatString {
    fn initial_state(&self, _signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(EatStringState { index: 0, frame_stack })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, _c: Option<char>, _signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<EatStringState>().unwrap();
        if state.index > self.0.len() {
            return ParserIterationResult::new(U8Set::none(), false, state.frame_stack.clone());
        }
        if state.index == self.0.len() {
            return ParserIterationResult::new(U8Set::none(), true, state.frame_stack.clone());
        }
        let u8set = U8Set::from_chars(&self.0[state.index..=state.index]);
        state.index += 1;
        ParserIterationResult::new(u8set, false, state.frame_stack.clone())
    }
}

// EatU8Matching Combinator
#[derive(Clone)]
struct EatU8Matching(U8Set);

impl PartialEq for EatU8Matching {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for EatU8Matching {}

impl Hash for EatU8Matching {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Debug for EatU8Matching {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EatU8Matching({:?})", self.0)
    }
}

#[derive(Clone, Debug)]
struct EatU8MatchingState {
    state: u8,
    frame_stack: FrameStack,
}

impl_combinator_state!(EatU8MatchingState);

impl Combinator for EatU8Matching {
    fn initial_state(&self, _signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(EatU8MatchingState { state: 0, frame_stack })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, _signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<EatU8MatchingState>().unwrap();
        match state.state {
            0 => {
                state.state = 1;
                ParserIterationResult::new(self.0.clone(), false, state.frame_stack.clone())
            }
            1 => {
                state.state = 2;
                let is_complete = c.map(|c| self.0.contains(c as u8)).unwrap_or(false);
                ParserIterationResult::new(U8Set::none(), is_complete, state.frame_stack.clone())
            }
            _ => panic!("EatU8Matching: state out of bounds"),
        }
    }
}

// Eps Combinator
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Eps;

#[derive(Clone, Debug)]
struct EpsState {
    frame_stack: FrameStack,
}

impl_combinator_state!(EpsState);

impl Combinator for Eps {
    fn initial_state(&self, _signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(EpsState { frame_stack })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, _c: Option<char>, _signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<EpsState>().unwrap();
        ParserIterationResult::new(U8Set::none(), true, state.frame_stack.clone())
    }
}

// ForwardRef Combinator
#[derive(Clone)]
struct ForwardRef(Rc<RefCell<Option<Box<dyn Combinator>>>>);

impl PartialEq for ForwardRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for ForwardRef {}

impl Hash for ForwardRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the RefCell, not the Rc
        let ptr = Rc::as_ptr(&self.0);
        ptr.hash(state);
    }
}

impl Debug for ForwardRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ForwardRef({:?})", self.0)
    }
}

#[derive(Clone, Debug)]
struct ForwardRefState {
    inner_state: Option<Box<dyn CombinatorState>>,
}

impl_combinator_state!(ForwardRefState);

impl Combinator for ForwardRef {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let inner_state = self.0.borrow().as_ref().map(|c| c.initial_state(signal_id, frame_stack));
        Box::new(ForwardRefState { inner_state })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<ForwardRefState>().unwrap();
        match state.inner_state {
            Some(ref mut inner_state) => self.0.borrow().as_ref().unwrap().next_state(inner_state, c, signal_id),
            None => panic!("Forward reference not set before use"),
        }
    }
}

// Repeat1 Combinator
#[derive(Clone)]
struct Repeat1(Box<dyn Combinator>);

impl PartialEq for Repeat1 {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for Repeat1 {}

impl Hash for Repeat1 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Debug for Repeat1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Repeat1({:?})", self.0)
    }
}

#[derive(Clone, Debug)]
struct Repeat1State {
    inner_states: Vec<Box<dyn CombinatorState>>,
}

impl_combinator_state!(Repeat1State);

impl Combinator for Repeat1 {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(Repeat1State {
            inner_states: vec![self.0.initial_state(signal_id, frame_stack)],
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<Repeat1State>().unwrap();
        let mut a_result = process(&self.0, c, &mut state.inner_states, signal_id);
        let b_result = a_result.clone();
        seq2_helper(&self.0, &mut a_result, b_result, &mut state.inner_states, signal_id);
        a_result
    }
}

// Seq Combinator
#[derive(Clone)]
struct Seq(Rc<Vec<Box<dyn Combinator>>>);

impl PartialEq for Seq {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Seq {}

impl Hash for Seq {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Debug for Seq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Seq({:?})", self.0)
    }
}

#[derive(Clone, Debug)]
struct SeqState {
    inner_states: Vec<Vec<Box<dyn CombinatorState>>>,
}

impl_combinator_state!(SeqState);

impl Combinator for Seq {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let mut inner_states = Vec::with_capacity(self.0.len());
        inner_states.push(vec![self.0[0].initial_state(signal_id, frame_stack)]);
        for _ in 1..self.0.len() {
            inner_states.push(Vec::new());
        }
        Box::new(SeqState { inner_states })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<SeqState>().unwrap();
        let mut a_result = process(&self.0[0], c, &mut state.inner_states[0], signal_id);
        for (combinator, its) in self.0.iter().zip(state.inner_states.iter_mut()).skip(1) {
            let b_result = process(combinator, c, its, signal_id);
            seq2_helper(combinator, &mut a_result, b_result, its, signal_id);
        }
        a_result
    }
}

// WithNewFrame Combinator
#[derive(Clone)]
struct WithNewFrame(Box<dyn Combinator>);

impl PartialEq for WithNewFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for WithNewFrame {}

impl Hash for WithNewFrame {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Debug for WithNewFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WithNewFrame({:?})", self.0)
    }
}

#[derive(Clone, Debug)]
struct WithNewFrameState {
    inner_state: Box<dyn CombinatorState>,
}

impl_combinator_state!(WithNewFrameState);

impl Combinator for WithNewFrame {
    fn initial_state(&self, signal_id: &mut usize, mut frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        frame_stack.push_empty_frame();
        Box::new(WithNewFrameState {
            inner_state: self.0.initial_state(signal_id, frame_stack),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<WithNewFrameState>().unwrap();
        let mut result = self.0.next_state(&mut *state.inner_state, c, signal_id);
        result.frame_stack.pop();
        result
    }
}

// WithExistingFrame Combinator
#[derive(Clone)]
struct WithExistingFrame(Frame, Box<dyn Combinator>);

impl PartialEq for WithExistingFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1.eq(&other.1)
    }
}

impl Eq for WithExistingFrame {}

impl Hash for WithExistingFrame {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

impl Debug for WithExistingFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WithExistingFrame({:?}, {:?})", self.0, self.1)
    }
}

#[derive(Clone, Debug)]
struct WithExistingFrameState {
    inner_state: Box<dyn CombinatorState>,
}

impl_combinator_state!(WithExistingFrameState);

impl Combinator for WithExistingFrame {
    fn initial_state(&self, signal_id: &mut usize, mut frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        frame_stack.push_frame(self.0.clone());
        Box::new(WithExistingFrameState {
            inner_state: self.1.initial_state(signal_id, frame_stack),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<WithExistingFrameState>().unwrap();
        let mut result = self.1.next_state(&mut *state.inner_state, c, signal_id);
        result.frame_stack.pop();
        result.frame_stack.push_frame(self.0.clone());
        result
    }
}

// InFrameStack Combinator
#[derive(Clone)]
struct InFrameStack(Box<dyn Combinator>);

impl PartialEq for InFrameStack {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for InFrameStack {}

impl Hash for InFrameStack {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Debug for InFrameStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InFrameStack({:?})", self.0)
    }
}

#[derive(Clone, Debug)]
struct InFrameStackState {
    inner_state: Box<dyn CombinatorState>,
    name: Vec<u8>,
}

impl_combinator_state!(InFrameStackState);

impl Combinator for InFrameStack {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(InFrameStackState {
            inner_state: self.0.initial_state(signal_id, frame_stack),
            name: Vec::new(),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<InFrameStackState>().unwrap();
        if let Some(c) = c {
            state.name.push(c.try_into().unwrap());
        }
        let mut result = self.0.next_state(&mut *state.inner_state, c, signal_id);
        let (u8set, is_complete) = result.frame_stack.next_u8_given_contains(&state.name);
        if result.is_complete && !is_complete {
            result.is_complete = false;
        } else if result.is_complete && is_complete {
            result.frame_stack.filter_contains(&state.name);
        }
        result.u8set &= u8set;
        result
    }
}

// NotInFrameStack Combinator
#[derive(Clone)]
struct NotInFrameStack(Box<dyn Combinator>);

impl PartialEq for NotInFrameStack {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for NotInFrameStack {}

impl Hash for NotInFrameStack {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Debug for NotInFrameStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NotInFrameStack({:?})", self.0)
    }
}

#[derive(Clone, Debug)]
struct NotInFrameStackState {
    inner_state: Box<dyn CombinatorState>,
    name: Vec<u8>,
}

impl_combinator_state!(NotInFrameStackState);

impl Combinator for NotInFrameStack {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(NotInFrameStackState {
            inner_state: self.0.initial_state(signal_id, frame_stack),
            name: Vec::new(),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<NotInFrameStackState>().unwrap();
        let mut result = self.0.next_state(&mut *state.inner_state, c, signal_id);
        let (u8set, is_complete) = result.frame_stack.next_u8_given_excludes(&state.name);
        if result.is_complete && !is_complete {
            result.is_complete = false;
        } else if result.is_complete && is_complete {
            result.frame_stack.filter_excludes(&state.name);
        }
        if let Some(c) = c {
            state.name.push(c.try_into().unwrap());
        }
        result.u8set &= u8set;
        result
    }
}

// AddToFrameStack Combinator
#[derive(Clone)]
struct AddToFrameStack(Box<dyn Combinator>);

impl PartialEq for AddToFrameStack {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for AddToFrameStack {}

impl Hash for AddToFrameStack {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Debug for AddToFrameStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AddToFrameStack({:?})", self.0)
    }
}

#[derive(Clone, Debug)]
struct AddToFrameStackState {
    inner_state: Box<dyn CombinatorState>,
    name: Vec<u8>,
}

impl_combinator_state!(AddToFrameStackState);

impl Combinator for AddToFrameStack {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(AddToFrameStackState {
            inner_state: self.0.initial_state(signal_id, frame_stack),
            name: Vec::new(),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<AddToFrameStackState>().unwrap();
        let mut result = self.0.next_state(&mut *state.inner_state, c, signal_id);
        if let Some(c) = c {
            state.name.push(c.try_into().unwrap());
        }
        if result.is_complete {
            result.frame_stack.push_name(&state.name);
        }
        result
    }
}

// RemoveFromFrameStack Combinator
#[derive(Clone)]
struct RemoveFromFrameStack(Box<dyn Combinator>);

impl PartialEq for RemoveFromFrameStack {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for RemoveFromFrameStack {}

impl Hash for RemoveFromFrameStack {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Debug for RemoveFromFrameStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RemoveFromFrameStack({:?})", self.0)
    }
}

#[derive(Clone, Debug)]
struct RemoveFromFrameStackState {
    inner_state: Box<dyn CombinatorState>,
    name: Vec<u8>,
}

impl_combinator_state!(RemoveFromFrameStackState);

impl Combinator for RemoveFromFrameStack {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(RemoveFromFrameStackState {
            inner_state: self.0.initial_state(signal_id, frame_stack),
            name: Vec::new(),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<RemoveFromFrameStackState>().unwrap();
        let mut result = self.0.next_state(&mut *state.inner_state, c, signal_id);
        if let Some(c) = c {
            state.name.push(c.try_into().unwrap());
        }
        if result.is_complete {
            result.frame_stack.pop_name(&state.name);
        }
        result
    }
}

// Helper functions to create combinators
fn call<F: Fn() -> Box<dyn Combinator> + 'static>(f: F) -> Box<dyn Combinator> {
    Box::new(Call(Rc::new(f)))
}

fn seq(combinators: Vec<Box<dyn Combinator>>) -> Box<dyn Combinator> {
    Box::new(Seq(Rc::new(combinators)))
}

fn repeat1(a: Box<dyn Combinator>) -> Box<dyn Combinator> {
    Box::new(Repeat1(a))
}

fn choice(combinators: Vec<Box<dyn Combinator>>) -> Box<dyn Combinator> {
    Box::new(Choice(Rc::new(combinators)))
}

fn eat_u8_matching<F: Fn(u8) -> bool + 'static>(fn_: F) -> Box<dyn Combinator> {
    Box::new(EatU8Matching(U8Set::from_match_fn(&fn_)))
}

fn eat_u8(value: char) -> Box<dyn Combinator> {
    eat_u8_matching(move |c: u8| c == value as u8)
}

fn eat_u8_range(start: char, end: char) -> Box<dyn Combinator> {
    eat_u8_matching(move |c: u8| (start as u8..=end as u8).contains(&c))
}

fn eat_u8_range_complement(start: char, end: char) -> Box<dyn Combinator> {
    eat_u8_matching(move |c: u8| !(start as u8..=end as u8).contains(&c))
}

fn eat_string(value: &'static str) -> Box<dyn Combinator> {
    Box::new(EatString(value))
}

fn eps() -> Box<dyn Combinator> {
    Box::new(Eps)
}

fn opt(a: Box<dyn Combinator>) -> Box<dyn Combinator> {
    choice(vec![a, eps()])
}

fn repeat(a: Box<dyn Combinator>) -> Box<dyn Combinator> {
    opt(repeat1(a))
}

fn forward_ref() -> Box<dyn Combinator> {
    Box::new(ForwardRef(Rc::new(RefCell::new(None))))
}

fn in_frame_stack(a: Box<dyn Combinator>) -> Box<dyn Combinator> {
    Box::new(InFrameStack(a))
}

fn not_in_frame_stack(a: Box<dyn Combinator>) -> Box<dyn Combinator> {
    Box::new(NotInFrameStack(a))
}

fn add_to_frame_stack(a: Box<dyn Combinator>) -> Box<dyn Combinator> {
    Box::new(AddToFrameStack(a))
}

fn remove_from_frame_stack(a: Box<dyn Combinator>) -> Box<dyn Combinator> {
    Box::new(RemoveFromFrameStack(a))
}

// Macros for easier combinator creation
macro_rules! seq {
    ($($a:expr),+ $(,)?) => {
        seq(vec![$($a),+])
    }
}

macro_rules! choice {
    ($($a:expr),+ $(,)?) => {
        choice(vec![$($a),+])
    }
}

// State iteration and processing functions remain similar, adapted for trait objects
// ...

// ActiveCombinator struct
#[derive(Clone)]
struct ActiveCombinator {
    combinator: Box<dyn Combinator>,
    state: Box<dyn CombinatorState>,
    signal_id: usize,
}

impl ActiveCombinator {
    fn new(combinator: Box<dyn Combinator>) -> Self {
        let mut signal_id = 0;
        let state = combinator.initial_state(&mut signal_id, FrameStack::default());
        Self {
            combinator,
            state,
            signal_id,
        }
    }

    fn new_with_names(combinator: Box<dyn Combinator>, names: Vec<String>) -> Self {
        let mut signal_id = 0;
        let mut frame_stack = FrameStack::default();
        for name in names {
            frame_stack.push_name(name.as_bytes());
        }
        let state = combinator.initial_state(&mut signal_id, frame_stack);
        Self {
            combinator,
            state,
            signal_id,
        }
    }

    fn send(&mut self, c: Option<char>) -> ParserIterationResult {
        self.combinator.next_state(&mut *self.state, c, &mut self.signal_id)
    }
}
