use crate::combinator::Combinator;
use crate::helper_functions::{process, seq2_helper};
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;
use std::rc::Rc;

pub struct Repeat1<C>(pub Rc<C>);

impl<C> Combinator for Repeat1<C>
where
    C: Combinator<State = Box<dyn CombinatorState>> + 'static,
{
    type State = Box<dyn CombinatorState>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        self.0.initial_state(signal_id, frame_stack)
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let mut a_result = process(&self.0, c, state, signal_id);
        let b_result = a_result.clone();
        seq2_helper(&self.0, &mut a_result, b_result, state, signal_id);
        a_result
    }
}