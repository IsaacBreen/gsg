use crate::{FrameStack};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ParseData {
    pub(crate) frame_stack: FrameStack,
}

impl ParseData {
    pub fn new(frame_stack: FrameStack) -> Self {
        Self { frame_stack }
    }

    pub fn merge(&self, other: Self) -> Self {
        Self { frame_stack: self.frame_stack.clone() | other.frame_stack }
    }
}
