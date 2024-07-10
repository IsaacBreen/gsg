use crate::{FrameStack, U8Set};

#[derive(Debug, Clone, PartialEq)]
pub struct HorizontalData {
    pub frame_stack: Option<FrameStack>,
    pub indents: Vec<Vec<u8>>,
    pub dedents: usize,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct VerticalData {
    pub u8set: U8Set,
}

impl Default for HorizontalData {
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

impl Squash for Vec<HorizontalData> {
    type Output = Vec<HorizontalData>;
    fn squashed(self) -> Self::Output {
        let mut new_horizontal_data = vec![];
        for hd in self {
            if new_horizontal_data.is_empty() || hd != new_horizontal_data.last().cloned().unwrap() {
                new_horizontal_data.push(hd);
            }
        }
        new_horizontal_data
    }
}

impl Squash for Vec<VerticalData> {
    type Output = Vec<VerticalData>;
    fn squashed(self) -> Self::Output {
        let mut u8set = U8Set::none();
        for vd in self {
            u8set = u8set.union(&vd.u8set);
        }
        if u8set.is_empty() {
            vec![]
        } else {
            vec![VerticalData { u8set }]
        }
    }
}

impl Squash for (Vec<HorizontalData>, Vec<VerticalData>) {
    type Output = (Vec<HorizontalData>, Vec<VerticalData>);
    fn squashed(self) -> Self::Output {
        (self.0.squashed(), self.1.squashed())
    }
}