use crate::{FrameStack, U8Set};

#[derive(Debug, Clone, PartialEq)]
pub struct ParseResults(pub Vec<RightData>, pub Vec<UpData>);

#[derive(Debug, Clone, PartialEq)]
pub struct RightData {
    pub frame_stack: Option<FrameStack>,
    pub indents: Vec<Vec<u8>>,
    pub dedents: usize,
    pub scope_count: usize,
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
            scope_count: 0,
        }
    }
}

impl RightData {
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

impl Squash for ParseResults {
    type Output = ParseResults;
    fn squashed(self) -> Self::Output {
        ParseResults(self.0.squashed(), self.1.squashed())
    }
}