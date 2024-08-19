use crate::{RightData, Squash, vecy};
use crate::internal_vec::VecY;
use crate::VecX;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseResults {
    pub right_data_vec: VecY<RightData>,
    pub done: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnambiguousParseError {
    Incomplete,
    Ambiguous,
    Fail,
}

pub type UnambiguousParseResults = Result<RightData, UnambiguousParseError>;

pub trait ParseResultTrait {
    fn done(&self) -> bool;
    fn succeeds_decisively(&self) -> bool;
    fn merge_assign(&mut self, p0: Self) where Self: Sized;
    fn merge(self, p0: Self) -> Self where Self: Sized;
    fn combine_seq(&mut self, p0: Self) where Self: Sized;
    fn new(right_data_vec: VecY<RightData>, done: bool) -> Self where Self: Sized;
    fn new_single(right_data_vec: RightData, done: bool) -> Self where Self: Sized;
    fn empty(done: bool) -> Self where Self: Sized;
    fn empty_unfinished() -> Self where Self: Sized;
    fn empty_finished() -> Self where Self: Sized;
}

impl ParseResultTrait for ParseResults {
    fn done(&self) -> bool {
        self.done
    }
    fn succeeds_decisively(&self) -> bool {
        self.done() && !self.right_data_vec.is_empty() && !self.right_data_vec.iter().any(|rd| rd.failable())
        // TODO: remove the below line and uncomment the above line
        // self.done() && !self.right_data_vec.is_empty()
    }
    fn new(right_data_vec: VecY<RightData>, done: bool) -> Self {
        ParseResults {
            right_data_vec,
            done,
        }
    }
    fn new_single(right_data: RightData, done: bool) -> Self {
        ParseResults {
            right_data_vec: vecy![right_data],
            done,
        }
    }
    fn empty(done: bool) -> Self {
        ParseResults {
            right_data_vec: VecY::new(),
            done,
        }
    }
    fn empty_unfinished() -> Self {
        ParseResults::empty(false)
    }
    fn empty_finished() -> Self {
        ParseResults::empty(true)
    }
    fn merge_assign(&mut self, mut p0: ParseResults) {
        self.right_data_vec.append(&mut p0.right_data_vec);
        self.done &= p0.done();
    }
    fn merge(mut self, p0: ParseResults) -> Self {
        self.merge_assign(p0);
        self
    }
    fn combine_seq(&mut self, mut p0: ParseResults) {
        self.right_data_vec.append(&mut p0.right_data_vec);
        self.done |= p0.done();
    }
}

impl ParseResultTrait for UnambiguousParseResults {
    fn done(&self) -> bool {
        match self {
            Ok(_) => true,
            Err(UnambiguousParseError::Incomplete) => false,
            Err(UnambiguousParseError::Ambiguous) => true,
            Err(UnambiguousParseError::Fail) => true,
        }
    }
    fn succeeds_decisively(&self) -> bool {
        self.is_ok()
    }
    fn new(right_data_vec: VecY<RightData>, done: bool) -> Self {
        match (right_data_vec.len(), done) {
            (1, true) => Ok(right_data_vec[0].clone()),
            (1, false) => Err(UnambiguousParseError::Incomplete),
            _ => Err(UnambiguousParseError::Ambiguous),
        }
    }
    fn new_single(right_data: RightData, done: bool) -> Self {
        Self::new(vecy![right_data], done)
    }
    fn empty(done: bool) -> Self {
        if done {
            Err(UnambiguousParseError::Fail)
        } else {
            Err(UnambiguousParseError::Incomplete)
        }
    }
    fn empty_unfinished() -> Self {
        Err(UnambiguousParseError::Incomplete)
    }
    fn empty_finished() -> Self {
        Err(UnambiguousParseError::Fail)
    }
    fn merge_assign(&mut self, p0: Self) {
        // This is a bit of a hack, but it should work
        *self = self.clone().merge(p0);
    }
    fn merge(self, p0: Self) -> Self {
        match (self, p0) {
            (Ok(right_data1), Ok(right_data2)) => {
                if right_data1 == right_data2 {
                    Ok(right_data1)
                } else {
                    Err(UnambiguousParseError::Ambiguous)
                }
            },
            (Ok(right_data), Err(UnambiguousParseError::Incomplete)) => Ok(right_data),
            (Err(UnambiguousParseError::Incomplete), Ok(right_data)) => Ok(right_data),
            (Err(UnambiguousParseError::Incomplete), Err(UnambiguousParseError::Incomplete)) => Err(UnambiguousParseError::Incomplete),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
    fn combine_seq(&mut self, p0: Self) {
        // This is a bit of a hack, but it should work
        *self = self.clone().merge(p0);
    }
}