use std::rc::Rc;
use crate::combinator::Combinator;
use crate::helper_functions::{process, seq2_helper};
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;

pub struct Repeat1(pub Rc<dyn Combinator<State = Box<dyn CombinatorState>>>);

impl Combinator for Repeat1 {
    type State = Box<dyn CombinatorState>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(Repeat1State {
            a_its: vec![self.0.initial_state(signal_id, frame_stack)],
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<Repeat1State>().expect("Invalid state type");
        let mut a_result = process(&self.0, c, &mut state.a_its, signal_id);
        let b_result = a_result.clone();
        seq2_helper(&self.0, &mut a_result, b_result, &mut state.a_its, signal_id);
        a_result
    }
}

pub struct Repeat1State {
    pub a_its: Vec<Box<dyn CombinatorState>>,
}

impl CombinatorState for Repeat1State {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
