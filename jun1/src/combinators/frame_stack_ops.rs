use std::rc::Rc;
use crate::combinator::Combinator;
use crate::parse_iteration_result::{Frame, FrameStack, ParserIterationResult};
use crate::state::CombinatorState;

pub struct WithNewFrame(pub Rc<dyn Combinator<State = Box<dyn CombinatorState>>>);
pub struct WithExistingFrame(pub Frame, pub Rc<dyn Combinator<State = Box<dyn CombinatorState>>>);
pub struct InFrameStack(pub Rc<dyn Combinator<State = Box<dyn CombinatorState>>>);
pub struct NotInFrameStack(pub Rc<dyn Combinator<State = Box<dyn CombinatorState>>>);
pub struct AddToFrameStack(pub Rc<dyn Combinator<State = Box<dyn CombinatorState>>>);
pub struct RemoveFromFrameStack(pub Rc<dyn Combinator<State = Box<dyn CombinatorState>>>);

impl Combinator for WithNewFrame {
    type State = Box<dyn CombinatorState>;

    fn initial_state(&self, signal_id: &mut usize, mut frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        frame_stack.push_empty_frame();
        let a_state = self.0.initial_state(signal_id, frame_stack);
        Box::new(WithNewFrameState { a_state })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<WithNewFrameState>().expect("Invalid state type");
        let mut result = self.0.next_state(state.a_state.as_mut(), c, signal_id);
        result.frame_stack.pop();
        result
    }
}

impl Combinator for WithExistingFrame {
    type State = Box<dyn CombinatorState>;

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
        let state = state.as_any_mut().downcast_mut::<WithExistingFrameState>().expect("Invalid state type");
        let mut result = self.1.next_state(state.a_state.as_mut(), c, signal_id);
        result.frame_stack.pop();
        result.frame_stack.push_frame(self.0.clone());
        result
    }
}

impl Combinator for InFrameStack {
    type State = Box<dyn CombinatorState>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let a_state = self.0.initial_state(signal_id, frame_stack);
        Box::new(InFrameStackState {
            a_state,
            name: Vec::new(),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<InFrameStackState>().expect("Invalid state type");
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
    }
}

impl Combinator for NotInFrameStack {
    type State = Box<dyn CombinatorState>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let a_state = self.0.initial_state(signal_id, frame_stack);
        Box::new(NotInFrameStackState {
            a_state,
            name: Vec::new(),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<NotInFrameStackState>().expect("Invalid state type");
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
    }
}

impl Combinator for AddToFrameStack {
    type State = Box<dyn CombinatorState>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let a_state = self.0.initial_state(signal_id, frame_stack);
        Box::new(AddToFrameStackState {
            a_state,
            name: Vec::new(),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<AddToFrameStackState>().expect("Invalid state type");
        if let Some(c) = c {
            state.name.push(c as u8);
        }
        let mut result = self.0.next_state(state.a_state.as_mut(), c, signal_id);
        if result.is_complete {
            result.frame_stack.push_name(state.name.as_slice());
        }
        result
    }
}

impl Combinator for RemoveFromFrameStack {
    type State = Box<dyn CombinatorState>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let a_state = self.0.initial_state(signal_id, frame_stack);
        Box::new(RemoveFromFrameStackState {
            a_state,
            name: Vec::new(),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<RemoveFromFrameStackState>().expect("Invalid state type");
        if let Some(c) = c {
            state.name.push(c as u8);
        }
        let mut result = self.0.next_state(state.a_state.as_mut(), c, signal_id);
        if result.is_complete {
            result.frame_stack.pop_name(state.name.as_slice());
        }
        result
    }
}

pub struct WithNewFrameState {
    pub a_state: Box<dyn CombinatorState>,
}
impl CombinatorState for WithNewFrameState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
pub struct WithExistingFrameState {
    pub a_state: Box<dyn CombinatorState>,
}
impl CombinatorState for WithExistingFrameState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
pub struct InFrameStackState {
    pub a_state: Box<dyn CombinatorState>,
    pub name: Vec<u8>,
}
impl CombinatorState for InFrameStackState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
pub struct NotInFrameStackState {
    pub a_state: Box<dyn CombinatorState>,
    pub name: Vec<u8>,
}
impl CombinatorState for NotInFrameStackState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
pub struct AddToFrameStackState {
    pub a_state: Box<dyn CombinatorState>,
    pub name: Vec<u8>,
}
impl CombinatorState for AddToFrameStackState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
pub struct RemoveFromFrameStackState {
    pub a_state: Box<dyn CombinatorState>,
    pub name: Vec<u8>,
}
impl CombinatorState for RemoveFromFrameStackState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
