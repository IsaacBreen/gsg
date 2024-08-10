use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{Combinator, CombinatorTrait, FailParser, Parser, ParseResults, ParserTrait, RightData, U8Set};

#[derive(Clone)]
pub struct BruteForce {
    pub(crate) run: Rc<dyn Fn(RightData, &[u8]) -> Option<RightData>>,
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

impl CombinatorTrait for BruteForce {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let maybe_right_data = (self.run)(right_data.clone(), bytes);
        if let Some(right_data) = maybe_right_data {
            (Parser::BruteForceParser(BruteForceParser::new(self.run.clone(), right_data.clone(), bytes.to_vec())), ParseResults::new_single(right_data, true))
        } else {
            (Parser::BruteForceParser(BruteForceParser::new(self.run.clone(), right_data, bytes.to_vec())), ParseResults::empty_unfinished())
        }
    }
}

#[derive(Clone)]
pub struct BruteForceParser {
    pub(crate) run: Rc<dyn Fn(RightData, &[u8]) -> Option<RightData>>,
    pub(crate) right_data: RightData,
    pub(crate) bytes: Vec<u8>,
}

impl BruteForceParser {
    pub fn new(run: Rc<dyn Fn(RightData, &[u8]) -> Option<RightData>>, right_data: RightData, bytes: Vec<u8>) -> Self {
        Self { run, right_data, bytes }
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
        Rc::ptr_eq(&self.run, &other.run) && self.right_data == other.right_data && self.bytes == other.bytes
    }
}

impl Eq for BruteForceParser {}

impl Debug for BruteForceParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BruteForceParser").finish()
    }
}

impl ParserTrait for BruteForceParser {
    fn get_u8set(&self) -> U8Set {
        U8Set::all()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.bytes.extend_from_slice(bytes);
        let maybe_right_data = (self.run)(self.right_data.clone(), &self.bytes);
        if let Some(right_data) = maybe_right_data {
            ParseResults::new_single(right_data, true)
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