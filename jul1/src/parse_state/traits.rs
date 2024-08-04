use std::collections::{HashMap, HashSet};

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
            let mut decomposed: HashMap<RightData, LookaheadData> = HashMap::new();
            for mut right_data in self {
                let lookahead_data = std::mem::take(&mut right_data.lookahead_data);
                let mut existing_lookahead_data = decomposed.entry(right_data).or_default();
                // TODO: In general, all the lookaheads needs to be satisfied, i.e. it's an AND operation between Vecs of lookaheads. But this implies OR.
                existing_lookahead_data.partial_lookaheads.extend(lookahead_data.partial_lookaheads);
                existing_lookahead_data.has_omitted_partial_lookaheads &= lookahead_data.has_omitted_partial_lookaheads;
            }
            let mut result = vec![];
            for (mut right_data, lookahead_data) in decomposed {
                right_data.lookahead_data = lookahead_data;
                result.push(right_data);
            }
            result
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
            right_data_vec: self.right_data_vec.squashed(),
            done: self.done,
        }
    }
    fn squash(&mut self) {
        *self = self.clone().squashed();
    }
}