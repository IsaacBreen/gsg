use std::cell::RefCell;
use std::rc::Rc;
use derivative::Derivative;

use crate::{CacheData, ForbidFollowsData, FrameStack, LookaheadData, PythonQuoteType};

#[derive(Derivative)]
#[derivative(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RightData {
    #[derivative(Hash = "ignore")]
    pub frame_stack: Option<FrameStack>,
    #[derivative(Hash = "ignore")]
    pub indents: smallvec::SmallVec<[Vec<u8>; 0]>,
    pub dedents: usize,
    pub scope_count: usize,
    pub fstring_start_stack: smallvec::SmallVec<[PythonQuoteType; 0]>,
    #[derivative(Hash = "ignore")]
    pub forbidden_consecutive_matches: ForbidFollowsData,
    #[derivative(PartialEq = "ignore", Hash = "ignore", Debug = "ignore")]
    pub cache_data: CacheData,
    #[derivative(Hash = "ignore")]
    pub lookahead_data: LookaheadData,
    pub position: usize,
}

impl Default for RightData {
    fn default() -> Self {
        Self {
            frame_stack: None,
            indents: vec![].into(),
            dedents: 0,
            scope_count: 0,
            fstring_start_stack: vec![].into(),
            forbidden_consecutive_matches: ForbidFollowsData::default(),
            cache_data: CacheData::default(),
            lookahead_data: LookaheadData::default(),
            position: 0,
        }
    }
}

impl RightData {
    pub fn with_position(mut self, position: usize) -> Self {
        self.position = position;
        self
    }

    pub fn failable(&self) -> bool {
        self.lookahead_data.has_omitted_partial_lookaheads
    }
}