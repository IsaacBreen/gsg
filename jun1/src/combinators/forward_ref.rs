use std::cell::RefCell;
use std::rc::Rc;
use crate::combinator::Combinator;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;

pub struct ForwardRef<C>(pub Rc<RefCell<Option<C>>>);

impl<C> Combinator for ForwardRef<C>
where
    C: Combinator,
    C::State: 'static,
{
    type State = ForwardRefState<C::State>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        let inner_state = match self.0.borrow().as_ref() {
            Some(c) => Some(c.initial_state(signal_id, frame_stack)),
            None => panic!("ForwardRef not set"),
        };
        ForwardRefState { inner_state }
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

impl<State: CombinatorState> CombinatorState for ForwardRefState<State> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}