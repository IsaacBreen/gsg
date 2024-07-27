use std::cell::RefCell;
use std::collections::{BTreeSet, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use derivative::Derivative;
use crate::{FrameStack, ForbidConsecutiveMatchesData, U8Set};
use crate::CacheData;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseResults {
    pub right_data_vec: Vec<RightData>,
    pub up_data_vec: Vec<UpData>,
    pub done: bool,
}

impl ParseResults {
    pub fn empty_unfinished() -> Self {
        ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            done: false,
        }
    }
    pub fn empty_finished() -> Self {
        ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            done: true,
        }
    }

    pub(crate) fn combine(&mut self, mut p0: ParseResults) {
        self.right_data_vec.append(&mut p0.right_data_vec);
        self.up_data_vec.append(&mut p0.up_data_vec);
        self.done &= p0.done;
    }

    pub(crate) fn combine_inplace(mut self, p0: ParseResults) -> Self {
        self.combine(p0);
        self
    }
}

#[derive(Derivative)]
#[derivative(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct RightData {
    pub frame_stack: Option<FrameStack>,
    pub indents: Vec<Vec<u8>>,
    pub dedents: usize,
    pub scope_count: usize,
    #[derivative(PartialEq = "ignore", Hash = "ignore", Debug = "ignore")]
    pub forbidden_consecutive_matches: ForbidConsecutiveMatchesData,
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
            forbidden_consecutive_matches: ForbidConsecutiveMatchesData::default(),
            cache_data: CacheData::default(),
            position: 0,
        }
    }
}

impl RightData {
}

pub trait Squash {
    type Output;
    fn squashed(self) -> Self::Output;
    fn squash(&mut self);
}

impl Squash for Vec<RightData> {
    type Output = Vec<RightData>;
    fn squashed(self) -> Self::Output {
        if self.len() > 1 {
            self.into_iter().collect::<BTreeSet<_>>().into_iter().collect()
        } else {
            self
        }
    }
    fn squash(&mut self) {
        if self.len() > 1 {
            *self = self.drain(..).collect::<Self>().squashed()
        }
    }
}

impl Squash for Vec<UpData> {
    type Output = Vec<UpData>;
    fn squashed(self) -> Self::Output {
        let mut u8set = U8Set::none();
        for vd in self {
            u8set = u8set.union(&vd.u8set);
        }
        if u8set.is_empty() {
            vec![]
        } else {
            vec![UpData { u8set }]
        }
    }
    fn squash(&mut self) {
        *self = self.clone().squashed();
    }
}

impl Squash for ParseResults {
    type Output = ParseResults;
    fn squashed(self) -> Self::Output {
        ParseResults {
            right_data_vec: self.right_data_vec.squashed(),
            up_data_vec: self.up_data_vec.squashed(),
            done: self.done,
        }
    }
    fn squash(&mut self) {
        *self = self.clone().squashed();
    }
}