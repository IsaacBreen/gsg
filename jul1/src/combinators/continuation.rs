use crate::{BaseCombinatorTrait, DynCombinatorTrait, UnambiguousParseResults};
use crate::RightData;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{CombinatorTrait, ParseResults, ParserTrait, ParseResultTrait, U8Set};

pub type ParseFn = dyn Fn(RightData, &[u8]) -> (Box<dyn ParserTrait>, ParseResults);

pub type ParseContinuationFn = dyn FnMut(&[u8]) -> ParseResults;

pub struct ParseContinuationWrapper(Box<ParseContinuationFn>);

// impl Clone for ParseContinuationWrapper {
//     fn clone(&self) -> Self {
//         ParseContinuationWrapper(self.0.dyn_clone())
//     }
// }
//
// pub trait DynClone {
//     fn dyn_clone(&self) -> Self;
// }
//
// impl<T: ?Sized + Clone + FnMut(&[u8]) -> ParseResults> DynClone for T {
//     fn dyn_clone(&self) -> Self {
//         self.clone()
//     }
// }

pub struct Continuation {
    pub(crate) run: Rc<ParseFn>,
}

pub struct ContinuationParser {
    // pub(crate) run: ParseContinuationWrapper,
    // pub(crate) right_data: Option<RightData>,
    // pub(crate) bytes: Vec<u8>,
}

impl Hash for Continuation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(Rc::as_ptr(&self.run) as *const (), state);
    }
}

impl PartialEq for Continuation {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.run, &other.run)
    }
}

impl Eq for Continuation {}

impl Debug for Continuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Continuation").finish()
    }
}

impl Hash for ContinuationParser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // std::ptr::hash(Rc::as_ptr(&self.run) as *const (), state);
        // self.right_data.hash(state);
        // self.bytes.hash(state);
    }
}

impl PartialEq for ContinuationParser {
    fn eq(&self, other: &Self) -> bool {
        // Rc::ptr_eq(&self.run, &other.run)
        //     && self.right_data == other.right_data
        //     && self.bytes == other.bytes
        todo!()
    }
}

impl Eq for ContinuationParser {}

impl Debug for ContinuationParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContinuationParser").finish()
    }
}

impl DynCombinatorTrait for Continuation {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(right_data, bytes)
    }
}

impl CombinatorTrait for Continuation {
    type Parser<'a> = ContinuationParser;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        todo!("one_shot_parse")
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let result = (self.run)(right_data.clone(), bytes);
        let run = self.run.clone();
        // match result {
        //     ContinuationResult::Ok(right_data) => (
        //         Parser::ContinuationParser(ContinuationParser { run, right_data: None, bytes: bytes.to_vec() }),
        //         ParseResults::new_single(right_data, true)
        //     ),
        //     ContinuationResult::Err => (
        //         Parser::ContinuationParser(ContinuationParser { run, right_data: None, bytes: bytes.to_vec() }),
        //         ParseResults::empty_finished()
        //     ),
        //     ContinuationResult::Incomplete => (
        //         Parser::ContinuationParser(ContinuationParser { run, right_data: Some(right_data), bytes: bytes.to_vec() }),
        //         ParseResults::empty_unfinished()
        //     ),
        // }
        todo!()
    }
}

impl BaseCombinatorTrait for Continuation {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ParserTrait for ContinuationParser {
    fn get_u8set(&self) -> U8Set {
        U8Set::all()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        // self.bytes.extend_from_slice(bytes);
        // if let Some(right_data) = self.right_data.take() {
        //     match (self.run)(right_data.clone(), &self.bytes) {
        //         ContinuationResult::Ok(new_right_data) => ParseResults::new_single(new_right_data, true),
        //         ContinuationResult::Err => ParseResults::empty_finished(),
        //         ContinuationResult::Incomplete => {
        //             self.right_data = Some(right_data);
        //             ParseResults::empty_unfinished()
        //         }
        //     }
        // } else {
        //     ParseResults::empty_unfinished()
        // }
        todo!()
    }
}

// pub fn continuation(run: impl Fn(RightData, &[u8]) -> ContinuationResult + 'static) -> Continuation {
//     Continuation { run: Rc::new(run) }
// }
//
// impl From<Continuation> for Combinator {
//     fn from(value: Continuation) -> Self {
//         Combinator::Continuation(value)
//     }
// }