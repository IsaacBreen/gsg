use std::cell::{Ref, RefCell, RefMut};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use derivative::Derivative;

use crate::{CacheData, ForbidFollowsData, FrameStack, LookaheadData, PythonQuoteType};
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
    #[derivative(PartialEq = "ignore", Hash = "ignore", Debug = "ignore")]
    pub cache_data: CacheData,
    #[derivative(Hash = "ignore")]
    pub lookahead_data: LookaheadData,
    pub position: usize
}

#[derive(Derivative)]
#[derivative(Debug, Clone, Eq)]
pub struct RightData {
    pub right_data_inner: Rc<RefCell<RightDataInner>>,
}

impl Hash for RightData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.borrow().hash(state);
    }
}

impl PartialEq for RightData {
    fn eq(&self, other: &Self) -> bool {
        self.borrow().eq(&other.borrow())
    }
}

impl Default for RightData {
    fn default() -> Self {
        Self {
            right_data_inner: Rc::new(RefCell::new(RightDataInner {
                frame_stack: None,
                indents: VecX::new(),
                dedents: 0,
                scope_count: 0,
                fstring_start_stack: VecX::new(),
                forbidden_consecutive_matches: ForbidFollowsData::default(),
                cache_data: CacheData::default(),
                lookahead_data: LookaheadData::default(),
                position: 0,
            }))
        }
    }
}

impl RightData {
    pub fn with_position(mut self, position: usize) -> Self {
        self.borrow_mut().position = position;
        self
    }

    pub fn failable(&self) -> bool {
        self.borrow().lookahead_data.has_omitted_partial_lookaheads
    }

    pub fn borrow(&self) -> Ref<'_, RightDataInner> {
        self.right_data_inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, RightDataInner> {
        self.right_data_inner.borrow_mut()
    }
}
