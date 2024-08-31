
// src/_03_combinators/nullable/forbid_follows.rs
use crate::BaseCombinatorTrait;
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct ForbidFollowsData {
    pub prev_match_ids: u16, // Using a bitset
}

#[derive(Debug)]
pub struct ForbidFollows {
    pub(crate) match_ids: u16, // Using a bitset
}

#[derive(Debug)]
pub struct ForbidFollowsClear {}

#[derive(Debug)]
pub struct ForbidFollowsCheckNot {
    pub(crate) match_ids: u16,
}

impl DynCombinatorTrait for ForbidFollows {
    fn parse_dyn(&self, down_data: DownData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(down_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(down_data, bytes)
    }
}

impl CombinatorTrait for ForbidFollows {
    type Parser<'a> = FailParser;
    type Output = ();
    type PartialOutput = ();

    fn one_shot_parse(&self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        let mut right_data = down_data.right_data;
        right_data.get_inner_mut().get_fields1_mut().forbidden_consecutive_matches.prev_match_ids = self.match_ids;
        Ok(OneShotUpData { right_data })
    }
    fn old_parse(&self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let mut right_data = down_data.right_data;
        right_data.get_inner_mut().get_fields1_mut().forbidden_consecutive_matches.prev_match_ids = self.match_ids;
        (FailParser, ParseResults::new_single(UpData { right_data }, true))
    }
}

impl BaseCombinatorTrait for ForbidFollows {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

impl DynCombinatorTrait for ForbidFollowsClear {
    fn parse_dyn(&self, down_data: DownData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(down_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(down_data, bytes)
    }
}

impl CombinatorTrait for ForbidFollowsClear {
    type Parser<'a> = FailParser;
    type Output = ();
    type PartialOutput = ();

    fn one_shot_parse(&self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        Ok(OneShotUpData { right_data: down_data.right_data })
    }
    fn old_parse(&self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let mut right_data = down_data.right_data;
        right_data.get_inner_mut().get_fields1_mut().forbidden_consecutive_matches.prev_match_ids = 0;
        (FailParser, ParseResults::new_single(UpData { right_data }, true))
    }
}

impl BaseCombinatorTrait for ForbidFollowsClear {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

impl DynCombinatorTrait for ForbidFollowsCheckNot {
    fn parse_dyn(&self, down_data: DownData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(down_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(down_data, bytes)
    }
}

impl CombinatorTrait for ForbidFollowsCheckNot {
    type Parser<'a> = FailParser;
    type Output = ();
    type PartialOutput = ();

    fn one_shot_parse(&self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        Ok(OneShotUpData { right_data: down_data.right_data })
    }
    fn old_parse(&self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let mut right_data = down_data.right_data;
        if right_data.get_fields1().forbidden_consecutive_matches.prev_match_ids & self.match_ids != 0 {
            (FailParser, ParseResults::empty_finished())
        } else {
            right_data.get_inner_mut().get_fields1_mut().forbidden_consecutive_matches.prev_match_ids = 0;
            (FailParser, ParseResults::new_single(UpData { right_data }, true))
        }
    }
}

impl BaseCombinatorTrait for ForbidFollowsCheckNot {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

pub fn forbid_follows(match_ids: &[usize]) -> ForbidFollows { // Using a bitset
    let mut bitset = 0;
    for &id in match_ids {
        bitset |= 1 << TryInto::<u16>::try_into(id).unwrap();
    }
    ForbidFollows { match_ids: bitset }
}

pub fn forbid_follows_clear() -> ForbidFollowsClear {
    ForbidFollowsClear {}
}

pub fn forbid_follows_check_not(match_id: usize) -> ForbidFollowsCheckNot {
    let bitmask = 1 << match_id;
    ForbidFollowsCheckNot { match_ids: bitmask }
}