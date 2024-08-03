use std::collections::HashSet;

use crate::{ParseResults, RightData, U8Set};

const SQUASH_THRESHOLD: usize = 0;

pub trait Squash {
    type Output;
    fn squashed(self) -> Self::Output;
    fn squash(&mut self);
}

impl Squash for Vec<RightData> {
    type Output = Vec<RightData>;
    fn squashed(self) -> Self::Output {
        if self.len() > SQUASH_THRESHOLD {
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

impl Squash for ParseResults {
    type Output = ParseResults;
    fn squashed(self) -> Self::Output {
        ParseResults {
            right_data_vec: self.right_data_vec.squashed(),
            done: self.done,
        }
    }
    fn squash(&mut self) {
        *self = self.clone().squashed();
    }
}