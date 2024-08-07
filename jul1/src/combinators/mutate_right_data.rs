use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::*;

#[derive(Clone)]
pub struct MutateRightData {
    run: Rc<dyn Fn(&mut RightData) -> bool>,
}

impl Hash for MutateRightData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.run.as_ref() as *const dyn Fn(&mut RightData) -> bool, state);
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
        write!(f, "MutateRightData")
    }
}

impl CombinatorTrait for MutateRightData {
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        if (self.run)(&mut right_data) {
            (Parser::FailParser(FailParser), ParseResults::new_single(right_data, true))
        } else {
            (Parser::FailParser(FailParser), ParseResults::empty_finished())
        }
    }
}

pub fn mutate_right_data(run: impl Fn(&mut RightData) -> bool + 'static) -> MutateRightData {
    MutateRightData { run: Rc::new(run) }
}

impl From<MutateRightData> for Combinator {
    fn from(value: MutateRightData) -> Self {
        Combinator::MutateRightData(value)
    }
}
