use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;
use derivative::Derivative;

use crate::{ForbidFollowsData, FrameStack, LookaheadData, PythonQuoteType};
use crate::internal_vec::VecZ;
use crate::VecX;

#[repr(packed(1))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Fields1 {
    pub lookahead_data: LookaheadData,
    pub position: usize,
    pub forbidden_consecutive_matches: ForbidFollowsData,
    pub dedents: u8,
    pub scope_count: u8
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fields2 {
    // #[derivative(Hash = "ignore")]
    // pub frame_stack: Option<FrameStack>,
    pub indents: VecZ<Vec<u8>>,
    pub fstring_start_stack: VecZ<PythonQuoteType>
}

#[derive(Derivative)]
#[derivative(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RightDataInner {
    pub fields1: Fields1,
    #[derivative(Hash = "ignore")]
    pub fields2: Rc<Fields2>,
}

#[derive(Derivative)]
#[derivative(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RightData {
    pub right_data_inner: Rc<RightDataInner>,
}

impl Default for RightData {
    fn default() -> Self {
        // Print the size of RightDataInner
        println!("RightDataInner size: {}", std::mem::size_of::<RightDataInner>());
        Self {
            right_data_inner: RightDataInner {
                // frame_stack: None,
                fields2: Fields2 { indents: VecZ::new(), fstring_start_stack: VecZ::new() }.into(),
                fields1: Fields1 { dedents: 0, scope_count: 0, forbidden_consecutive_matches: ForbidFollowsData::default(), lookahead_data: LookaheadData::default(), position: 0 },
            }.into()
        }
    }
}

impl RightData {
    pub fn advance(&mut self, n: usize) {
        Rc::make_mut(&mut self.right_data_inner).fields1.position += n;
    }

    pub fn with_position(mut self, position: usize) -> Self {
        Rc::make_mut(&mut self.right_data_inner).fields1.position = position;
        self
    }

    pub fn failable(&self) -> bool {
        self.right_data_inner.fields1.lookahead_data.has_omitted_partial_lookaheads
    }
}
