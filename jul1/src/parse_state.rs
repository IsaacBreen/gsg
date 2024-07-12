use crate::{FrameStack, U8Set};
use crate::left_recursion_guard_data::{LeftRecursionGuardUpData, LeftRecursionGuardDownData};

#[derive(Debug, Clone, PartialEq)]
pub struct RightData {
    pub frame_stack: Option<FrameStack>,
    pub indents: Vec<Vec<u8>>,
    pub dedents: usize,
    pub left_recursion_guard_data: LeftRecursionGuardDownData,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct UpData {
    pub u8set: U8Set,
}

impl Default for RightData {
    fn default() -> Self {
        Self {
            frame_stack: Some(FrameStack::default()),
            indents: vec![],
            dedents: 0,
            left_recursion_guard_data: LeftRecursionGuardDownData::default(),
        }
    }
}

impl RightData {
    pub fn may_consume(&self) -> bool {
        self.left_recursion_guard_data.may_consume()
    }

    pub fn on_consume(&mut self) {
        self.left_recursion_guard_data.on_consume();
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
            vec![UpData { u8set }]
        }
    }
}

impl Squash for (Vec<RightData>, Vec<UpData>) {
    type Output = (Vec<RightData>, Vec<UpData>);
    fn squashed(self) -> Self::Output {
        (self.0.squashed(), self.1.squashed())
    }
}