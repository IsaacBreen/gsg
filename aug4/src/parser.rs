use std::fmt::Debug;
use crate::{CombinatorTrait, RightData, U8Set, UnambiguousParseResults};

pub enum ParserError {
    Incomplete,
    Fail,
}

pub struct ParserSuccessResult<'a> {
    pub right_data: RightData,
    pub combinator: Box<dyn CombinatorTrait + 'a>,
}

pub type ParserResult<'a> = Result<ParserSuccessResult<'a>, ParserError>;

pub trait ParserTrait: Debug {
    fn parse(&mut self, input: &[u8]) -> ParserResult;
    fn get_u8set(&self) -> U8Set;
}

impl<'a> ParserTrait for Box<dyn ParserTrait + 'a> {
    fn parse(&mut self, input: &[u8]) -> ParserResult {
        self.as_mut().parse(input)
    }

    fn get_u8set(&self) -> U8Set {
        self.as_ref().get_u8set()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChoiceParser<T> {
    pub children: Vec<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SeqParser<Head: ParserTrait, Tail: CombinatorTrait> {
    pub head: Head,
    pub tail: Tail,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EatU8Parser {
    pub u8: u8,
}

impl<T: ParserTrait> ParserTrait for ChoiceParser<T> {
    fn parse(&mut self, input: &[u8]) -> ParserResult {
        todo!()
    }

    fn get_u8set(&self) -> U8Set {
        let mut u8set = U8Set::none();
        for child in self.children.iter() {
            u8set |= child.get_u8set();
        }
        u8set
    }
}

impl<Head: ParserTrait, Tail: CombinatorTrait> ParserTrait for SeqParser<Head, Tail> {
    fn parse(&mut self, input: &[u8]) -> ParserResult {
        todo!()
    }

    fn get_u8set(&self) -> U8Set {
        self.head.get_u8set()
    }
}