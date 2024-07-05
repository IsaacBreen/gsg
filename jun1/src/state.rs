use crate::combinators::*;
use std::rc::Rc;

pub trait CombinatorState {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T> CombinatorState for Box<T> 
where
    T: CombinatorState + ?Sized
{
    fn as_any(&self) -> &dyn std::any::Any {
        self.as_ref() as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self.as_mut() as &mut dyn std::any::Any
    }
}

// Include other state structs and their implementations