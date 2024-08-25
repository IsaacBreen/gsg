use crate::RightData;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{ParseResults, ParseResultTrait, U8Set};

// macro_rules! profile {
//     ($tag:expr, $body:expr) => {{
//         $crate::profile!($tag, $body)
//     }};
// }

macro_rules! profile {
    ($tag:expr, $body:expr) => {{
        $body
    }};
}

const SQUASH_THRESHOLD: usize = 1;

pub trait Squash {
    type Output;
    fn squashed(self) -> Self::Output;
    fn squash(&mut self);
}

impl Squash for Vec<RightData> {
    type Output = Vec<RightData>;
    fn squashed(self) -> Self::Output {
        if self.len() > SQUASH_THRESHOLD {
            profile!("RightDataSquasher::squashed", {
                // let mut squasher = RightDataSquasher::new();
                // squasher.extend(self.into_iter());
                // squasher.finish()
                self.into_iter().collect::<HashSet<_>>().into_iter().collect()
            })
        } else {
            self
        }
    }
    fn squash(&mut self) {
        if self.len() > SQUASH_THRESHOLD {
            *self = self.drain(..).collect::<Vec<RightData>>().squashed()
        }
    }
}

impl Squash for ParseResults {
    type Output = ParseResults;
    fn squashed(self) -> Self::Output {
        if self.right_data_vec.len() > SQUASH_THRESHOLD {
            profile!("ParseResults::squashed", {
                    let done = self.done();
                    ParseResults::new(self.right_data_vec.squashed(), done)
                }
            )
        } else {
            self
        }
    }
    fn squash(&mut self) {
        if self.right_data_vec.len() > SQUASH_THRESHOLD {
            // *self.right_data_vec = std::mem::take(&mut self.right_data_vec).squashed();
            *self = self.clone().squashed();
        }
    }
}
