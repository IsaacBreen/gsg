use crate::{BaseCombinatorTrait, DynCombinatorTrait, ParserTrait, UnambiguousParseError, UnambiguousParseResults, RightData, UpData, OneShotUpData};
use crate::{CombinatorTrait, FailParser, ParseResultTrait, ParseResults, RightDataGetters};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

pub struct CheckRightData<Output> {
    pub(crate) run: Box<dyn Fn(&RightData) -> bool>,
    pub(crate) _phantom: std::marker::PhantomData<Output>,
}

impl<Output> Hash for CheckRightData<Output> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ptr = std::ptr::addr_of!(self.run) as *const ();
        ptr.hash(state);
    }
}

impl<Output> PartialEq for CheckRightData<Output> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&self.run, &other.run)
    }
}

impl<Output> Eq for CheckRightData<Output> {}

impl<Output> Debug for CheckRightData<Output> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CheckRightData").finish()
    }
}

impl<Output: 'static> DynCombinatorTrait for CheckRightData<Output> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults<Output>) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &'a [u8]) -> UnambiguousParseResults<Output> where Output: 'a {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<Output: 'static> CombinatorTrait for CheckRightData<Output> {
    type Parser<'a> = FailParser where Self: 'a;
    type Output = Output;

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Output> where Output: 'b {
        if (self.run)(&right_data.clone()) {
            Ok(OneShotUpData::new(right_data, Output::default()))
        } else {
            Err(UnambiguousParseError::Fail)
        }
    }

    fn old_parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Output>) where Output: 'b {
        if (self.run)(&right_data.clone()) {
            (FailParser, ParseResults::new_single(UpData::new(right_data, Output::default()), true))
        } else {
            (FailParser, ParseResults::empty_finished())
        }
    }
}

impl<Output> BaseCombinatorTrait for CheckRightData<Output> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

pub fn check_right_data<Output>(run: impl Fn(&RightData) -> bool + 'static) -> CheckRightData<Output> {
    CheckRightData { run: Box::new(run), _phantom: std::marker::PhantomData }
}

pub struct MutateRightData<Output> {
    pub(crate) run: Box<dyn Fn(&mut RightData) -> bool>,
    pub(crate) _phantom: std::marker::PhantomData<Output>,
}

impl<Output> Hash for MutateRightData<Output> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ptr = std::ptr::addr_of!(self.run) as *const ();
        std::ptr::hash(ptr, state);
    }
}

impl<Output> PartialEq for MutateRightData<Output> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&self.run, &other.run)
    }
}

impl<Output> Eq for MutateRightData<Output> {}

impl<Output> Debug for MutateRightData<Output> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MutateRightData").finish()
    }
}

impl<Output: 'static> DynCombinatorTrait for MutateRightData<Output> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults<Output>) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &'a [u8]) -> UnambiguousParseResults<Output> where Output: 'a {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<Output: 'static> CombinatorTrait for MutateRightData<Output> {
    type Parser<'a> = FailParser where Self: 'a;
    type Output = Output;

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Output> where Output: 'b {
        let mut right_data = right_data;
        if (self.run)(&mut right_data) {
            Ok(OneShotUpData::new(right_data, Output::default()))
        } else {
            Err(UnambiguousParseError::Fail)
        }
    }
    fn old_parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Output>) where Output: 'b {
        let mut right_data = right_data;
        if (self.run)(&mut right_data) {
            (FailParser, ParseResults::new_single(UpData::new(right_data, Output::default()), true))
        } else {
            (FailParser, ParseResults::empty_finished())
        }
    }
}

impl<Output> BaseCombinatorTrait for MutateRightData<Output> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

pub fn mutate_right_data<Output>(run: impl Fn(&mut RightData) -> bool + 'static) -> MutateRightData<Output> {
    MutateRightData { run: Box::new(run), _phantom: std::marker::PhantomData }
}