use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::*;

#[derive(Clone)]
pub struct MutateRightData {
    pub(crate) run: Rc<dyn Fn(&mut RightData) -> bool>,
}

impl Hash for MutateRightData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(Rc::as_ptr(&self.run) as *const (), state);
    }
}

impl PartialEq for MutateRightData {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.run, &other.run)
    }
}

impl Eq for MutateRightData {}

impl Debug for MutateRightData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MutateRightData").finish()
    }
}

impl CombinatorTrait<'_> for MutateRightData {
    fn parse(&self, mut right_data: RightData, _bytes: &[u8]) -> (Parser, ParseResults) {
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

impl From<MutateRightData> for Combinator<'_> {
    fn from(value: MutateRightData) -> Self {
        Combinator::MutateRightData(value)
    }
}