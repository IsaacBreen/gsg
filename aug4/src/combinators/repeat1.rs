use std::any::Any;
use std::fmt::{Debug, Formatter};
use crate::*;
use crate::compile::Compile;
use crate::helper_traits::AsAny;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Repeat1<T> {
    pub child: T,
}

impl<T: CombinatorTrait> Repeat1<T> {
    pub fn new(child: T) -> Self {
        Self { child }
    }
}

impl<T: CombinatorTrait> AsAny for Repeat1<T> {
    fn as_any<'a>(&'a self) -> &(dyn Any + 'a) {
        self
    }
}

impl<T: CombinatorTrait> Compile for Repeat1<T> {
    fn compile_inner(&self) {
        self.child.compile_inner();
    }
}

impl<T: CombinatorTrait> CombinatorTrait for Repeat1<T> {
    fn parse(&self, mut right_data: RightData, input: &[u8]) -> UnambiguousParseResults {
        let mut prev_result = Err(UnambiguousParseError::Fail);
        loop {
            let parse_result = self.child.parse(right_data.clone(), input);
            match parse_result {
                Ok(new_right_data) => {
                    right_data = new_right_data.clone();
                    prev_result = Ok(new_right_data);
                }
                Err(UnambiguousParseError::Fail) => return prev_result,
                Err(UnambiguousParseError::Incomplete | UnambiguousParseError::Ambiguous) => {
                    return prev_result;
                }
            }
        }
    }

    fn rotate_right<'a>(&'a self) -> Choice<Seq<&'a dyn CombinatorTrait>> {
        choice!(seq!(self))
    }
}

