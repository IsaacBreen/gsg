use crate::combinator::Combinator;
use crate::helper_functions::process;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;
use crate::U8Set;
use std::rc::Rc;

pub struct Choice<C>(pub Vec<C>);

impl<C> Combinator for Choice<Rc<C>>
where
    C: Combinator<State = Box<dyn CombinatorState>> + 'static,
{
    type State = Box<ChoiceState<Box<dyn CombinatorState>>>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        Box::new(ChoiceState {
            its: self
                .0
                .iter()
                .map(|combinator| {
                    vec![combinator.initial_state(signal_id, frame_stack.clone())]
                })
                .collect(),
        })
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let mut final_result = ParserIterationResult::new(U8Set::none(), false, FrameStack::default());
        for (combinator, its) in self.0.iter().zip(state.its.iter_mut()) {
            let mut tmp = combinator.initial_state(signal_id, FrameStack::default());
            let res = process(combinator, c, &mut tmp, signal_id);
            final_result.merge_assign(res);
            if res.u8set().len() != 0 {
                its.push(tmp);
            }
        }
        final_result
    }
}

pub struct ChoiceState<S> {
    pub its: Vec<Vec<S>>,
}

impl<S> CombinatorState for Box<ChoiceState<S>>
where
    S: CombinatorState
{
    fn as_any(&self) -> &dyn std::any::Any {
        self.as_ref() as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self.as_mut() as &mut dyn std::any::Any
    }
}