use crate::{Combinator, CombinatorState, FrameStack, ParserIterationResult, process, seq2_helper, U8Set};

pub struct Seq2<A, B>(pub A, pub B);

impl<A, B, StateA, StateB> Combinator for Seq2<A, B>
where
    A: Combinator<State = StateA>,
    B: Combinator<State = StateB>,
{
    type State = (Option<StateA>, Vec<StateB>);

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        (Some(self.0.initial_state(signal_id, frame_stack.clone())), Vec::new())
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let (a_state, ref mut b_states) = state;
        let (a_combinator, b_combinator) = (&self.0, &self.1);
        if let Some(a_state) = a_state {
            let mut a_result = a_combinator.next_state(a_state, c, signal_id);
            if a_result.u8set().is_empty() {
                state.0 = None;
            }
            let b_result = process(b_combinator, c, b_states, signal_id);
            seq2_helper(b_combinator, &mut a_result, b_result, b_states, signal_id);
            a_result
        } else {
            process(b_combinator, c, b_states, signal_id)
        }
    }
}

impl<A, B> CombinatorState for (Option<A>, Vec<B>)
where
    A: CombinatorState,
    B: CombinatorState,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}