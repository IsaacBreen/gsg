use std::cell::RefCell;
use std::rc::Rc;
use derivative::Derivative;

use crate::{CacheData, ForbidFollowsData, FrameStack};

#[derive(Derivative)]
#[derivative(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RightData {
    #[derivative(Hash = "ignore")]
    pub frame_stack: Option<FrameStack>,
    #[derivative(Hash = "ignore")]
    pub indents: Vec<Vec<u8>>,
    pub dedents: usize,
    pub scope_count: usize,
    #[derivative(Hash = "ignore")]
    pub forbidden_consecutive_matches: ForbidFollowsData,
    #[derivative(PartialEq = "ignore", Hash = "ignore", Debug = "ignore")]
    pub cache_data: CacheData,
    pub position: usize,
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    pub time: Rc<RefCell<u128>>,
}

impl Default for RightData {
    fn default() -> Self {
        Self {
            frame_stack: Some(FrameStack::default()),
            indents: vec![],
            dedents: 0,
            scope_count: 0,
            forbidden_consecutive_matches: ForbidFollowsData::default(),
            cache_data: CacheData::default(),
            position: 0,
            time: Rc::new(RefCell::new(0)),
        }
    }
}