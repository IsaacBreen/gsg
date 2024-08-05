use std::collections::HashSet;
use crate::{LookaheadData, ParseResults, profile, RightData};

pub trait Squash {
    type Output;
    fn squashed(self) -> Self::Output;
    fn squash(&mut self);
}

impl Squash for Vec<RightData> {
    type Output = Vec<RightData>;
    fn squashed(mut self) -> Self::Output {
        self.squash();
        self
    }
    fn squash(&mut self) {
        *self = self.drain(..).collect::<HashSet<_>>().into_iter().collect();
    }
}

impl Squash for ParseResults {
    type Output = ParseResults;
    fn squashed(mut self) -> Self::Output {
        self.right_data_vec.squash();
        self
    }
    fn squash(&mut self) {
        self.right_data_vec.squash();
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct RightDataSquasher {
    data: Vec<RightData>,
}

impl RightDataSquasher {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, right_data: RightData) {
        self.data.push(right_data);
    }

    pub fn extend(&mut self, right_data_vec: Vec<RightData>) {
        self.data.extend(right_data_vec);
    }

    pub fn finish(self) -> Vec<RightData> {
        self.data
    }
}

impl From<Vec<RightData>> for RightDataSquasher {
    fn from(right_data_vec: Vec<RightData>) -> Self {
        Self { data: right_data_vec }
    }
}

impl RightDataSquasher {
    pub fn into_iter(self) -> std::vec::IntoIter<RightData> {
        self.data.into_iter()
    }
}

pub type RightDataSquasherIterator = std::vec::IntoIter<RightData>;

impl IntoIterator for RightDataSquasher {
    type Item = RightData;
    type IntoIter = RightDataSquasherIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

pub type RightDataSquasherIteratorMut<'a> = std::slice::IterMut<'a, RightData>;

impl RightDataSquasher {
    pub fn iter(&self) -> std::slice::Iter<RightData> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> RightDataSquasherIteratorMut {
        self.data.iter_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn append(&mut self, right_data_squasher: &mut RightDataSquasher) {
        self.data.append(&mut right_data_squasher.data);
    }

    pub fn retain(&mut self, f: impl Fn(&RightData) -> bool) {
        self.data.retain(f);
    }
}