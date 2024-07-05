use crate::combinators::*;

pub trait CombinatorState {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

// Include other state structs and their implementations
