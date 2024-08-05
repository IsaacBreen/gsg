use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use crate::{LookaheadData, ParseResults, RightData, U8Set};

const SQUASH_THRESHOLD: usize = 0;

pub trait Squash {
    type Output;
    fn squashed(self) -> Self::Output;
    fn squash(&mut self);
}

impl Squash for Vec<RightData> {
    type Output = Vec<RightData>;
    fn squashed(self) -> Self::Output {
        if self.len() > SQUASH_THRESHOLD {
            let mut squasher = RightDataSquasher::new();
            squasher.extend(self);
            squasher.finish()
        } else {
            self
        }
    }
    fn squash(&mut self) {
        if self.len() > SQUASH_THRESHOLD {
            *self = self.drain(..).collect::<Self>().squashed()
        }
    }
}

impl Squash for ParseResults {
    type Output = ParseResults;
    fn squashed(self) -> Self::Output {
        ParseResults {
            right_data_vec: self.right_data_vec,
            done: self.done,
        }
    }
    fn squash(&mut self) {
        *self = self.clone().squashed();
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RightDataSquasher {
    decomposed: HashMap<RightData, LookaheadData>,
}

impl Hash for RightDataSquasher {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.decomposed.len().hash(state);
    }
}

impl RightDataSquasher {
    pub fn new() -> Self {
        Self {
            decomposed: HashMap::new(),
        }
    }
}

impl RightDataSquasher {
    pub fn push(&mut self, right_data: RightData) {
        let lookahead_data = right_data.lookahead_data.clone();
        let mut existing_lookahead_data = self.decomposed.entry(right_data).or_default();
        // TODO: In general, all the lookaheads needs to be satisfied, i.e. it's an AND operation between Vecs of lookaheads. But this implies OR.
        existing_lookahead_data.partial_lookaheads.extend(lookahead_data.partial_lookaheads);
        existing_lookahead_data.has_omitted_partial_lookaheads &= lookahead_data.has_omitted_partial_lookaheads;
    }

    pub fn extend(&mut self, right_data_vec: impl Into<RightDataSquasher>) {
        for right_data in right_data_vec.into().iter() {
            self.push(right_data);
        }
    }

    pub fn finish(self) -> Vec<RightData> {
        let mut result = vec![];
        for (mut right_data, lookahead_data) in self.decomposed {
            right_data.lookahead_data = lookahead_data;
            result.push(right_data);
        }
        result
    }
}

impl From<Vec<RightData>> for RightDataSquasher {
    fn from(right_data_vec: Vec<RightData>) -> Self {
        let mut squasher = RightDataSquasher::new();
        squasher.extend(right_data_vec);
        squasher
    }
}

impl RightDataSquasher {
    pub fn into_iter(self) -> RightDataSquasherIterator {
        RightDataSquasherIterator {
            inner: self.decomposed.into_iter()
        }
    }
}

pub struct RightDataSquasherIterator {
    inner: std::collections::hash_map::IntoIter<RightData, LookaheadData>,
}

impl Iterator for RightDataSquasherIterator {
    type Item = RightData;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(mut right_data, lookahead_data)| {
            right_data.lookahead_data = lookahead_data;
            right_data
        })
    }
}

impl IntoIterator for RightDataSquasher {
    type Item = RightData;
    type IntoIter = RightDataSquasherIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

pub struct RightDataSquasherIteratorMut<'a> {
    inner: std::collections::hash_map::IterMut<'a, RightData, LookaheadData>,
}

impl<'a> Iterator for RightDataSquasherIteratorMut<'a> {
    type Item = &'a mut RightData;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl RightDataSquasher {
    pub fn iter(&self) -> RightDataSquasherIterator {
        RightDataSquasherIterator {
            inner: self.decomposed.clone().into_iter()
        }
    }

    pub fn iter_mut(&mut self) -> RightDataSquasherIteratorMut {
        RightDataSquasherIteratorMut {
            inner: self.decomposed.iter_mut()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.decomposed.is_empty()
    }

    pub fn len(&self) -> usize {
        self.decomposed.len()
    }

    pub fn clear(&mut self) {
        self.decomposed.clear();
    }

    pub fn append(&mut self, right_data_squasher: &mut RightDataSquasher) {
        for right_data in std::mem::take(right_data_squasher) {
            self.push(right_data);
        }
    }

    pub fn retain(&mut self, f: impl Fn(&RightData) -> bool) {
        todo!()
    }

    pub fn retain_mut(&mut self, f: impl FnMut(&mut RightData)) {
        todo!()
    }
}
