use crate::combinator::Combinator;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;
use crate::U8Set;

pub struct Eps;

impl Combinator for Eps {
    type State = EpsState;

    fn initial_state(&self, _signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        EpsState { frame_stack }
    }

    fn next_state(&self, state: &mut Self::State, _c: Option<char>, _signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<EpsState>().expect("Invalid state type");
        let mut result = ParserIterationResult::new(U8Set::none(), true, state.frame_stack.clone());
        result
    }
}

pub struct EpsState {
    pub frame_stack: FrameStack,
}

impl CombinatorState for EpsState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
