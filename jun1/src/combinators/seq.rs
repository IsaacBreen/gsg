use crate::combinator::Combinator;
use crate::helper_functions::{process, seq2_helper};
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;

pub struct Seq<C>(pub Vec<C>);

impl<C> Combinator for Seq<C>
where
    C: Combinator<State = Box<dyn CombinatorState>> + 'static,
{
    type State = SeqState<Box<dyn CombinatorState>>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        let mut its = Vec::with_capacity(self.0.len());
        its.push(vec![self.0[0].initial_state(signal_id, frame_stack)]);
        for _ in 1..self.0.len() {
            its.push(Vec::new());
        }
        SeqState { its }
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let mut a_result = process(&self.0[0], c, &mut state.its[0], signal_id);
        for (combinator, its) in self.0.iter().zip(state.its.iter_mut()).skip(1) {
            let b_result = process(combinator, c, its, signal_id);
            seq2_helper(combinator, &mut a_result, b_result, its, signal_id);
        }
        a_result
    }
}

pub struct SeqState<State> {
    pub its: Vec<Vec<State>>,
}

impl<State: CombinatorState + 'static> CombinatorState for SeqState<State> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}