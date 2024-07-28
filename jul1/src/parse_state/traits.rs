use std::collections::HashSet;

use crate::{ParseResults, RightData, U8Set, UpData};

pub trait Squash {
    type Output;
    fn squashed(self) -> Self::Output;
    fn squash(&mut self);
}

impl Squash for Vec<RightData> {
    type Output = Vec<RightData>;
    fn squashed(self) -> Self::Output {
        if self.len() > 1 {
            self.into_iter().collect::<HashSet<_>>().into_iter().collect()
        } else {
            self
        }
    }
    fn squash(&mut self) {
        if self.len() > 1 {
            *self = self.drain(..).collect::<Self>().squashed()
        }
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
            done: self.done,
        }
    }
    fn squash(&mut self) {
        *self = self.clone().squashed();
    }
}