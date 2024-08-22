use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::*;
use crate::BaseCombinatorTrait;

pub struct MutateRightData {
    pub(crate) run: Box<dyn Fn(&mut RightData) -> bool>,
}

impl Hash for MutateRightData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ptr = std::ptr::addr_of!(self.run) as *const ();
        std::ptr::hash(ptr, state);
    }
}

impl PartialEq for MutateRightData {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&self.run, &other.run)
    }
}

impl Eq for MutateRightData {}

impl Debug for MutateRightData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MutateRightData").finish()
    }
}

impl CombinatorTrait for MutateRightData {
    type Parser = FailParser;

    fn one_shot_parse(&self, mut right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        if (self.run)(&mut right_data) {
            Ok(right_data)
        } else {
            Err(UnambiguousParseError::Fail)
        }
    }
    fn old_parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
        if (self.run)(&mut right_data) {
            (FailParser, ParseResults::new_single(right_data, true))
        } else {
            (FailParser, ParseResults::empty_finished())
        }
    }
}

impl BaseCombinatorTrait for MutateRightData {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub fn mutate_right_data(run: impl Fn(&mut RightData) -> bool + 'static) -> MutateRightData {
    MutateRightData { run: Box::new(run) }
}
//
// impl From<MutateRightData> for Combinator {
//     fn from(value: MutateRightData) -> Self {
//         Combinator::MutateRightData(value)
//     }
// }