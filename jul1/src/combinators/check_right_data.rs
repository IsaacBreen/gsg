use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{Combinator, CombinatorTrait, FailParser, Parser, ParseResults, RightData};

pub struct CheckRightData {
    pub(crate) run: Rc<dyn Fn(&RightData) -> bool>,
}

impl Hash for CheckRightData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(Rc::as_ptr(&self.run) as *const (), state);
    }
}

impl PartialEq for CheckRightData {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.run, &other.run)
    }
}

impl Eq for CheckRightData {}

impl Debug for CheckRightData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CheckRightData").finish()
    }
}

impl CombinatorTrait for CheckRightData {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn parse<'a>(&'a self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'a>, ParseResults) {
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

// impl From<CheckRightData> for Combinator {
//     fn from(value: CheckRightData) -> Self {
//         Combinator::CheckRightData(value)
//     }
// }
