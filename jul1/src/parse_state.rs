use crate::{FrameStack, U8Set};

#[derive(Debug, Clone, PartialEq)]
pub struct ParseResults {
    pub right_data_vec: Vec<RightData>,
    pub up_data_vec: Vec<UpData>,
    pub cut: bool,
}

impl ParseResults {
    pub(crate) fn combine(&mut self, mut p0: ParseResults) {
        self.right_data_vec.append(&mut p0.right_data_vec);
        self.up_data_vec.append(&mut p0.up_data_vec);
        self.cut |= p0.cut;
    }

    pub(crate) fn combine_inplace(mut self, p0: ParseResults) -> Self {
        self.combine(p0);
        self
    }
}

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
    fn squash(&mut self);
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
    fn squash(&mut self) {
        *self = self.clone().squashed();
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
    fn squash(&mut self) {
        *self = self.clone().squashed();
    }
}

impl Squash for ParseResults {
    type Output = ParseResults;
    fn squashed(self) -> Self::Output {
        ParseResults {
            right_data_vec: self.right_data_vec.squashed(),
            up_data_vec: self.up_data_vec.squashed(),
            cut: self.cut,
        }
    }
    fn squash(&mut self) {
        *self = self.clone().squashed();
    }
}