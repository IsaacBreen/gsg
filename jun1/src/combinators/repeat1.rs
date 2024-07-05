use std::rc::Rc;
use crate::combinator::Combinator;
use crate::helper_functions::{process, seq2_helper};
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;

pub struct Repeat1<C>(pub Rc<C>);

impl<C> Combinator for Repeat1<C>
where
    C: Combinator<State = Box<dyn CombinatorState>> + 'static,
{
    type State = Box<Repeat1State<Box<dyn CombinatorState>>>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        Box::new(Repeat1State {
            inner_state: vec![self.0.initial_state(signal_id, frame_stack)],
        })
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let mut a_result = process(&self.0, c, &mut state.inner_state, signal_id);
        let b_result = a_result.clone();
        seq2_helper(&self.0, &mut a_result, b_result, &mut state.inner_state, signal_id);
        a_result
    }
}

pub struct Repeat1State<State> {
    pub inner_state: Vec<State>,
}

impl<State: CombinatorState + 'static> CombinatorState for Repeat1State<State> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}