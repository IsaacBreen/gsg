use crate::combinator::Combinator;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;
use crate::u8set::U8Set;

pub struct EatU8Matching(pub U8Set);

impl Combinator for EatU8Matching {
    fn initial_state(&self, _signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        Box::new(EatU8MatchingState {
            state: 0,
            frame_stack,
        })
    }

    fn next_state(
        &self,
        state: &mut dyn CombinatorState,
        c: Option<char>,
        _signal_id: &mut usize,
    ) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<EatU8MatchingState>().expect("Invalid state type");
        match state.state {
            0 => {
                state.state = 1;
                ParserIterationResult::new(self.0.clone(), false, state.frame_stack.clone())
            }
            1 => {
                state.state = 2;
                let is_complete = c.map(|c| self.0.contains(c as u8)).unwrap_or(false);
                let mut result = ParserIterationResult::new(
                    U8Set::none(),
                    is_complete,
                    state.frame_stack.clone(),
                );
                result
            }
            _ => panic!("EatU8Matching: state out of bounds"),
        }
    }
}

pub struct EatU8MatchingState {
    pub state: u8,
    pub frame_stack: FrameStack,
}

impl CombinatorState for EatU8MatchingState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
