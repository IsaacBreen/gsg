use derivative::Derivative;
use std::rc::Rc;

use crate::internal_vec::VecZ;
use crate::{ForbidFollowsData, LookaheadData, PythonQuoteType};

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
    fields1: Fields1,
    #[derivative(Hash = "ignore")]
    fields2: Rc<Fields2>,
    // pub fields2: Box<Fields2>,
    // pub fields2: Fields2,
}

#[derive(Derivative)]
#[derivative(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RightData {
    right_data_inner: Rc<RightDataInner>,
    // pub right_data_inner: Box<RightDataInner>,
    // pub right_data_inner: RightDataInner,
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
        self.get_inner_mut().fields1.position += n;
    }

    pub fn with_position(mut self, position: usize) -> Self {
        self.get_inner_mut().fields1.position = position;
        self
    }

    pub fn failable(&self) -> bool {
        self.right_data_inner.fields1.lookahead_data.has_omitted_partial_lookaheads
    }

    pub fn get_inner_mut(&mut self) -> &mut RightDataInner {
        Rc::make_mut(&mut self.right_data_inner)
        // &mut *self.right_data_inner
        // &mut self.right_data_inner
    }
}

pub trait RightDataGetters {
    fn get_fields1(&self) -> &Fields1;
    fn get_fields1_mut(&mut self) -> &mut Fields1;
    fn get_fields2(&self) -> &Fields2;
    fn get_fields2_mut(&mut self) -> &mut Fields2;
}

impl RightDataGetters for RightDataInner {
    fn get_fields1(&self) -> &Fields1 {
        &self.fields1
    }

    fn get_fields1_mut(&mut self) -> &mut Fields1 {
        &mut self.fields1
    }

    fn get_fields2(&self) -> &Fields2 {
        &self.fields2
    }

    fn get_fields2_mut(&mut self) -> &mut Fields2 {
        Rc::make_mut(&mut self.fields2)
        // &mut *self.fields2
        // &mut self.fields2
    }
}

impl RightDataGetters for RightData {
    fn get_fields1(&self) -> &Fields1 {
        self.right_data_inner.get_fields1()
    }

    fn get_fields1_mut(&mut self) -> &mut Fields1 {
        Rc::make_mut(&mut self.right_data_inner).get_fields1_mut()
    }

    fn get_fields2(&self) -> &Fields2 {
        self.right_data_inner.get_fields2()
    }

    fn get_fields2_mut(&mut self) -> &mut Fields2 {
        Rc::make_mut(&mut self.right_data_inner).get_fields2_mut()
    }
}