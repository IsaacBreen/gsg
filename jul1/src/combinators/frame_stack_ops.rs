use crate::Eps;

pub enum FrameStackOp {
    WithNewFrame,
    PushToFrame,
    PopFromFrame,
    FrameStackContains,
}

// pub enum FrameStackOpParser {
//     WithNewFrame(Eps),
//     PushToFrame(Eps),
//     PopFromFrame(Eps),
//     FrameStackContains(Eps),
// }

pub fn with_new_frame<T>(parser: T) -> Eps {
    todo!()
}

pub fn push_to_frame<T>(parser: T) -> Eps {
    todo!()
}

pub fn pop_from_frame<T>(parser: T) -> Eps {
    todo!()
}

pub fn frame_stack_contains<T>(parser: T) -> Eps {
    todo!()
}