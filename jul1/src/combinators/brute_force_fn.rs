use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, RightData, U8Set};

pub type BruteForceFn = dyn Fn(RightData, &[u8]) -> Option<RightData>;

#[derive(Clone)]
pub struct BruteForce {
    pub(crate) run: Rc<BruteForceFn>,
}

#[derive(Clone)]
pub struct BruteForceParser {
    pub(crate) run: Rc<BruteForceFn>,
    pub(crate) right_data: Option<RightData>,
    pub(crate) bytes: Vec<u8>,
}

impl Hash for BruteForce {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(Rc::as_ptr(&self.run) as *const (), state);
    }
}

impl PartialEq for BruteForce {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.run, &other.run)
    }
}

impl Eq for BruteForce {}

impl Debug for BruteForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BruteForce").finish()
    }
}

impl Hash for BruteForceParser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(Rc::as_ptr(&self.run) as *const (), state);
        self.right_data.hash(state);
        self.bytes.hash(state);
    }
}

impl PartialEq for BruteForceParser {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.run, &other.run)
            && self.right_data == other.right_data
            && self.bytes == other.bytes
    }
}

impl Eq for BruteForceParser {}

impl Debug for BruteForceParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BruteForceParser").finish()
    }
}

impl CombinatorTrait for BruteForce {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let maybe_right_data = (self.run)(right_data.clone(), bytes);
        let run = self.run.clone();
        match maybe_right_data {
            Some(right_data) => (
                Parser::BruteForceParser(BruteForceParser { run, right_data: None, bytes: bytes.to_vec() }),
                ParseResults::new_single(right_data, true)
            ),
            None => (
                Parser::BruteForceParser(BruteForceParser { run, right_data: Some(right_data), bytes: bytes.to_vec() }),
                ParseResults::empty_unfinished()
            ),
        }
    }
}

impl ParserTrait for BruteForceParser {
    fn get_u8set(&self) -> U8Set {
        U8Set::all()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.bytes.extend_from_slice(bytes);
        if let Some(right_data) = self.right_data.take() {
            if let Some(new_right_data) = (self.run)(right_data, &self.bytes) {
                ParseResults::new_single(new_right_data, true)
            } else {
                ParseResults::empty_unfinished()
            }
        } else {
            ParseResults::empty_unfinished()
        }
    }
}

pub fn brute_force(run: impl Fn(RightData, &[u8]) -> Option<RightData> + 'static) -> BruteForce {
    BruteForce { run: Rc::new(run) }
}

impl From<BruteForce> for Combinator {
    fn from(value: BruteForce) -> Self {
        Combinator::BruteForce(value)
    }
}