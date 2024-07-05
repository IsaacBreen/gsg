use std::cell::RefCell;
use std::rc::Rc;
use crate::combinator::Combinator;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;

pub struct ForwardRef(pub Rc<RefCell<Option<Rc<dyn Combinator<State = Box<dyn CombinatorState>>>>>>);

impl Combinator for ForwardRef {
    type State = Box<dyn CombinatorState>;

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let inner_state = match self.0.borrow().as_ref() {
            Some(c) => Some(c.initial_state(signal_id, frame_stack)),
            None => panic!("ForwardRef not set"),
        };
        Box::new(ForwardRefState { inner_state })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<ForwardRefState>().expect("Invalid state type");
        match state.inner_state.as_mut() {
            Some(inner_state) => match self.0.borrow().as_ref() {
                Some(combinator) => combinator.next_state(inner_state.as_mut(), c, signal_id),
                None => panic!("Forward reference not set before use"),
            },
            None => panic!("Forward reference not set before use"),
        }
    }
}

pub struct ForwardRefState {
    pub inner_state: Option<Box<dyn CombinatorState>>,
}

impl CombinatorState for ForwardRefState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
