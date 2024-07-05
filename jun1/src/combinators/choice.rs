use std::rc::Rc;
use crate::combinator::Combinator;
use crate::helper_functions::process;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;
use crate::U8Set;

pub struct Choice<State>(pub Rc<[State]>);

impl<C, State> Combinator for Choice<C> where C: Combinator<State = State> {
    type State = Box<ChoiceState<State>>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        Box::new(ChoiceState {
            its: self
                .0
                .iter()
                .map(|combinator| vec![combinator.initial_state(signal_id, frame_stack.clone())])
                .collect(),
        })
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let mut final_result =
            ParserIterationResult::new(U8Set::none(), false, FrameStack::default());
        for (combinator, its) in self.0.iter().zip(state.its.iter_mut()) {
            final_result.merge_assign(process(combinator, c, its, signal_id));
        }
        final_result
    }
}

pub struct ChoiceState<State> {
    pub its: Vec<Vec<State>>,
}

impl<State> CombinatorState for ChoiceState<State> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
