use std::rc::Rc;
use crate::combinator::Combinator;
use crate::helper_functions::process;
use crate::parse_iteration_result::{FrameStack, ParserIterationResult};
use crate::state::CombinatorState;
use crate::U8Set;

// pub struct Choice(pub Rc<[Rc<dyn Combinator<State = Box<dyn CombinatorState>>>]>);
//
// impl Combinator for Choice {
//     type State = Box<dyn CombinatorState>;
//
//     fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
//         Box::new(ChoiceState {
//             its: self
//                 .0
//                 .iter()
//                 .map(|combinator| vec![combinator.initial_state(signal_id, frame_stack.clone())])
//                 .collect(),
//         })
//     }
//
//     fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
//         let state = state.as_any_mut().downcast_mut::<ChoiceState>().expect("Invalid state type");
//         let mut final_result =
//             ParserIterationResult::new(U8Set::none(), false, FrameStack::default());
//         for (combinator, its) in self.0.iter().zip(state.its.iter_mut()) {
//             final_result.merge_assign(process(combinator.as_ref(), c, its, signal_id));
//         }
//         final_result
//     }
// }
//
// pub struct ChoiceState {
//     pub its: Vec<Vec<Box<dyn CombinatorState>>>,
// }
//
// impl CombinatorState for ChoiceState {
//     fn as_any(&self) -> &dyn std::any::Any {
//         self
//     }
//
//     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
//         self
//     }
// }

pub struct Choice2<A, B>(pub A, pub B);

impl<A, B, StateA, StateB> Combinator for Choice2<A, B>
where
    A: Combinator<State = StateA>,
    B: Combinator<State = StateB>,
{
    type State = (Option<StateA>, Option<StateB>);

    fn initial_state(&self, signal_id: &mut usize, frame_stack: FrameStack) -> Self::State {
        (Some(self.0.initial_state(signal_id, frame_stack.clone())), Some(self.1.initial_state(signal_id, frame_stack)))
    }

    fn next_state(&self, state: &mut Self::State, c: Option<char>, signal_id: &mut usize) -> ParserIterationResult {
        let (state_a, state_b) = state;
        let (combinator_a, combinator_b) = (&self.0, &self.1);
        match (state_a, state_b) {
            (Some(state_a), Some(state_b)) => {
                let result_a = combinator_a.next_state(state_a, c, signal_id);
                if result_a.u8set.is_empty() {
                    state.0 = None;
                }
                let result_b = combinator_b.next_state(state_b, c, signal_id);
                if result_b.u8set.is_empty() {
                    state.1 = None;
                }
                result_a.merge(result_b)
            }
            (Some(state_a), None) => {
                let result_a = combinator_a.next_state(state_a, c, signal_id);
                if result_a.u8set.is_empty() {
                    state.0 = None;
                }
                result_a
            }
            (None, Some(state_b)) => {
                let result_b = combinator_b.next_state(state_b, c, signal_id);
                if result_b.u8set.is_empty() {
                    state.1 = None;
                }
                result_b
            }
            _ => panic!("Invalid state")
        }
    }
}

impl<A, B> CombinatorState for (Option<A>, Option<B>)
where
    A: CombinatorState,
    B: CombinatorState,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}