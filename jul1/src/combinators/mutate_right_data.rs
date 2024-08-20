use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::*;

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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        dumb_one_shot_parse(self, right_data, bytes)
    }
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        if (self.run)(&mut right_data) {
            (Parser::FailParser(FailParser), ParseResults::new_single(right_data, true))
        } else {
            (Parser::FailParser(FailParser), ParseResults::empty_finished())
        }
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
