use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::*;

#[derive(Clone)]
pub struct MutateRightData {
    pub run: Rc<dyn Fn(&mut RightData) -> bool>,
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

#[derive(Clone)]
pub struct MutateRightDataParser {
    pub run: Rc<dyn Fn(&mut RightData) -> bool>,
}

impl Hash for MutateRightDataParser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.run.as_ref() as *const dyn Fn(&mut RightData) -> bool, state);
    }
}

impl PartialEq for MutateRightDataParser {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&self.run, &other.run)
    }
}

impl Eq for MutateRightDataParser {}

impl Debug for MutateRightDataParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MutateRightDataParser")
    }
}

impl CombinatorTrait for MutateRightData {
    fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
        if (self.run)(&mut right_data) {
            (Parser::MutateRightDataParser(MutateRightDataParser { run: self.run.clone() }), ParseResults {
                right_data_vec: vec![right_data],
                up_data_vec: vec![],
                done: true
            })
        } else {
            (Parser::MutateRightDataParser(MutateRightDataParser { run: self.run.clone() }), ParseResults {
                right_data_vec: vec![],
                up_data_vec: vec![],
                done: true,
            })
        }
    }

    fn parser_with_steps(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
         if (self.run)(&mut right_data) {
            (Parser::MutateRightDataParser(MutateRightDataParser { run: self.run.clone() }), ParseResults {
                right_data_vec: vec![right_data],
                up_data_vec: vec![],
                done: true
            })
        } else {
            (Parser::MutateRightDataParser(MutateRightDataParser { run: self.run.clone() }), ParseResults {
                right_data_vec: vec![],
                up_data_vec: vec![],
                done: true,
            })
        }
    }
}

impl ParserTrait for MutateRightDataParser {
    fn step(&mut self, c: u8) -> ParseResults {
        panic!("MutateRightData parser already consumed")
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        panic!("MutateRightData parser already consumed")
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
