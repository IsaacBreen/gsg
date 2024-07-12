use crate::{FrameStack, U8Set};
use crate::left_recursion_guard_data::{LeftRecursionGuardDownData, LeftRecursionGuardUpData};

#[derive(Debug, Clone, PartialEq)]
pub struct RightData {
    pub frame_stack: Option<FrameStack>,
    pub indents: Vec<Vec<u8>>,
    pub dedents: usize,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct UpData {
    pub u8set: U8Set,
    pub left_recursion_guard_data: LeftRecursionGuardUpData,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DownData {
    pub left_recursion_guard_data: LeftRecursionGuardDownData,
}

impl Default for RightData {
    fn default() -> Self {
        Self {
            frame_stack: Some(FrameStack::default()),
            indents: vec![],
            dedents: 0,
        }
    }
}

pub trait Squash {
    type Output;
    fn squashed(self) -> Self::Output;
}

impl Squash for Vec<RightData> {
    type Output = Vec<RightData>;
    fn squashed(self) -> Self::Output {
        let mut new_right_data = vec![];
        for hd in self {
            if new_right_data.is_empty() || hd != new_right_data.last().cloned().unwrap() {
                new_right_data.push(hd);
            }
        }
        new_right_data
    }
}

impl Squash for Vec<UpData> {
    type Output = Vec<UpData>;
    fn squashed(self) -> Self::Output {
        let mut u8set = U8Set::none();
        for vd in self {
            u8set = u8set.union(&vd.u8set);
        }
        if u8set.is_empty() {
            vec![]
        } else {
            vec![UpData { u8set, left_recursion_guard_data: Default::default() }]
        }
    }
}

impl Squash for (Vec<RightData>, Vec<UpData>) {
    type Output = (Vec<RightData>, Vec<UpData>);
    fn squashed(self) -> Self::Output {
        (self.0.squashed(), self.1.squashed())
    }
}