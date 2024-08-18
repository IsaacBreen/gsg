use crate::{RightData, Squash, vecy};
use crate::internal_vec::VecY;
use crate::VecX;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseResults {
    pub right_data_vec: VecY<RightData>,
    pub done: bool,
}

impl ParseResults {
    pub fn done(&self) -> bool {
        self.done
    }
    pub fn new(right_data_vec: VecY<RightData>, done: bool) -> Self {
        ParseResults {
            right_data_vec,
            done,
        }
    }
    pub fn new_single(right_data: RightData, done: bool) -> Self {
        ParseResults {
            right_data_vec: vecy![right_data],
            done,
        }
    }
    pub fn empty(done: bool) -> Self {
        ParseResults {
            right_data_vec: VecY::new(),
            done,
        }
    }
    pub fn empty_unfinished() -> Self {
        ParseResults::empty(false)
    }
    pub fn empty_finished() -> Self {
        ParseResults::empty(true)
    }
    pub(crate) fn merge_assign(&mut self, mut p0: ParseResults) {
        self.right_data_vec.append(&mut p0.right_data_vec);
        self.done &= p0.done();
    }
    pub(crate) fn merge(mut self, p0: ParseResults) -> Self {
        self.merge_assign(p0);
        self
    }
    pub fn combine_seq(&mut self, mut p0: ParseResults) {
        self.right_data_vec.append(&mut p0.right_data_vec);
        self.done |= p0.done();
    }
    pub fn succeeds_decisively(&self) -> bool {
        self.done() && !self.right_data_vec.is_empty() && !self.right_data_vec.iter().any(|rd| rd.failable())
        // TODO: remove the below line and uncomment the above line
        // self.done() && !self.right_data_vec.is_empty()
    }
}
