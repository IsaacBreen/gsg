use std::cell::RefCell;
use std::rc::Rc;

use crate::parse_iteration_result::{Frame, FrameStack, ParserIterationResult};
use crate::u8set::U8Set;

trait Combinator {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState>;
    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult;
}

trait CombinatorState {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

struct Call<F: Fn() -> Rc<dyn Combinator> + 'static + ?Sized>(Rc<F>);
struct Choice(Rc<[Rc<dyn Combinator>]>);
struct EatString(&'static str);
struct EatU8Matching(U8Set);
struct Eps;
struct ForwardRef(Rc<RefCell<Option<Rc<dyn Combinator>>>>);
struct Repeat1(Rc<dyn Combinator>);
struct Seq(Rc<[Rc<dyn Combinator>]>);
struct WithNewFrame(Rc<dyn Combinator>);
struct WithExistingFrame(Frame, Rc<dyn Combinator>);
struct InFrameStack(Rc<dyn Combinator>);
struct NotInFrameStack(Rc<dyn Combinator>);
struct AddToFrameStack(Rc<dyn Combinator>);
struct RemoveFromFrameStack(Rc<dyn Combinator>);

// Implement Combinator trait for each struct...

impl<F> Combinator for Call<F>
where
    F: Fn() -> Rc<dyn Combinator> + 'static + ?Sized,
{
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let inner_combinator = (self.0)();
        Box::new(CallState {
            inner_state: Some(inner_combinator.initial_state(signal_id, frame_stack)),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        if let Some(state) = state.as_any_mut().downcast_mut::<CallState>() {
            let inner_combinator = (self.0)();
            let inner_state = state.inner_state.as_mut().unwrap();
            inner_combinator.next_state(inner_state.as_mut(), c, signal_id)
        } else {
            panic!("Invalid state type");
        }
    }
}

// Implement for other combinators similarly...
impl Combinator for Choice {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(ChoiceState {
            its: self
                .0
                .iter()
                .map(|combinator| vec![combinator.initial_state(signal_id, frame_stack.clone())])
                .collect(),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        if let Some(state) = state.as_any_mut().downcast_mut::<ChoiceState>() {
            let mut final_result =
                ParserIterationResult::new(U8Set::none(), false, FrameStack::default());
            for (combinator, its) in self.0.iter().zip(state.its.iter_mut()) {
                final_result.merge_assign(process(combinator, c, its, signal_id));
            }
            final_result
        } else {
            panic!("Invalid state type");
        }
    }
}

impl Combinator for EatString {
    fn initial_state(&self, _signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(EatStringState {
            index: 0,
            frame_stack,
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, _c: Option<char>, _signal_id: &mut usize) -> ParserIterationResult {
        if let Some(state) = state.as_any_mut().downcast_mut::<EatStringState>() {
            if state.index > self.0.len() {
                return ParserIterationResult::new(U8Set::none(), false, state.frame_stack.clone());
            }
            if state.index == self.0.len() {
                let mut result =
                    ParserIterationResult::new(U8Set::none(), true, state.frame_stack.clone());
                return result;
            }
            let u8set = U8Set::from_chars(&self.0[state.index..=state.index]);
            state.index += 1;
            ParserIterationResult::new(u8set, false, state.frame_stack.clone())
        } else {
            panic!("Invalid state type");
        }
    }
}

impl Combinator for EatU8Matching {
    fn initial_state(&self, _signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(EatU8MatchingState {
            state: 0,
            frame_stack,
        })
    }

    fn next_state(
        &self,
        state: &mut dyn CombinatorState,
        c: Option<char>,
        _signal_id: &mut usize,
    ) -> ParserIterationResult {
        if let Some(state) = state.as_any_mut().downcast_mut::<EatU8MatchingState>() {
            match state.state {
                0 => {
                    state.state = 1;
                    ParserIterationResult::new(self.0.clone(), false, state.frame_stack.clone())
                }
                1 => {
                    state.state = 2;
                    let is_complete = c.map(|c| self.0.contains(c as u8)).unwrap_or(false);
                    let mut result = ParserIterationResult::new(
                        U8Set::none(),
                        is_complete,
                        state.frame_stack.clone(),
                    );
                    result
                }
                _ => panic!("EatU8Matching: state out of bounds"),
            }
        } else {
            panic!("Invalid state type");
        }
    }
}

impl Combinator for Eps {
    fn initial_state(&self, _signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(EpsState { frame_stack })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, _c: Option<char>, _signal_id: &mut usize) -> ParserIterationResult {
        if let Some(state) = state.as_any_mut().downcast_mut::<EpsState>() {
            let mut result = ParserIterationResult::new(U8Set::none(), true, state.frame_stack.clone());
            result
        } else {
            panic!("Invalid state type");
        }
    }
}

impl Combinator for ForwardRef {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let inner_state = match self.0.borrow().as_ref() {
            Some(c) => Some(c.initial_state(signal_id, frame_stack)),
            None => panic!("ForwardRef not set"),
        };
        Box::new(ForwardRefState { inner_state })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        if let Some(state) = state.as_any_mut().downcast_mut::<ForwardRefState>() {
            match state.inner_state.as_mut() {
                Some(inner_state) => match self.0.borrow().as_ref() {
                    Some(combinator) => combinator.next_state(inner_state.as_mut(), c, signal_id),
                    None => panic!("Forward reference not set before use"),
                },
                None => panic!("Forward reference not set before use"),
            }
        } else {
            panic!("Invalid state type");
        }
    }
}

impl Combinator for Repeat1 {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(Repeat1State {
            a_its: vec![self.0.initial_state(signal_id, frame_stack)],
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        if let Some(state) = state.as_any_mut().downcast_mut::<Repeat1State>() {
            let mut a_result = process(&self.0, c, &mut state.a_its, signal_id);
            let b_result = a_result.clone();
            seq2_helper(&self.0, &mut a_result, b_result, &mut state.a_its, signal_id);
            a_result
        } else {
            panic!("Invalid state type");
        }
    }
}

impl Combinator for Seq {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let mut its = Vec::with_capacity(self.0.len());
        its.push(vec![self.0[0].initial_state(signal_id, frame_stack)]);
        for _ in 1..self.0.len() {
            its.push(Vec::new());
        }
        Box::new(SeqState { its })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        if let Some(state) = state.as_any_mut().downcast_mut::<SeqState>() {
            let mut a_result = process(&self.0[0], c, &mut state.its[0], signal_id);
            for (combinator, its) in self.0.iter().zip(state.its.iter_mut()).skip(1) {
                let b_result = process(combinator, c, its, signal_id);
                seq2_helper(combinator, &mut a_result, b_result, its, signal_id);
            }
            a_result
        } else {
            panic!("Invalid state type");
        }
    }
}

impl Combinator for WithNewFrame {
    fn initial_state(&self, signal_id: &mut usize, mut frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        frame_stack.push_empty_frame();
        let a_state = self.0.initial_state(signal_id, frame_stack);
        Box::new(WithNewFrameState { a_state })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        if let Some(state) = state.as_any_mut().downcast_mut::<WithNewFrameState>() {
            let mut result = self.0.next_state(state.a_state.as_mut(), c, signal_id);
            result.frame_stack.pop();
            result
        } else {
            panic!("Invalid state type");
        }
    }
}

impl Combinator for WithExistingFrame {
    fn initial_state(&self, signal_id: &mut usize, mut frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        frame_stack.push_frame(self.0.clone());
        let a_state = self.1.initial_state(signal_id, frame_stack);
        Box::new(WithExistingFrameState { a_state })
    }

    fn next_state(
        &self,
        state: &mut dyn CombinatorState,
        c: Option<char>,
        signal_id: &mut usize,
    ) -> ParserIterationResult {
        if let Some(state) = state.as_any_mut().downcast_mut::<WithExistingFrameState>() {
            let mut result = self.1.next_state(state.a_state.as_mut(), c, signal_id);
            result.frame_stack.pop();
            result.frame_stack.push_frame(self.0.clone());
            result
        } else {
            panic!("Invalid state type");
        }
    }
}

impl Combinator for InFrameStack {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let a_state = self.0.initial_state(signal_id, frame_stack);
        Box::new(InFrameStackState {
            a_state,
            name: Vec::new(),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        if let Some(state) = state.as_any_mut().downcast_mut::<InFrameStackState>() {
            if let Some(c) = c {
                state.name.push(c as u8);
            }
            let mut result = self.0.next_state(state.a_state.as_mut(), c, signal_id);
            let (u8set, is_complete) =
                result.frame_stack.next_u8_given_contains(state.name.as_slice());
            if result.is_complete && !is_complete {
                result.is_complete = false;
            } else if result.is_complete && is_complete {
                result
                    .frame_stack
                    .filter_contains(state.name.as_slice());
            }
            result.u8set &= u8set;
            result
        } else {
            panic!("Invalid state type");
        }
    }
}

impl Combinator for NotInFrameStack {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let a_state = self.0.initial_state(signal_id, frame_stack);
        Box::new(NotInFrameStackState {
            a_state,
            name: Vec::new(),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        if let Some(state) = state.as_any_mut().downcast_mut::<NotInFrameStackState>() {
            if let Some(c) = c {
                state.name.push(c as u8);
            }
            let mut result = self.0.next_state(state.a_state.as_mut(), c, signal_id);
            let (u8set, is_complete) =
                result.frame_stack.next_u8_given_excludes(state.name.as_slice());
            if result.is_complete && !is_complete {
                result.is_complete = false;
            } else if result.is_complete && is_complete {
                result
                    .frame_stack
                    .filter_excludes(state.name.as_slice());
            }
            result.u8set &= u8set;
            result
        } else {
            panic!("Invalid state type");
        }
    }
}

impl Combinator for AddToFrameStack {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let a_state = self.0.initial_state(signal_id, frame_stack);
        Box::new(AddToFrameStackState {
            a_state,
            name: Vec::new(),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        if let Some(state) = state.as_any_mut().downcast_mut::<AddToFrameStackState>() {
            if let Some(c) = c {
                state.name.push(c as u8);
            }
            let mut result = self.0.next_state(state.a_state.as_mut(), c, signal_id);
            if result.is_complete {
                result.frame_stack.push_name(state.name.as_slice());
            }
            result
        } else {
            panic!("Invalid state type");
        }
    }
}

impl Combinator for RemoveFromFrameStack {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let a_state = self.0.initial_state(signal_id, frame_stack);
        Box::new(RemoveFromFrameStackState {
            a_state,
            name: Vec::new(),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        if let Some(state) = state.as_any_mut().downcast_mut::<RemoveFromFrameStackState>() {
            if let Some(c) = c {
                state.name.push(c as u8);
            }
            let mut result = self.0.next_state(state.a_state.as_mut(), c, signal_id);
            if result.is_complete {
                result.frame_stack.pop_name(state.name.as_slice());
            }
            result
        } else {
            panic!("Invalid state type");
        }
    }
}

struct CallState {
    inner_state: Option<Box<dyn CombinatorState>>,
}
impl CombinatorState for CallState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

struct ChoiceState {
    its: Vec<Vec<Box<dyn CombinatorState>>>,
}
impl CombinatorState for ChoiceState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
struct EatStringState {
    index: usize,
    frame_stack: FrameStack,
}
impl CombinatorState for EatStringState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
struct EatU8MatchingState {
    state: u8,
    frame_stack: FrameStack,
}
impl CombinatorState for EatU8MatchingState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
struct EpsState {
    frame_stack: FrameStack,
}
impl CombinatorState for EpsState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
struct ForwardRefState {
    inner_state: Option<Box<dyn CombinatorState>>,
}
impl CombinatorState for ForwardRefState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
struct Repeat1State {
    a_its: Vec<Box<dyn CombinatorState>>,
}
impl CombinatorState for Repeat1State {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
struct SeqState {
    its: Vec<Vec<Box<dyn CombinatorState>>>,
}
impl CombinatorState for SeqState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
struct WithNewFrameState {
    a_state: Box<dyn CombinatorState>,
}
impl CombinatorState for WithNewFrameState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
struct WithExistingFrameState {
    a_state: Box<dyn CombinatorState>,
}
impl CombinatorState for WithExistingFrameState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
struct InFrameStackState {
    a_state: Box<dyn CombinatorState>,
    name: Vec<u8>,
}
impl CombinatorState for InFrameStackState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
struct NotInFrameStackState {
    a_state: Box<dyn CombinatorState>,
    name: Vec<u8>,
}
impl CombinatorState for NotInFrameStackState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
struct AddToFrameStackState {
    a_state: Box<dyn CombinatorState>,
    name: Vec<u8>,
}
impl CombinatorState for AddToFrameStackState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
struct RemoveFromFrameStackState {
    a_state: Box<dyn CombinatorState>,
    name: Vec<u8>,
}
impl CombinatorState for RemoveFromFrameStackState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Combinator for Rc<dyn Combinator> {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        (**self).initial_state(signal_id, frame_stack)
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        (**self).next_state(state, c, signal_id)
    }
}

// Helper functions
fn seq<I>(combinators: I) -> Rc<dyn Combinator>
where
    I: IntoIterator<Item = Rc<dyn Combinator>>,
{
    Rc::new(Seq(Rc::from(combinators.into_iter().collect::<Vec<_>>())))
}

fn repeat1(a: Rc<dyn Combinator>) -> Rc<dyn Combinator> {
    Rc::new(Repeat1(a))
}

fn choice<I>(combinators: I) -> Rc<dyn Combinator>
where
    I: IntoIterator<Item = Rc<dyn Combinator>>,
{
    Rc::new(Choice(Rc::from(combinators.into_iter().collect::<Vec<_>>())))
}

fn eat_u8_matching(u8set: U8Set) -> Rc<dyn Combinator> {
    Rc::new(EatU8Matching(u8set))
}

fn eat_u8(value: char) -> Rc<dyn Combinator> {
    eat_u8_matching(U8Set::from_char(value))
}

fn eat_u8_range(start: char, end: char) -> Rc<dyn Combinator> {
    eat_u8_matching(U8Set::from_range(start as u8, end as u8))
}

fn eat_string(value: &'static str) -> Rc<dyn Combinator> {
    Rc::new(EatString(value))
}

fn eps() -> Rc<dyn Combinator> {
    Rc::new(Eps)
}

fn opt(a: Rc<dyn Combinator>) -> Rc<dyn Combinator> {
    choice(vec![a, eps()])
}

fn repeat(a: Rc<dyn Combinator>) -> Rc<dyn Combinator> {
    opt(repeat1(a))
}

fn call<F>(f: &'static F) -> Rc<dyn Combinator>
where
    F: Fn() -> Rc<dyn Combinator> + 'static + ?Sized,
{
    Rc::new(Call(Rc::new(f)))
}

fn forward_ref() -> Rc<dyn Combinator> {
    Rc::new(ForwardRef(Rc::new(RefCell::new(None))))
}

fn in_frame_stack(a: Rc<dyn Combinator>) -> Rc<dyn Combinator> {
    Rc::new(InFrameStack(a))
}

fn not_in_frame_stack(a: Rc<dyn Combinator>) -> Rc<dyn Combinator> {
    Rc::new(NotInFrameStack(a))
}

fn add_to_frame_stack(a: Rc<dyn Combinator>) -> Rc<dyn Combinator> {
    Rc::new(AddToFrameStack(a))
}

fn remove_from_frame_stack(a: Rc<dyn Combinator>) -> Rc<dyn Combinator> {
    Rc::new(RemoveFromFrameStack(a))
}

struct ActiveCombinator {
    combinator: Rc<dyn Combinator>,
    state: Box<dyn CombinatorState>,
    signal_id: usize,
}

impl ActiveCombinator {
    fn new(combinator: Rc<dyn Combinator>) -> Self {
        let mut signal_id = 0;
        let state = combinator.initial_state(&mut signal_id, FrameStack::default());
        Self {
            combinator,
            state,
            signal_id,
        }
    }

    fn new_with_names(combinator: Rc<dyn Combinator>, names: Vec<String>) -> Self {
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
        self.combinator
            .next_state(&mut *self.state, c, &mut self.signal_id)
    }
}

// Implement macros
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

fn eat_u8_range_complement(start: char, end: char) -> Rc<dyn Combinator> {
    choice!(
        eat_u8_range(0 as char, start as char),
        eat_u8_range(end, 255 as char),
    )
}

fn process(
    combinator: &Rc<dyn Combinator>,
    c: Option<char>,
    its: &mut Vec<Box<dyn CombinatorState>>,
    signal_id: &mut usize,
) -> ParserIterationResult {
    if its.len() > 100 {
        // Warn if there are too many states
        eprintln!("Warning: there are {} states (process)", its.len());
    }
    let mut final_result = ParserIterationResult::new(U8Set::none(), false, FrameStack::default());
    its.retain_mut(|it| {
        let result = combinator.next_state(it.as_mut(), c, signal_id);
        let is_empty = result.u8set().is_empty();
        final_result.merge_assign(result);
        !is_empty
    });
    final_result
}

fn seq2_helper(
    b: &Rc<dyn Combinator>,
    a_result: &mut ParserIterationResult,
    b_result: ParserIterationResult,
    b_its: &mut Vec<Box<dyn CombinatorState>>,
    signal_id: &mut usize,
) {
    if b_its.len() > 100 {
        // Warn if there are too many states
        eprintln!("Warning: there are {} states (seq2_helper)", b_its.len());
    }
    if a_result.is_complete {
        let mut b_it = b.initial_state(signal_id, a_result.frame_stack.clone());
        let b_result = b.next_state(b_it.as_mut(), None, signal_id);
        b_its.push(b_it);
        a_result.forward_assign(b_result);
    }
    a_result.merge_assign(b_result);
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    // Test cases remain the same, just update the combinator creation syntax
    #[test]
    fn test_eat_u8() {
        let mut it = ActiveCombinator::new(eat_u8('a'));
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("a"));
        let result = it.send(Some('a'));
        assert_matches!(result, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set.is_empty());
    }

    #[test]
    fn test_eat_string() {
        let mut it = ActiveCombinator::new(eat_string("abc"));
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("a"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("b"));
        let result2 = it.send(Some('b'));
        assert_matches!(result2, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("c"));
        let result3 = it.send(Some('c'));
        assert_matches!(result3, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set.is_empty());
    }

    #[test]
    fn test_seq() {
        let mut it = ActiveCombinator::new(seq!(eat_u8('a'), eat_u8('b')));
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("a"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("b"));
        let result2 = it.send(Some('b'));
        assert_matches!(result2, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set.is_empty());
    }

    #[test]
    fn test_repeat1() {
        let mut it = ActiveCombinator::new(repeat1(eat_u8('a')));
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("a"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set == &U8Set::from_chars("a"));
        let result2 = it.send(Some('a'));
        assert_matches!(result2, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set == &U8Set::from_chars("a"));
    }

    #[test]
    fn test_choice() {
        let mut it = ActiveCombinator::new(choice!(eat_u8('a'), eat_u8('b')));
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("ab"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set.is_empty());

        let mut it = ActiveCombinator::new(choice!(eat_u8('a'), eat_u8('b')));
        it.send(None);
        let result2 = it.send(Some('b'));
        assert_matches!(result2, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set.is_empty());
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
        assert_matches!(result0, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("a"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("bc"));
        let result2 = it.send(Some('b'));
        assert_matches!(result2, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("c"));
        let result3 = it.send(Some('c'));
        assert_matches!(result3, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set.is_empty());
    }

    #[test]
    fn test_names() {
        let mut it = ActiveCombinator::new_with_names(
            in_frame_stack(
                choice!(
                        eat_string("ab"),
                        eat_string("c"),
                        eat_string("cd"),
                        eat_string("ce"),
                    ),
            ),
            vec!["cd".to_string()],
        );
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("c"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set.is_empty());

        let mut it = ActiveCombinator::new_with_names(
            in_frame_stack(
                choice!(
                        eat_string("ab"),
                        eat_string("c"),
                        eat_string("cd"),
                        eat_string("ce"),
                    ),
            ),
            vec!["cd".to_string()],
        );
        let result1 = it.send(None);
        assert_matches!(result1, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("c"));
        let result2 = it.send(Some('c'));
        assert_matches!(result2, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("d"));
        let result3 = it.send(Some('d'));
        assert_matches!(result3, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set.is_empty());
    }

    #[test]
    fn test_names2() {
        let mut it = ActiveCombinator::new_with_names(
    choice!(
                seq!(add_to_frame_stack(eat_string("a")), in_frame_stack(eat_string("a")), eat_string("b")),
                seq!(eat_string("a"), in_frame_stack(eat_string("a")), eat_string("c")),
            ),
            vec![],
        );
        let result0 = it.send(None);
        assert_matches!(result0, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("a"));
        let result1 = it.send(Some('a'));
        assert_matches!(result1, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("a"));
        let result2 = it.send(Some('a'));
        assert_matches!(result2, ParserIterationResult { ref u8set, is_complete: false, .. } if u8set == &U8Set::from_chars("b"));
        let result3 = it.send(Some('b'));
        assert_matches!(result3, ParserIterationResult { ref u8set, is_complete: true, .. } if u8set.is_empty());
    }
}

#[cfg(test)]
mod json_parser {
    use super::*;

    #[ignore]
    #[test]
    fn test_json_parser() {
        // Helper combinators for JSON parsing
        let whitespace = repeat(choice!(eat_u8(' '), eat_u8('\t'), eat_u8('\n'), eat_u8('\r')));
        let digit = eat_u8_range('0', '9');
        let digits = repeat(digit);
        let integer = seq!(opt(choice!(eat_u8('-'), eat_u8('+'))), digits.clone());
        let fraction = seq!(eat_u8('.'), digits.clone());
        let exponent = seq!(choice!(eat_u8('e'), eat_u8('E')), seq!(choice!(eat_u8('+'), eat_u8('-')), digits));
        let number = seq!(integer, opt(fraction), opt(exponent));

        let string_char = eat_u8_range_complement('"', '"');
        let string = seq!(eat_u8('"'), repeat(string_char), eat_u8('"'));

        let json_value: Rc<dyn Combinator> = forward_ref();

        let json_array = Rc::new(seq!(
            eat_u8('['),
            whitespace.clone(),
            opt(seq!(
                json_value.clone(),
                repeat(seq!(whitespace.clone(), eat_u8(','), whitespace.clone(), json_value.clone())),
                whitespace.clone(),
            )),
            eat_u8(']'),
        ));

        let key_value_pair = seq!(string.clone(), whitespace.clone(), eat_u8(':'), whitespace.clone(), json_value.clone());

        let json_object = Rc::new(seq!(
            eat_u8('{'),
            whitespace.clone(),
            opt(seq!(
                key_value_pair.clone(),
                whitespace.clone(),
                repeat(seq!(eat_u8(','), whitespace.clone(), key_value_pair.clone())),
                whitespace.clone(),
            )),
            eat_u8('}'),
        ));

        // json_value.set(
        //     choice!(
        //         string, number,
        //         eat_string("true"), eat_string("false"),
        //         eat_string("null"), json_array, json_object,
        //     )
        // );
        let json_value = Rc::new(choice!(
            string,
            number,
            eat_string("true"),
            eat_string("false"),
            eat_string("null"),
            json_array,
            json_object,
        ));

        // Test cases
        let json_parser = seq!(whitespace, json_value);
        // let json_parser = simplify_combinator(json_parser, &mut HashSet::new());

        let test_cases = [
            "null",
            "true",
            "false",
            "42",
            r#""Hello, world!""#,
            r#"{"key": "value"}"#,
            "[1, 2, 3]",
            r#"{"nested": {"array": [1, 2, 3], "object": {"a": true, "b": false}}}"#,
        ];

        let parse_json = |json_string: &str| -> bool {
            let mut it = ActiveCombinator::new(json_parser.clone());
            let mut result = it.send(None);
            for char in json_string.chars() {
                assert!(result.u8set().contains(char as u8), "Expected {} to be in {:?}", char, result.u8set());
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
            // "GeneratedCSV_1.json",
            // "GeneratedCSV_2.json",
            // "GeneratedCSV_10.json",
            "GeneratedCSV_20.json",
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
        for i in vec![1_000, 10_000] {
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