use crate::BaseCombinatorTrait;
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct ForbidFollowsData {
    pub prev_match_ids: u16, // Using a bitset
}

#[derive(Debug)]
pub struct ForbidFollows<Output> {
    pub(crate) match_ids: u16, // Using a bitset
    pub(crate) _phantom: std::marker::PhantomData<Output>,
}

#[derive(Debug)]
pub struct ForbidFollowsClear<Output> {
    pub(crate) _phantom: std::marker::PhantomData<Output>,
}

#[derive(Debug)]
pub struct ForbidFollowsCheckNot<Output> {
    pub(crate) match_ids: u16,
    pub(crate) _phantom: std::marker::PhantomData<Output>,
}

impl<Output: 'static> DynCombinatorTrait for ForbidFollows<Output> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults<Output>) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &'a [u8]) -> UnambiguousParseResults<Output> where Output: 'a {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<Output: 'static> CombinatorTrait for ForbidFollows<Output> {
    type Parser<'a> = FailParser where Self: 'a;
    type Output = Output;

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Output> where Output: 'b {
        let mut right_data = right_data;
        right_data.get_inner_mut().get_fields1_mut().forbidden_consecutive_matches.prev_match_ids = self.match_ids;
        Ok(OneShotUpData::new(right_data, Output::default()))
    }
    fn old_parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Output>) where Output: 'b {
        let mut right_data = right_data;
        right_data.get_inner_mut().get_fields1_mut().forbidden_consecutive_matches.prev_match_ids = self.match_ids;
        (FailParser, ParseResults::new_single(UpData::new(right_data, Output::default()), true))
    }
}

impl<Output> BaseCombinatorTrait for ForbidFollows<Output> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

impl<Output: 'static> DynCombinatorTrait for ForbidFollowsClear<Output> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults<Output>) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &'a [u8]) -> UnambiguousParseResults<Output> where Output: 'a {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<Output: 'static> CombinatorTrait for ForbidFollowsClear<Output> {
    type Parser<'a> = FailParser where Self: 'a;
    type Output = Output;

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Output> where Output: 'b {
        Ok(OneShotUpData::new(right_data, Output::default()))
    }
    fn old_parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Output>) where Output: 'b {
        let mut right_data = right_data;
        right_data.get_inner_mut().get_fields1_mut().forbidden_consecutive_matches.prev_match_ids = 0;
        (FailParser, ParseResults::new_single(UpData::new(right_data, Output::default()), true))
    }
}

impl<Output> BaseCombinatorTrait for ForbidFollowsClear<Output> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

impl<Output: 'static> DynCombinatorTrait for ForbidFollowsCheckNot<Output> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults<Output>) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &'a [u8]) -> UnambiguousParseResults<Output> where Output: 'a {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<Output: 'static> CombinatorTrait for ForbidFollowsCheckNot<Output> {
    type Parser<'a> = FailParser where Self: 'a;
    type Output = Output;

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Output> where Output: 'b {
        Ok(OneShotUpData::new(right_data, Output::default()))
    }
    fn old_parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Output>) where Output: 'b {
        let mut right_data = right_data;
        if right_data.get_fields1().forbidden_consecutive_matches.prev_match_ids & self.match_ids != 0 {
            (FailParser, ParseResults::empty_finished())
        } else {
            right_data.get_inner_mut().get_fields1_mut().forbidden_consecutive_matches.prev_match_ids = 0;
            (FailParser, ParseResults::new_single(UpData::new(right_data, Output::default()), true))
        }
    }
}

impl<Output> BaseCombinatorTrait for ForbidFollowsCheckNot<Output> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

pub fn forbid_follows<Output>(match_ids: &[usize]) -> ForbidFollows<Output> { // Using a bitset
    let mut bitset = 0;
    for &id in match_ids {
        bitset |= 1 << TryInto::<u16>::try_into(id).unwrap();
    }
    ForbidFollows { match_ids: bitset, _phantom: std::marker::PhantomData }
}

pub fn forbid_follows_clear<Output>() -> ForbidFollowsClear<Output> {
    ForbidFollowsClear { _phantom: std::marker::PhantomData }
}

pub fn forbid_follows_check_not<Output>(match_id: usize) -> ForbidFollowsCheckNot<Output> {
    let bitmask = 1 << match_id;
    ForbidFollowsCheckNot { match_ids: bitmask, _phantom: std::marker::PhantomData }
}