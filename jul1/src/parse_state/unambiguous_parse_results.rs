use crate::RightData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnambiguousParseResults {
    pub option_right_data: Option<RightData>,
    pub done: bool,
}

impl UnambiguousParseResults {
    pub fn done(&self) -> bool {
        self.done
    }
    pub fn new(right_data: RightData, done: bool) -> Self {
        UnambiguousParseResults {
            option_right_data: Some(right_data),
            done,
        }
    }
    pub fn new_single(right_data: RightData, done: bool) -> Self {
        UnambiguousParseResults {
            option_right_data: Some(right_data),
            done,
        }
    }
    pub fn empty(done: bool) -> Self {
        UnambiguousParseResults {
            option_right_data: None,
            done,
        }
    }
    pub fn empty_unfinished() -> Self {
        UnambiguousParseResults::empty(false)
    }
    pub fn empty_finished() -> Self {
        UnambiguousParseResults::empty(true)
    }
    pub fn succeeds_decisively(&self) -> bool {
        self.done() && !self.option_right_data.is_none() && !self.option_right_data.as_ref().unwrap().failable()
        // TODO: remove the below line and uncomment the above line
        // self.done() && !self.option_right_data.is_empty()
    }
}
