use std::rc::Rc;
use crate::combinator::Combinator;
use crate::state::CombinatorState;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};

pub struct Call<F: Fn() -> Rc<C> + 'static + ?Sized, C: Combinator<State = State>, State>(pub F);

impl<F, C, State> Combinator for Call<F, C, State>
where
    F: Fn() -> Rc<C> + 'static + ?Sized,
    C: Combinator<State = State>,
{
    type State = State;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        let inner_combinator = (self.0)();
        inner_combinator.initial_state(signal_id, frame_stack)
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let inner_combinator = (self.0)();
        inner_combinator.next_state(state, c, signal_id)
    }
}
