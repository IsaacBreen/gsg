use std::rc::Rc;
use crate::combinator::Combinator;
use crate::helper_functions::process;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;
use crate::U8Set;

pub struct Choice(pub Rc<[Rc<dyn Combinator>]>);

impl Combinator for Choice {
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(ChoiceState {
            its: self
                .0
                .iter()
                .map(|combinator| vec![combinator.initial_state(signal_id, frame_stack.clone())])
                .collect(),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<ChoiceState>().expect("Invalid state type");
        let mut final_result =
            ParserIterationResult::new(U8Set::none(), false, FrameStack::default());
        for (combinator, its) in self.0.iter().zip(state.its.iter_mut()) {
            final_result.merge_assign(process(combinator, c, its, signal_id));
        }
        final_result
    }
}

pub struct ChoiceState {
    pub its: Vec<Vec<Box<dyn CombinatorState>>>,
}

impl CombinatorState for ChoiceState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
