use std::rc::Rc;
use crate::combinator::Combinator;
use crate::state::CombinatorState;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};

pub struct Call<F: Fn() -> Rc<dyn Combinator> + 'static + ?Sized>(pub Rc<F>);

impl<F> Combinator for Call<F>
where
    F: Fn() -> Rc<dyn Combinator> + 'static + ?Sized,
{
    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Box<dyn CombinatorState> {
        let inner_combinator = (self.0)();
        Box::new(CallState {
            inner_state: Some(inner_combinator.initial_state(signal_id, frame_stack)),
        })
    }

    fn next_state(&self, state: &mut dyn CombinatorState, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let state = state.as_any_mut().downcast_mut::<CallState>().expect("Invalid state type");
        let inner_combinator = (self.0)();
        let inner_state = state.inner_state.as_mut().unwrap();
        inner_combinator.next_state(inner_state.as_mut(), c, signal_id)
    }
}

pub struct CallState {
    inner_state: Option<Box<dyn CombinatorState>>,
}

impl CombinatorState for CallState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
