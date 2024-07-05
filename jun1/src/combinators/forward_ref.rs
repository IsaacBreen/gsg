use std::cell::RefCell;
use std::rc::Rc;
use crate::combinator::Combinator;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;

pub struct ForwardRef<C>(pub Rc<RefCell<Option<C>>>);

impl<C> Combinator for ForwardRef<C>
where
    C: Combinator<State = Box<dyn CombinatorState>> + 'static,
{
    type State = Box<ForwardRefState<Box<dyn CombinatorState>>>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        Box::new(ForwardRefState {
            inner_state: self.0.borrow().as_ref().map(|c| c.initial_state(signal_id, frame_stack)),
        })
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        match state.inner_state.as_mut() {
            Some(inner_state) => match self.0.borrow().as_ref() {
                Some(combinator) => combinator.next_state(inner_state, c, signal_id),
                None => panic!("Forward reference not set before use"),
            },
            None => panic!("Forward reference not set before use"),
        }
    }
}

pub struct ForwardRefState<State> {
    pub inner_state: Option<State>,
}

impl<State: CombinatorState + 'static> CombinatorState for ForwardRefState<State> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}