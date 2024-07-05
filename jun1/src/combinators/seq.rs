use std::rc::Rc;
use crate::combinator::Combinator;
use crate::helper_functions::{process, seq2_helper};
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;

pub struct Seq(pub Rc<[Rc<dyn Combinator>]>);

impl Combinator for Seq {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let mut its = Vec::with_capacity(self.0.len());
        its.push(vec![self.0[0].initial_state(signal_id, frame_stack)]);
        for _ in 1..self.0.len() {
            its.push(Vec::new());
        }
        Box::new(SeqState { its })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<SeqState>().expect("Invalid state type");
        let mut a_result = process(&self.0[0], c, &mut state.its[0], signal_id);
        for (combinator, its) in self.0.iter().zip(state.its.iter_mut()).skip(1) {
            let b_result = process(combinator, c, its, signal_id);
            seq2_helper(combinator, &mut a_result, b_result, its, signal_id);
        }
        a_result
    }
}

pub struct SeqState {
    pub its: Vec<Vec<Box<dyn CombinatorState>>>,
}

impl CombinatorState for SeqState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
