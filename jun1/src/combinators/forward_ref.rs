use std::cell::RefCell;
use std::rc::Rc;
use crate::combinator::Combinator;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;

pub struct ForwardRef<State>(pub Rc<RefCell<Option<Rc<dyn Combinator<State = State>>>>>);

impl<State> Combinator for ForwardRef<State> {
    type State = State;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        self.0.borrow().as_ref().unwrap().initial_state(signal_id, frame_stack)
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        self.0.borrow().as_ref().unwrap().next_state(state, c, signal_id)
    }
}
