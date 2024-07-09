use crate::{FrameStack, U8Set};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct HorizontalData {
    pub frame_stack: FrameStack,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct VerticalData {
    pub u8set: U8Set,
}
