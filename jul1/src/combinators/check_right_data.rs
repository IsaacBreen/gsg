use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, FailParser, Parser, ParseResults, RightData};

#[derive(Clone)]
pub struct CheckRightData {
    run: Rc<dyn Fn(&RightData) -> bool>,
}

impl Hash for CheckRightData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.run.as_ref() as *const dyn Fn(&RightData) -> bool, state);
    }
}

impl PartialEq for CheckRightData {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&self.run, &other.run)
    }
}

impl Eq for CheckRightData {}

impl Debug for CheckRightData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CheckRightData")
    }
}

impl CombinatorTrait for CheckRightData {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        if (self.run)(&right_data) {
            (Parser::FailParser(FailParser), ParseResults::new_single(right_data, true))
        } else {
            (Parser::FailParser(FailParser), ParseResults::empty_finished())
        }
    }
}

pub fn check_right_data(run: impl Fn(&RightData) -> bool + 'static) -> CheckRightData {
    CheckRightData { run: Rc::new(run) }
}

impl From<CheckRightData> for Combinator {
    fn from(value: CheckRightData) -> Self {
        Combinator::CheckRightData(value)
    }
}
