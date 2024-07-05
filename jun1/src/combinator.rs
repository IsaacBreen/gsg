use std::rc::Rc;
use crate::state::CombinatorState;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};

pub trait Combinator {
    type State: CombinatorState;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State;
    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult;
}

impl<C> Combinator for Rc<C>
where
    C: Combinator + ?Sized,
{
    type State = C::State;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        (**self).initial_state(signal_id, frame_stack)
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        (**self).next_state(state, c, signal_id)
    }
}

impl<C> Combinator for Box<C>
where
    C: Combinator,
{
    type State = C::State;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        (**self).initial_state(signal_id, frame_stack)
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        (**self).next_state(state, c, signal_id)
    }
}