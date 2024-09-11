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

pub trait Squash {
    type Output;
    fn squashed(self) -> Self::Output;
    fn squash(&mut self);
}

impl<Output: Clone> Squash for VecY<UpData<Output>> {
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

impl<Output: Clone> Squash for ParseResults<Output> {
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
pub struct RightDataSquasher<Output: Clone> {
    decomposed: HashMap<RightData, LookaheadData>,
    output_squasher: OutputSquasher<Output>,
}

impl<Output: Clone> Hash for RightDataSquasher<Output> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.decomposed.len().hash(state);
    }
}

impl<Output: Clone> RightDataSquasher<Output> {
    pub fn new() -> Self {
        Self {
            decomposed: HashMap::new(),
            output_squasher: OutputSquasher::new(),
        }
    }

    pub fn push(&mut self, mut right_data: RightData, output: Output) {
        let lookahead_data = std::mem::take(&mut right_data.get_inner_mut().get_fields1_mut().lookahead_data);
        let mut existing_lookahead_data = self.decomposed.entry(right_data).or_insert_with_key(|_| lookahead_data.clone());
        // TODO: In general, all the lookaheads needs to be satisfied, i.e. it's an AND operation between Vecs of lookaheads. But this implies OR.
        existing_lookahead_data.has_omitted_partial_lookaheads &= lookahead_data.has_omitted_partial_lookaheads;
        self.output_squasher.push(output);
    }

    pub fn extend(&mut self, right_data_vec: impl IntoIterator<Item = (RightData, Output)>) {
        profile!("RightDataSquasher::extend", {
        for (right_data, output) in right_data_vec {
            self.push(right_data, output);
        }
            })
    }

    pub fn finish(self) -> VecY<UpData<Output>> {
        profile!("RightDataSquasher::finish", {
            let mut result = VecY::new();
            for (mut right_data, lookahead_data) in self.decomposed {
                right_data.get_inner_mut().get_fields1_mut().lookahead_data = lookahead_data;
                result.push(UpData::new(right_data, self.output_squasher.pop()));
            }
            result
        })
    }
}

impl<Output: Clone> From<Vec<(RightData, Output)>> for RightDataSquasher<Output> {
    fn from(right_data_vec: Vec<(RightData, Output)>) -> Self {
        profile!("RightDataSquasher::from", {
            let mut squasher = RightDataSquasher::new();
            squasher.extend(right_data_vec);
            squasher
        })
    }
}

impl<Output: Clone> RightDataSquasher<Output> {
    pub fn into_iter(self) -> RightDataSquasherIterator<Output> {
        RightDataSquasherIterator {
            inner: self.decomposed.into_iter()
        }
    }
}

pub struct RightDataSquasherIterator<Output: Clone> {
    inner: std::collections::hash_map::IntoIter<RightData, LookaheadData>,
    output_squasher: OutputSquasher<Output>,
}

impl<Output: Clone> Iterator for RightDataSquasherIterator<Output> {
    type Item = (RightData, LookaheadData, Output);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(right_data, lookahead_data)| (right_data, lookahead_data, self.output_squasher.pop()))
    }
}

impl<Output: Clone> IntoIterator for RightDataSquasher<Output> {
    type Item = (RightData, LookaheadData, Output);
    type IntoIter = RightDataSquasherIterator<Output>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

pub struct RightDataSquasherIteratorMut<'a, Output: Clone> {
    inner: std::collections::hash_map::IterMut<'a, RightData, LookaheadData>,
    output_squasher: &'a mut OutputSquasher<Output>,
}

impl<'a, Output: Clone> Iterator for RightDataSquasherIteratorMut<'a, Output> {
    type Item = (&'a RightData, &'a mut LookaheadData, Output);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(right_data, lookahead_data)| (right_data, lookahead_data, self.output_squasher.pop()))
    }
}

impl<Output: Clone> RightDataSquasher<Output> {
    pub fn iter(&self) -> RightDataSquasherIterator<Output> {
        RightDataSquasherIterator {
            inner: self.decomposed.clone().into_iter()
        }
    }

    pub fn iter_mut(&mut self) -> RightDataSquasherIteratorMut<Output> {
        RightDataSquasherIteratorMut {
            inner: self.decomposed.iter_mut(),
            output_squasher: &mut self.output_squasher
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
        for (right_data, _, output) in right_data_squasher.decomposed.drain() {
            self.push(right_data, output);
        }
            })
    }

    pub fn retain(&mut self, f: impl Fn(&RightData) -> bool) {
        self.decomposed.retain(|right_data, _| f(right_data));
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct OutputSquasher<Output: Clone> {
    outputs: Vec<Output>,
}

impl<Output: Clone> OutputSquasher<Output> {
    pub fn new() -> Self {
        Self {
            outputs: Vec::new(),
        }
    }

    pub fn push(&mut self, output: Output) {
        self.outputs.push(output);
    }

    pub fn pop(&mut self) -> Output {
        self.outputs.pop().unwrap()
    }
}

pub trait OutputTrait: Clone + PartialEq + Eq + std::fmt::Debug + std::hash::Hash {}

impl<T: Clone + PartialEq + Eq + std::fmt::Debug + std::hash::Hash> OutputTrait for T {}

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

pub trait CombinatorTrait: BaseCombinatorTrait + DynCombinatorTrait + std::fmt::Debug {
    type Parser<'a>: ParserTrait<Self::Output> where Self: 'a;
    type Output: OutputTrait;

    fn old_parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b;
    fn parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        self.old_parse(right_data, bytes)
    }
    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b;
}

