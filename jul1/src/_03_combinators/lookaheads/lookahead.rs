
// src/_03_combinators/lookaheads/lookahead.rs
use crate::BaseCombinatorTrait;
use crate::*;

#[derive(Debug)]
pub struct PartialLookahead {
    pub parser: Box<dyn ParserTrait>,
    pub positive: bool,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LookaheadData {
    pub has_omitted_partial_lookaheads: bool,
}

impl Default for LookaheadData {
    fn default() -> Self {
        // LookaheadData { partial_lookaheads: vec![PartialLookahead { parser: Box::new(Parser::FailParser(FailParser)), positive: true }] }
        LookaheadData { has_omitted_partial_lookaheads: false }
    }
}

#[derive(Debug)]
pub struct Lookahead<T: CombinatorTrait> {
    pub combinator: Box<T>,
    pub positive: bool,
    pub persist_with_partial_lookahead: bool,
}

impl<T: CombinatorTrait> DynCombinatorTrait for Lookahead<T> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<T: CombinatorTrait> CombinatorTrait for Lookahead<T> {
    type Parser<'a> = FailParser where Self: 'a;
    type Output = T::Output;
    type PartialOutput = T::PartialOutput;

    fn one_shot_parse(&self, mut right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        let parse_result = self.combinator.one_shot_parse(right_data.clone(), bytes);
        if self.positive {
            match parse_result {
                Ok(_) => Ok(right_data),
                Err(_) => parse_result,
            }
        } else {
            match parse_result {
                Ok(_) => Err(UnambiguousParseError::Fail),
                Err(UnambiguousParseError::Fail) => Ok(right_data),
                Err(UnambiguousParseError::Ambiguous | UnambiguousParseError::Incomplete) => parse_result,
            }
        }
    }
    fn old_parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let (parser, mut parse_results) = self.combinator.parse(right_data.clone(), bytes);
        let has_right_data = !parse_results.right_data_vec.is_empty();
        let succeeds = if self.positive {
            // A positive lookahead succeeds if it yields right data now or it *could* yield right data later (i.e. it's not done yet)
            has_right_data || !parse_results.done()
        } else {
            // A negative lookahead succeeds if it yields no right data now
            !has_right_data
        };
        if succeeds {
            if !parse_results.done() {
                    right_data.get_inner_mut().fields1.lookahead_data.has_omitted_partial_lookaheads = true;
            }
            (FailParser, ParseResults::new_single(right_data, true))
        } else {
            (FailParser, ParseResults::empty_finished())
        }
    }
}

impl<T: CombinatorTrait> BaseCombinatorTrait for Lookahead<T> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        f(&self.combinator);
    }
}

pub fn lookahead(combinator: impl CombinatorTrait) -> impl CombinatorTrait {
    Lookahead { combinator: Box::new(Box::new(combinator)), positive: true, persist_with_partial_lookahead: false }
}

pub fn negative_lookahead(combinator: impl CombinatorTrait) -> impl CombinatorTrait {
    Lookahead { combinator: Box::new(Box::new(combinator)), positive: false, persist_with_partial_lookahead: false }
}