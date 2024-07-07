use crate::frame_stack::FrameStack;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ParseData {
    pub(crate) frame_stack: Option<FrameStack>,
}

impl ParseData {
    pub fn new(frame_stack: FrameStack) -> Self {
        Self { frame_stack: Some(frame_stack) }
    }

    pub fn merge(&self, other: Self) -> Self {
        let frame_stack = match (&self.frame_stack, other.frame_stack) {
            (Some(frame_stack1), Some(frame_stack2)) => Some(frame_stack1.clone() | frame_stack2),
            (None, None) => None,
            _ => panic!()
        };
        Self { frame_stack }
    }
}
