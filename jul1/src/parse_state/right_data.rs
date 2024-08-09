use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;
use derivative::Derivative;

use crate::{ForbidFollowsData, FrameStack, LookaheadData, PythonQuoteType};
use crate::VecX;

#[derive(Derivative)]
#[derivative(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RightDataInner {
    #[derivative(Hash = "ignore")]
    pub frame_stack: Option<FrameStack>,
    #[derivative(Hash = "ignore")]
    pub indents: VecX<Vec<u8>>,
    pub dedents: usize,
    pub scope_count: usize,
    pub fstring_start_stack: VecX<PythonQuoteType>,
    #[derivative(Hash = "ignore")]
    pub forbidden_consecutive_matches: ForbidFollowsData,
    #[derivative(Hash = "ignore")]
    pub lookahead_data: LookaheadData,
    pub position: usize
}

#[derive(Derivative)]
#[derivative(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RightData {
    pub right_data_inner: Rc<RightDataInner>,
}

impl Default for RightData {
    fn default() -> Self {
        Self {
            right_data_inner: RightDataInner {
                frame_stack: None,
                indents: VecX::new(),
                dedents: 0,
                scope_count: 0,
                fstring_start_stack: VecX::new(),
                forbidden_consecutive_matches: ForbidFollowsData::default(),
                lookahead_data: LookaheadData::default(),
                position: 0,
            }.into()
        }
    }
}

impl RightData {
    pub fn with_position(mut self, position: usize) -> Self {
        Rc::make_mut(&mut self.right_data_inner).position = position;
        self
    }

    pub fn failable(&self) -> bool {
        self.right_data_inner.lookahead_data.has_omitted_partial_lookaheads
    }
}
