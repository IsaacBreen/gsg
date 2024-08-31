use crate::internal_vec::VecY;
use crate::{vecy, Fields1, Fields2, RightData, RightDataGetters};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpData {
    right_data: RightData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OneShotUpData {
    right_data: RightData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DownData {
    right_data: RightData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseResults {
    pub up_data_vec: VecY<UpData>,
    pub done: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnambiguousParseError {
    Incomplete,
    Ambiguous,
    Fail,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActuallyUnambiguousParseError {
    Incomplete,
    Fail,
}

pub type UnambiguousParseResults = Result<OneShotUpData, UnambiguousParseError>;

pub trait ParseResultTrait {
    fn done(&self) -> bool;
    fn succeeds_decisively(&self) -> bool;
    fn merge_assign(&mut self, p0: Self) where Self: Sized;
    fn merge(self, p0: Self) -> Self where Self: Sized;
    fn combine_seq(&mut self, p0: Self) where Self: Sized;
    fn new(up_data_vec: VecY<UpData>, done: bool) -> Self where Self: Sized;
    fn new_single(up_data_vec: UpData, done: bool) -> Self where Self: Sized;
    fn empty(done: bool) -> Self where Self: Sized;
    fn empty_unfinished() -> Self where Self: Sized;
    fn empty_finished() -> Self where Self: Sized;
}

impl From<ParseResults> for UnambiguousParseResults {
    fn from(value: ParseResults) -> Self {
        if !value.done() {
            return Err(UnambiguousParseError::Incomplete);
        }
        match value.up_data_vec.as_slice() {
            [] => Err(UnambiguousParseError::Fail),
            [up_data] => Ok(OneShotUpData { right_data: up_data.right_data.clone() }),
            [_, _, ..] => Err(UnambiguousParseError::Ambiguous),
        }
    }
}

impl ParseResultTrait for ParseResults {
    fn done(&self) -> bool {
        self.done
    }
    fn succeeds_decisively(&self) -> bool {
        self.done() && !self.up_data_vec.is_empty() && !self.up_data_vec.iter().any(|rd| rd.right_data.failable())
        // TODO: remove the below line and uncomment the above line
        // self.done() && !self.up_data_vec.is_empty()
    }
    fn new(up_data_vec: VecY<UpData>, done: bool) -> Self {
        ParseResults {
            up_data_vec,
            done,
        }
    }
    fn new_single(up_data: UpData, done: bool) -> Self {
        ParseResults {
            up_data_vec: vecy![up_data],
            done,
        }
    }
    fn empty(done: bool) -> Self {
        ParseResults {
            up_data_vec: VecY::new(),
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
        self.up_data_vec.append(&mut p0.up_data_vec);
        self.done &= p0.done();
    }
    fn merge(mut self, p0: ParseResults) -> Self {
        self.merge_assign(p0);
        self
    }
    fn combine_seq(&mut self, mut p0: ParseResults) {
        self.up_data_vec.append(&mut p0.up_data_vec);
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
    fn new(up_data_vec: VecY<UpData>, done: bool) -> Self {
        match (up_data_vec.len(), done) {
            (1, true) => Ok(OneShotUpData { right_data: up_data_vec[0].right_data.clone() }),
            (1, false) => Err(UnambiguousParseError::Incomplete),
            _ => Err(UnambiguousParseError::Ambiguous),
        }
    }
    fn new_single(up_data: UpData, done: bool) -> Self {
        Self::new(vecy![up_data], done)
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
            (Ok(up_data1), Ok(up_data2)) => {
                if up_data1 == up_data2 {
                    Ok(up_data1)
                } else {
                    Err(UnambiguousParseError::Ambiguous)
                }
            },
            (Ok(up_data), Err(UnambiguousParseError::Incomplete)) => Ok(up_data),
            (Err(UnambiguousParseError::Incomplete), Ok(up_data)) => Ok(up_data),
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

impl UpData {
    pub fn new(right_data: RightData) -> Self {
        Self { right_data }
    }
}

impl OneShotUpData {
    pub fn new(right_data: RightData) -> Self {
        Self { right_data }
    }
}

impl DownData {
    pub fn new(right_data: RightData) -> Self {
        Self { right_data }
    }
}

impl RightDataGetters for UpData {
    fn get_fields1(&self) -> &Fields1 { self.right_data.get_fields1() }
    fn get_fields1_mut(&mut self) -> &mut Fields1 { self.right_data.get_fields1_mut() }
    fn get_fields2(&self) -> &Fields2 { self.right_data.get_fields2() }
    fn get_fields2_mut(&mut self) -> &mut Fields2 { self.right_data.get_fields2_mut() }
    fn just_right_data(self) -> RightData { self.right_data }
}

impl RightDataGetters for OneShotUpData {
    fn get_fields1(&self) -> &Fields1 { self.right_data.get_fields1() }
    fn get_fields1_mut(&mut self) -> &mut Fields1 { self.right_data.get_fields1_mut() }
    fn get_fields2(&self) -> &Fields2 { self.right_data.get_fields2() }
    fn get_fields2_mut(&mut self) -> &mut Fields2 { self.right_data.get_fields2_mut() }
    fn just_right_data(self) -> RightData { self.right_data }
}

impl RightDataGetters for DownData {
    fn get_fields1(&self) -> &Fields1 { self.right_data.get_fields1() }
    fn get_fields1_mut(&mut self) -> &mut Fields1 { self.right_data.get_fields1_mut() }
    fn get_fields2(&self) -> &Fields2 { self.right_data.get_fields2() }
    fn get_fields2_mut(&mut self) -> &mut Fields2 { self.right_data.get_fields2_mut() }
    fn just_right_data(self) -> RightData { self.right_data.just_right_data() }
}
