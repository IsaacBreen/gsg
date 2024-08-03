use crate::{RightData};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseResults {
    pub right_data_vec: Vec<RightData>,
    pub done: bool,
}

impl ParseResults {
    pub fn new(right_data: RightData, done: bool) -> Self {
        ParseResults {
            right_data_vec: vec![right_data],
            done,
        }
    }
    pub fn empty_unfinished() -> Self {
        ParseResults {
            right_data_vec: vec![],
            done: false,
        }
    }
    pub fn empty_finished() -> Self {
        ParseResults {
            right_data_vec: vec![],
            done: true,
        }
    }
    pub(crate) fn combine(&mut self, mut p0: ParseResults) {
        self.right_data_vec.append(&mut p0.right_data_vec);
        self.done &= p0.done;
    }
    pub(crate) fn combine_inplace(mut self, p0: ParseResults) -> Self {
        self.combine(p0);
        self
    }
    pub fn combine_seq(&mut self, mut p0: ParseResults) {
        self.right_data_vec.append(&mut p0.right_data_vec);
        self.done |= p0.done;
    }
}
