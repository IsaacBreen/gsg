use std::rc::Rc;
use crate::state::CombinatorState;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};

pub trait Combinator {
    type State;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State;
    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult;
}

impl<C, State> Combinator for Rc<C>
where
    C: Combinator<State = State> + ?Sized,
{
    type State = State;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        (**self).initial_state(signal_id, frame_stack)
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        (**self).next_state(state, c, signal_id)
    }
}

impl<C, State> Combinator for Box<C>
where
    C: Combinator<State = State>,
{
    type State = State;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        (**self).initial_state(signal_id, frame_stack)
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        (**self).next_state(state, c, signal_id)
    }
}
