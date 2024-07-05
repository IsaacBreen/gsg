use std::rc::Rc;
use crate::combinator::Combinator;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;

pub struct ActiveCombinator {
    combinator: Rc<dyn Combinator>,
    state: Box<dyn CombinatorState>,
    signal_id: usize,
}

impl ActiveCombinator {
    pub fn new(combinator: Rc<dyn Combinator>) -> Self {
        let mut signal_id = 0;
        let state = combinator.initial_state(&mut signal_id, FrameStack::default());
        Self {
            combinator,
            state,
            signal_id,
        }
    }

    pub fn new_with_names(combinator: Rc<dyn Combinator>, names: Vec<String>) -> Self {
        let mut signal_id = 0;
        let mut frame_stack = FrameStack::default();
        for name in names {
            frame_stack.push_name(name.as_bytes());
        }
        let state = combinator.initial_state(&mut signal_id, frame_stack);
        Self {
            combinator,
            state,
            signal_id,
        }
    }

    pub fn send(&mut self, c: Option<char>) -> ParserIterationResult {
        self.combinator
            .next_state(&mut *self.state, c, &mut self.signal_id)
    }
}
