use crate::RightData;
use crate::{profile, LookaheadData, ParseResultTrait, ParseResults, VecY};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

// macro_rules! profile {
//     ($tag:expr, $body:expr) => {{
//         $crate::profile!($tag, $body)
//     }};
// }

// macro_rules! profile {
//     ($tag:expr, $body:expr) => {{
//         $body
//     }};
// }

const SQUASH_THRESHOLD: usize = 1;

pub trait Squash {
    type Output;
    fn squashed(self) -> Self::Output;
    fn squash(&mut self);
}

impl Squash for VecY<RightData> {
    type Output = VecY<RightData>;
    fn squashed(self) -> Self::Output {
        if self.len() > SQUASH_THRESHOLD {
            profile!("RightDataSquasher::squashed", {
                // let mut squasher = RightDataSquasher::new();
                // squasher.extend(self.into_iter());
                // squasher.finish()
                self.into_iter().collect::<HashSet<_>>().into_iter().collect()
            })
        } else {
            self
        }
    }
    fn squash(&mut self) {
        if self.len() > SQUASH_THRESHOLD {
            *self = self.drain(..).collect::<VecY<RightData>>().squashed()
        }
    }
}

impl Squash for ParseResults {
    type Output = ParseResults;
    fn squashed(self) -> Self::Output {
        if self.right_data_vec.len() > SQUASH_THRESHOLD {
            profile!("ParseResults::squashed", {
                    let done = self.done();
                    ParseResults::new(self.right_data_vec.squashed(), done)
                }
            )
        } else {
            self
        }
    }
    fn squash(&mut self) {
        profile!("ParseResults::squash", {
        if self.right_data_vec.len() > SQUASH_THRESHOLD {
            // *self.right_data_vec = std::mem::take(&mut self.right_data_vec).squashed();
            *self = self.clone().squashed();
        }
            })
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

    pub fn push(&mut self, mut right_data: RightData) {
        let lookahead_data = std::mem::take(&mut right_data.get_inner_mut().fields1.lookahead_data);
        let mut existing_lookahead_data = self.decomposed.entry(right_data).or_insert_with_key(|_| lookahead_data.clone());
        // TODO: In general, all the lookaheads needs to be satisfied, i.e. it's an AND operation between Vecs of lookaheads. But this implies OR.
        existing_lookahead_data.has_omitted_partial_lookaheads &= lookahead_data.has_omitted_partial_lookaheads;
    }

    pub fn extend(&mut self, right_data_vec: impl IntoIterator<Item = RightData>) {
        profile!("RightDataSquasher::extend", {
        for right_data in right_data_vec {
            self.push(right_data);
        }
            })
    }

    pub fn finish(self) -> VecY<RightData> {
        profile!("RightDataSquasher::finish", {
            let mut result = VecY::new();
            for (mut right_data, lookahead_data) in self.decomposed {
                right_data.get_inner_mut().fields1.lookahead_data = lookahead_data;
                result.push(right_data);
            }
            result
        })
    }
}

impl From<Vec<RightData>> for RightDataSquasher {
    fn from(right_data_vec: Vec<RightData>) -> Self {
        profile!("RightDataSquasher::from", {
            let mut squasher = RightDataSquasher::new();
            squasher.extend(right_data_vec);
            squasher
        })
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
    type Item = (RightData, LookaheadData);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl IntoIterator for RightDataSquasher {
    type Item = (RightData, LookaheadData);
    type IntoIter = RightDataSquasherIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

pub struct RightDataSquasherIteratorMut<'a> {
    inner: std::collections::hash_map::IterMut<'a, RightData, LookaheadData>,
}

impl<'a> Iterator for RightDataSquasherIteratorMut<'a> {
    type Item = (&'a RightData, &'a mut LookaheadData);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
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
        profile!("RightDataSquasher::append", {
        for right_data in right_data_squasher.decomposed.drain() {
            self.push(right_data.0);
        }
            })
    }

    pub fn retain(&mut self, f: impl Fn(&RightData) -> bool) {
        self.decomposed.retain(|right_data, _| f(right_data));
    }
}
