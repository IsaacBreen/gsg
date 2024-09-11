use crate::{RightData, RightDataGetters};
use crate::{profile, LookaheadData, ParseResultTrait, ParseResults, UpData, VecY};
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

pub trait Squash<Output> {
    type Output;
    fn squashed(self) -> Self::Output;
    fn squash(&mut self);
}

impl<Output> Squash<VecY<UpData<Output>>> for VecY<UpData<Output>> {
    type Output = VecY<UpData<Output>>;
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
            *self = self.drain(..).collect::<VecY<UpData<Output>>>().squashed()
        }
    }
}

impl<Output> Squash<ParseResults<Output>> for ParseResults<Output> {
    type Output = ParseResults<Output>;
    fn squashed(self) -> Self::Output {
        if self.up_data_vec.len() > SQUASH_THRESHOLD {
            profile!("ParseResults::squashed", {
                    let done = self.done();
                    ParseResults::new(self.up_data_vec.squashed(), done)
                }
            )
        } else {
            self
        }
    }
    fn squash(&mut self) {
        profile!("ParseResults::squash", {
        if self.up_data_vec.len() > SQUASH_THRESHOLD {
            // *self.right_data_vec = std::mem::take(&mut self.right_data_vec).squashed();
            *self = self.clone().squashed();
        }
            })
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RightDataSquasher<Output> {
    decomposed: HashMap<RightData, LookaheadData>,
}

impl<Output> Hash for RightDataSquasher<Output> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.decomposed.len().hash(state);
    }
}

impl<Output> RightDataSquasher<Output> {
    pub fn new() -> Self {
        Self {
            decomposed: HashMap::new(),
        }
    }

    pub fn push(&mut self, mut right_data: RightData) {
        let lookahead_data = std::mem::take(&mut right_data.get_inner_mut().get_fields1_mut().lookahead_data);
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

    pub fn finish(self) -> VecY<UpData<Output>> {
        profile!("RightDataSquasher::finish", {
            let mut result = VecY::new();
            for (mut right_data, lookahead_data) in self.decomposed {
                right_data.get_inner_mut().get_fields1_mut().lookahead_data = lookahead_data;
                result.push(UpData::new(right_data, Output::default()));
            }
            result
        })
    }
}

impl<Output> From<Vec<RightData>> for RightDataSquasher<Output> {
    fn from(right_data_vec: Vec<RightData>) -> Self {
        profile!("RightDataSquasher::from", {
            let mut squasher = RightDataSquasher::new();
            squasher.extend(right_data_vec);
            squasher
        })
    }
}

impl<Output> RightDataSquasher<Output> {
    pub fn into_iter(self) -> RightDataSquasherIterator<Output> {
        RightDataSquasherIterator {
            inner: self.decomposed.into_iter()
        }
    }
}

pub struct RightDataSquasherIterator<Output> {
    inner: std::collections::hash_map::IntoIter<RightData, LookaheadData>,
}

impl<Output> Iterator for RightDataSquasherIterator<Output> {
    type Item = (RightData, LookaheadData);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<Output> IntoIterator for RightDataSquasher<Output> {
    type Item = (RightData, LookaheadData);
    type IntoIter = RightDataSquasherIterator<Output>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

pub struct RightDataSquasherIteratorMut<'a, Output> {
    inner: std::collections::hash_map::IterMut<'a, RightData, LookaheadData>,
}

impl<'a, Output> Iterator for RightDataSquasherIteratorMut<'a, Output> {
    type Item = (&'a RightData, &'a mut LookaheadData);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<Output> RightDataSquasher<Output> {
    pub fn iter(&self) -> RightDataSquasherIterator<Output> {
        RightDataSquasherIterator {
            inner: self.decomposed.clone().into_iter()
        }
    }

    pub fn iter_mut(&mut self) -> RightDataSquasherIteratorMut<Output> {
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

    pub fn append(&mut self, right_data_squasher: &mut RightDataSquasher<Output>) {
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

pub trait OutputTrait: Default + Clone + PartialEq + Eq + Hash + Debug {}

pub trait ParseResultTrait<Output: OutputTrait> {
    fn done(&self) -> bool;
    fn succeeds_decisively(&self) -> bool;
    fn merge_assign(&mut self, p0: Self) where Self: Sized;
    fn merge(self, p0: Self) -> Self where Self: Sized;
    fn combine_seq(&mut self, p0: Self) where Self: Sized;
    fn new(up_data_vec: VecY<UpData<Output>>, done: bool) -> Self where Self: Sized;
    fn new_single(up_data_vec: UpData<Output>, done: bool) -> Self where Self: Sized;
    fn empty(done: bool) -> Self where Self: Sized;
    fn empty_unfinished() -> Self where Self: Sized;
    fn empty_finished() -> Self where Self: Sized;
}