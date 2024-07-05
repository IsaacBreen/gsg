use std::rc::Rc;
use crate::state::CombinatorState;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};

pub trait Combinator {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState>;
    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult;
}

impl Combinator for Rc<dyn Combinator> {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        (**self).initial_state(signal_id, frame_stack)
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        (**self).next_state(state, c, signal_id)
    }
}
