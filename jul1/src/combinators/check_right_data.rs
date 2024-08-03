use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, RightData, U8Set};

#[derive(Clone)]
pub struct CheckRightData {
    pub run: Rc<dyn Fn(&RightData) -> bool>,
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

#[derive(Clone)]
pub struct CheckRightDataParser {
    pub run: Rc<dyn Fn(&RightData) -> bool>,
}

impl Hash for CheckRightDataParser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.run.as_ref() as *const dyn Fn(&RightData) -> bool, state);
    }
}

impl PartialEq for CheckRightDataParser {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&self.run, &other.run)
    }
}

impl Eq for CheckRightDataParser {}

impl Debug for CheckRightDataParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CheckRightDataParser")
    }
}

impl CombinatorTrait for CheckRightData {
    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        if (self.run)(&right_data) {
            (Parser::CheckRightDataParser(CheckRightDataParser { run: self.run.clone() }), ParseResults::new(right_data, true))
        } else {
            (Parser::CheckRightDataParser(CheckRightDataParser { run: self.run.clone() }), ParseResults {
                right_data_vec: vec![],
                done: true,
            })
        }
    }
}

impl ParserTrait for CheckRightDataParser {
    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        panic!("CheckRightData parser already consumed")
    }

    fn next_u8set(&self, bytes: &[u8]) -> U8Set {
        U8Set::none()
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
