use derivative::Derivative;
use crate::{FrameStack, ForbidFollowsData, CacheData, U8Set};

#[derive(Derivative)]
#[derivative(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct RightData {
    pub frame_stack: Option<FrameStack>,
    pub indents: Vec<Vec<u8>>, 
    pub dedents: usize,
    pub scope_count: usize,
    #[derivative(PartialEq = "ignore", Hash = "ignore", Debug = "ignore")]
    pub forbidden_consecutive_matches: ForbidFollowsData,
    pub cache_data: CacheData,
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    pub position: usize,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct UpData {
    pub u8set: U8Set,
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
        }
    }
}

impl RightData {}
