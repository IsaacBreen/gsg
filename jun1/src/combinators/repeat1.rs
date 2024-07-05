use std::rc::Rc;
use crate::combinator::Combinator;
use crate::helper_functions::{process, seq2_helper};
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;

pub struct Repeat1<C>(pub C);

impl<C, State> Combinator for Repeat1<C> where C: Combinator<State = State> {
    type State = Repeat1State<State>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        Repeat1State {
            a_its: vec![self.0.initial_state(signal_id, frame_stack.clone())],
        }
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let mut a_result = process(&self.0, c, &mut state.a_its, signal_id);
        let b_result = a_result.clone();
        seq2_helper(&self.0, &mut a_result, b_result, &mut state.a_its, signal_id);
        a_result
    }
}

pub struct Repeat1State<State> {
    pub a_its: Vec<State>,
}

impl<State> CombinatorState for Repeat1State<State> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
