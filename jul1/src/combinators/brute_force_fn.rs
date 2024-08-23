use crate::{BaseCombinatorTrait, UnambiguousParseError, UnambiguousParseResults};
use crate::RightData;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{CombinatorTrait, FailParser, ParseResults, ParserTrait, ParseResultTrait, U8Set};

pub struct Incomplete;

pub enum ParseError {
    Incomplete,
    Fail,
}

pub struct ParseFail;

type BruteForceResult = Option<Result<RightData, ParseFail>>;
type BruteForceResult2 = Result<RightData, ParseError>;
// type BruteForceResult = Result<RightData, ParseError>;
pub type BruteForceFn = dyn Fn(RightData, &[u8]) -> BruteForceResult;


pub struct BruteForce {
    pub(crate) run: Rc<BruteForceFn>,
}

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

fn convert_result(result: BruteForceResult) -> BruteForceResult2 {
    match result {
        Some(Ok(right_data)) => Ok(right_data),
        Some(Err(_)) => Err(ParseError::Fail),
        None => Err(ParseError::Incomplete),
    }
}
//
// fn convert_result(result: BruteForceResult) -> Result<RightData, ParseError> {
//     match result {
//         Ok(right_data) => Ok(right_data),
//         Err(ParseError::Fail) => Err(ParseError::Fail),
//         Err(ParseError::Incomplete) => Err(ParseError::Incomplete),
//     }
// }

impl CombinatorTrait for BruteForce {
    type Parser<'a> = BruteForceParser;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        let result = (self.run)(right_data.clone(), bytes);
        match convert_result(result) {
            Ok(right_data) => UnambiguousParseResults::Ok(right_data),
            Err(ParseError::Fail) => UnambiguousParseResults::Err(UnambiguousParseError::Fail),
            Err(ParseError::Incomplete) => UnambiguousParseResults::Err(UnambiguousParseError::Incomplete),
        }
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
        let result = (self.run)(right_data.clone(), bytes);
        let run = self.run.clone();
        match convert_result(result) {
            Ok(right_data) => (
                BruteForceParser { run, right_data: None, bytes: vec![] },
                ParseResults::new_single(right_data, true)
            ),
            Err(ParseError::Fail) => (
                BruteForceParser { run, right_data: None, bytes: vec![] },
                ParseResults::empty_finished()
            ),
            Err(ParseError::Incomplete) => (
                BruteForceParser { run, right_data: Some(right_data), bytes: bytes.to_vec() },
                ParseResults::empty_unfinished()
            ),
        }
    }
}

impl BaseCombinatorTrait for BruteForce {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ParserTrait for BruteForceParser {
    fn get_u8set(&self) -> U8Set {
        U8Set::all()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.bytes.extend_from_slice(bytes);
        if let Some(right_data) = self.right_data.take() {
            match convert_result((self.run)(right_data.clone(), &self.bytes)) {
                Ok(new_right_data) => ParseResults::new_single(new_right_data, true),
                Err(ParseError::Incomplete) => ParseResults::empty_unfinished(),
                Err(ParseError::Fail) => ParseResults::empty_finished(),
            }
        } else {
            ParseResults::empty_unfinished()
        }
    }
}

pub fn brute_force(run: impl Fn(RightData, &[u8]) -> BruteForceResult + 'static) -> BruteForce {
    BruteForce { run: Rc::new(run) }
}

// impl From<BruteForce> for Combinator {
//     fn from(value: BruteForce) -> Self {
//         Combinator::BruteForce(value)
//     }
// }

pub fn parse_error() -> BruteForceResult {
    Some(Err(ParseFail))
}

pub fn parse_incomplete() -> BruteForceResult {
    None
}

pub fn parse_ok(right_data: RightData) -> BruteForceResult {
    Some(Ok(right_data))
}

// pub fn parse_error() -> BruteForceResult {
//     Err(ParseError::Fail)
// }
//
// pub fn parse_incomplete() -> BruteForceResult {
//     Err(ParseError::Incomplete)
// }
//
// pub fn parse_ok(right_data: RightData) -> BruteForceResult {
//     Ok(right_data)
// }