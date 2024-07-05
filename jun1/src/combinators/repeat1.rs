use crate::combinator::Combinator;
use crate::helper_functions::{process, seq2_helper};
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;

pub struct Repeat1<C>(pub C);

impl<C> Combinator for Repeat1<C>
where
    C: Combinator<State = Box<dyn CombinatorState>> + 'static,
{
    type State = Box<Repeat1State<Box<dyn CombinatorState>>>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        Box::new(Repeat1State {
            a_its: vec![self.0.initial_state(signal_id, frame_stack)],
        })
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        process(&self.0, c, &mut state.a_its, signal_id)
    }
}

pub struct Repeat1State<State> {
    pub a_its: Vec<State>,
}

impl<State: CombinatorState + 'static> CombinatorState for Repeat1State<State> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}