pub trait DynCombinatorTrait: BaseCombinatorTrait + std::fmt::Debug {
    fn parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Box<dyn ParserTrait<Self::Output> + 'a>, ParseResults<Self::Output>) where Self::Output: 'b;
    fn one_shot_parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b;
}

pub trait BaseCombinatorTrait {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static;
    fn type_name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {}
    fn compile(mut self) -> Self
    where
        Self: Sized
    {
        self.compile_inner();
        self
    }
    fn compile_inner(&self) {
        self.apply_to_children(&mut |combinator| combinator.compile_inner());
    }
}

pub fn dumb_one_shot_parse<T: CombinatorTrait>(combinator: &T, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults<T::Output> {
    let (parser, parse_results) = combinator.old_parse(right_data, bytes);
    UnambiguousParseResults::from(parse_results)
}

pub trait ParserTrait<Output: OutputTrait>: std::fmt::Debug {
    fn get_u8set(&self) -> U8Set;
    fn parse<'b>(&mut self, bytes: &'b [u8]) -> ParseResults<Output> where Output: 'b;
    fn autoparse(&mut self, right_data: RightData, max_length: usize) -> (Vec<u8>, ParseResults<Output>) {
        let mut prefix = Vec::new();
        let mut parse_results = ParseResults::empty_finished();
        while prefix.len() < max_length {
            let u8set = self.get_u8set();
            if u8set.len() == 1 {
                let c = u8set.iter().next().unwrap();
                let new_parse_results = self.parse(&[c]);
                parse_results.combine_seq(new_parse_results);
                prefix.push(c);
            } else {
                break;
            }
        }
        (prefix, parse_results)
    }
}

impl<T: DynCombinatorTrait + ?Sized> DynCombinatorTrait for Box<T> {
    fn parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Box<dyn ParserTrait<Self::Output> + 'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        (**self).parse_dyn(right_data, bytes)
    }

    fn one_shot_parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b {
        (**self).one_shot_parse_dyn(right_data, bytes)
    }
}

impl<T: CombinatorTrait + ?Sized> CombinatorTrait for Box<T> {
    type Parser<'a> = T::Parser<'a> where Self: 'a;
    type Output = T::Output;

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b {
        (**self).one_shot_parse(right_data, bytes)
    }

    fn old_parse<'a, 'b>(&self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        (**self).old_parse(right_data, bytes)
    }

    fn parse<'a, 'b>(&self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        (**self).parse(right_data, bytes)
    }

}

impl<T: BaseCombinatorTrait + ?Sized> BaseCombinatorTrait for Box<T> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        (**self).as_any()
    }
    fn type_name(&self) -> &str {
        (**self).type_name()
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        (**self).apply_to_children(f);
    }
    fn compile_inner(&self) {
        (**self).compile_inner();
    }
}

impl<'a> ParserTrait<Output> for Box<dyn ParserTrait<Output> + 'a> where Output: OutputTrait {
    fn get_u8set(&self) -> U8Set {
        (**self).get_u8set()
    }

    fn parse<'b>(&mut self, bytes: &'b [u8]) -> ParseResults<Output> where Output: 'b {
        (**self).parse(bytes)
    }
}

impl<'b> CombinatorTrait for Box<dyn DynCombinatorTrait + 'b> {
    type Parser<'a> = Box<dyn ParserTrait<Self::Output> + 'a> where Self: 'a;
    type Output = Box<dyn std::any::Any>;

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Self::Output> where Self::Output: 'b {
        (**self).one_shot_parse_dyn(right_data, bytes)
    }

    fn old_parse<'a, 'b>(&self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Self::Output>) where Self::Output: 'b {
        (**self).parse_dyn(right_data, bytes)
    }
}

// Removed ParserTrait implementation for Parser enum

pub trait CombinatorTraitExt: CombinatorTrait {
    fn parser(&self, right_data: RightData) -> (Self::Parser<'_>, ParseResults<Self::Output>) {
        self.old_parse(right_data, &[])
    }
}

pub trait ParserTraitExt: ParserTrait<Output> where Output: OutputTrait {
    fn step(&mut self, c: u8) -> ParseResults<Output> {
        self.parse(&[c])
    }
}

impl<T: CombinatorTrait> CombinatorTraitExt for T {}
impl<T: ParserTrait<Output>> ParserTraitExt for T where Output: OutputTrait {}