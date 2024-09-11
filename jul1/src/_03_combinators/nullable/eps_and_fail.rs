use crate::_01_parse_state::{ParseResultTrait, RightData, RightDataGetters, UpData, OneShotUpData};
use crate::{BaseCombinatorTrait, DynCombinatorTrait, UnambiguousParseError, UnambiguousParseResults};
use crate::{CombinatorTrait, ParseResults, ParserTrait, U8Set};
#[derive(Debug)]
pub struct Eps<Output> {
    pub(crate) _phantom: std::marker::PhantomData<Output>,
}

#[derive(Debug)]
pub struct EpsParser;

impl<Output: 'static> DynCombinatorTrait for Eps<Output> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults<Output>) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &'a [u8]) -> UnambiguousParseResults<Output> where Output: 'a {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<Output: 'static> CombinatorTrait for Eps<Output> {
    type Parser<'a> = EpsParser;
    type Output = Output;

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Output> where Output: 'b {
        Ok(OneShotUpData::new(right_data, Output::default()))
    }

    fn old_parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Output>) where Output: 'b {
        (EpsParser, ParseResults::new_single(UpData::new(right_data, Output::default()), true))
    }
}

impl<Output> BaseCombinatorTrait for Eps<Output> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

impl ParserTrait for EpsParser {
    fn get_u8set(&self) -> U8Set {
        panic!("EpsParser.get_u8set() called")
    }

    fn parse<'b>(&mut self, bytes: &'b [u8]) -> ParseResults<()> where (): 'b {
        panic!("EpsParser already consumed")
    }
}

pub fn eps<Output>() -> Eps<Output> {
    Eps { _phantom: std::marker::PhantomData }
}


    
#[repr(transparent)]
#[derive(Debug)]
pub struct FailParser;

#[derive(Debug)]
pub struct Fail<Output> {
    pub(crate) _phantom: std::marker::PhantomData<Output>,
}

impl<Output: 'static> DynCombinatorTrait for Fail<Output> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults<Output>) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &'a [u8]) -> UnambiguousParseResults<Output> where Output: 'a {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<Output: 'static> CombinatorTrait for Fail<Output> {
    type Parser<'a> = FailParser where Self: 'a;
    type Output = Output;

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Output> where Output: 'b {
        Err(UnambiguousParseError::Fail)
    }

    fn old_parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Output>) where Output: 'b {
        (FailParser, ParseResults::empty_finished())
    }
}

impl<Output> BaseCombinatorTrait for Fail<Output> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

impl ParserTrait for FailParser {
    fn get_u8set(&self) -> U8Set {
        panic!("FailParser.get_u8set() called")
    }

    fn parse<'b>(&mut self, bytes: &'b [u8]) -> ParseResults<()> where (): 'b {
        panic!("FailParser already consumed")
    }
}

pub fn fail<Output>() -> Fail<Output> {
    Fail { _phantom: std::marker::PhantomData }
}