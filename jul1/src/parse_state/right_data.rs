use std::cell::RefCell;
use std::rc::Rc;
use derivative::Derivative;

use crate::{CacheData, CacheFirstData, ForbidFollowsData, FrameStack, LookaheadData, PythonQuoteType};

#[derive(Derivative)]
#[derivative(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RightData {
    #[derivative(Hash = "ignore")]
    pub frame_stack: Option<FrameStack>,
    #[derivative(Hash = "ignore")]
    pub indents: Vec<Vec<u8>>,
    pub dedents: usize,
    pub scope_count: usize,
    pub fstring_start_stack: Vec<PythonQuoteType>,
    #[derivative(Hash = "ignore")]
    pub forbidden_consecutive_matches: ForbidFollowsData,
    #[derivative(PartialEq = "ignore", Hash = "ignore", Debug = "ignore")]
    pub cache_data: CacheData,
    #[derivative(PartialEq = "ignore", Hash = "ignore", Debug = "ignore")]
    pub cache_first_data: CacheFirstData,
    #[derivative(Hash = "ignore")]
    pub lookahead_data: LookaheadData,
    pub position: usize,
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    pub time: Rc<RefCell<u128>>,
}

impl Default for RightData {
    fn default() -> Self {
        Self {
            frame_stack: None,
            indents: vec![],
            dedents: 0,
            scope_count: 0,
            fstring_start_stack: vec![],
            forbidden_consecutive_matches: ForbidFollowsData::default(),
            cache_data: CacheData::default(),
            cache_first_data: CacheFirstData::default(),
            lookahead_data: LookaheadData::default(),
            position: 0,
            time: Rc::new(RefCell::new(0)),
        }
    }
}

impl RightData {
    pub fn with_position(mut self, position: usize) -> Self {
        self.position = position;
        self
    }

    pub fn failable(&self) -> bool {
        !self.lookahead_data.partial_lookaheads.is_empty() || self.lookahead_data.has_omitted_partial_lookaheads
    }
}