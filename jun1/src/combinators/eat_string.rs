use crate::combinator::Combinator;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;
use crate::u8set::U8Set;

pub struct EatString(pub &'static str);

impl Combinator for EatString {
    type State = EatStringState;

    fn initial_state(&self, _signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        EatStringState {
            index: 0,
            frame_stack,
        }
    }

    fn next_state(&self, state: &mut Self::State, _c: Option<char>, _signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<EatStringState>().expect("Invalid state type");
        if state.index > self.0.len() {
            return ParserIterationResult::new(U8Set::none(), false, state.frame_stack.clone());
        }
        if state.index == self.0.len() {
            let mut result =
                ParserIterationResult::new(U8Set::none(), true, state.frame_stack.clone());
            return result;
        }
        let u8set = U8Set::from_chars(&self.0[state.index..=state.index]);
        state.index += 1;
        ParserIterationResult::new(u8set, false, state.frame_stack.clone())
    }
}

pub struct EatStringState {
    pub index: usize,
    pub frame_stack: FrameStack,
}

impl CombinatorState for EatStringState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
