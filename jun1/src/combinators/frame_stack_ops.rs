use std::rc::Rc;
use crate::combinator::Combinator;
use crate::parse_iteration_result::{Frame, FrameStack, ParserIterationResult};
use crate::state::CombinatorState;

pub struct WithNewFrame<C>(pub Rc<C>);
pub struct WithExistingFrame<C>(pub Frame, pub Rc<C>);
pub struct InFrameStack<C>(pub Rc<C>);
pub struct NotInFrameStack<C>(pub Rc<C>);
pub struct AddToFrameStack<C>(pub Rc<C>);
pub struct RemoveFromFrameStack<C>(pub Rc<C>);

impl<C, State> Combinator for WithNewFrame<C> where C: Combinator<State = State> {
    type State = WithNewFrameState<State>;

    fn initial_state(&self, signal_id: &mut usize, mut frame_stack: FrameStack) -> Self::State {
        frame_stack.push_empty_frame();
        let a_state = self.0.initial_state(signal_id, frame_stack);
        WithNewFrameState { a_state }
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let mut result = self.0.next_state(Box::new(state.a_state).as_mut(), c, signal_id);
        result.frame_stack.pop();
        result
    }
}

impl<C, State> Combinator for WithExistingFrame<C> where C: Combinator<State = State> {
    type State = WithExistingFrameState<State>;

    fn initial_state(&self, signal_id: &mut usize, mut frame_stack: FrameStack) -> Self::State {
        frame_stack.push_frame(self.0.clone());
        let a_state = self.1.initial_state(signal_id, frame_stack);
        WithExistingFrameState { a_state }
    }

    fn next_state(
        &self,
        state: &mut Self::State,
        c: Option<char>,
        signal_id: &mut usize,
    ) -> ParserIterationResult {
        let mut result = self.1.next_state(Box::new(state.a_state).as_mut(), c, signal_id);
        result.frame_stack.pop();
        result.frame_stack.push_frame(self.0.clone());
        result
    }
}

impl<C, State> Combinator for InFrameStack<C> where C: Combinator<State = State> {
    type State = InFrameStackState<State>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        let a_state = self.0.initial_state(signal_id, frame_stack);
        InFrameStackState {
            a_state,
            name: Vec::new(),
        }
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        if let Some(c) = c {
            state.name.push(c as u8);
        }
        let mut result = self.0.next_state(Box::new(state.a_state).as_mut(), c, signal_id);
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
    }
}

impl<C, State> Combinator for NotInFrameStack<C> where C: Combinator<State = State> {
    type State = NotInFrameStackState<State>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        let a_state = self.0.initial_state(signal_id, frame_stack);
        NotInFrameStackState {
            a_state,
            name: Vec::new(),
        }
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        if let Some(c) = c {
            state.name.push(c as u8);
        }
        let mut result = self.0.next_state(Box::new(state.a_state).as_mut(), c, signal_id);
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
    }
}

impl<C, State> Combinator for AddToFrameStack<C> where C: Combinator<State = State> {
    type State = AddToFrameStackState<State>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        let a_state = self.0.initial_state(signal_id, frame_stack);
        AddToFrameStackState {
            a_state,
            name: Vec::new(),
        }
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        if let Some(c) = c {
            state.name.push(c as u8);
        }
        let mut result = self.0.next_state(Box::new(state.a_state).as_mut(), c, signal_id);
        if result.is_complete {
            result.frame_stack.push_name(state.name.as_slice());
        }
        result
    }
}

impl<C, State> Combinator for RemoveFromFrameStack<C> where C: Combinator<State = State> {
    type State = RemoveFromFrameStackState<State>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        let a_state = self.0.initial_state(signal_id, frame_stack);
        RemoveFromFrameStackState {
            a_state,
            name: Vec::new(),
        }
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        if let Some(c) = c {
            state.name.push(c as u8);
        }
        let mut result = self.0.next_state(Box::new(state.a_state).as_mut(), c, signal_id);
        if result.is_complete {
            result.frame_stack.pop_name(state.name.as_slice());
        }
        result
    }
}

pub struct WithNewFrameState<State> {
    pub a_state: State,
}
impl<State> CombinatorState for WithNewFrameState<State> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
pub struct WithExistingFrameState<State> {
    pub a_state: State,
}
impl<State> CombinatorState for WithExistingFrameState<State> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
pub struct InFrameStackState<State> {
    pub a_state: State,
    pub name: Vec<u8>,
}
impl<State> CombinatorState for InFrameStackState<State> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
pub struct NotInFrameStackState<State> {
    pub a_state: State,
    pub name: Vec<u8>,
}
impl<State> CombinatorState for NotInFrameStackState<State> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
pub struct AddToFrameStackState<State> {
    pub a_state: State,
    pub name: Vec<u8>,
}
impl<State> CombinatorState for AddToFrameStackState<State> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
pub struct RemoveFromFrameStackState<State> {
    pub a_state: State,
    pub name: Vec<u8>,
}
impl<State> CombinatorState for RemoveFromFrameStackState<State> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
