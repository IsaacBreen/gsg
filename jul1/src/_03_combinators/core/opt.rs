
// src/_03_combinators/core/opt.rs
use crate::BaseCombinatorTrait;
use crate::*;

#[derive(Debug)]
pub struct Opt<T: CombinatorTrait> {
    pub(crate) inner: T,
    pub(crate) greedy: bool,
}

impl<T: CombinatorTrait> DynCombinatorTrait for Opt<T> {
    fn parse_dyn(&self, down_data: DownData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(down_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(down_data, bytes)
    }
}

impl<T: CombinatorTrait> CombinatorTrait for Opt<T> {
    type Parser<'a> = T::Parser<'a> where Self: 'a;
    type Output = Option<T::Output>;
    type PartialOutput = Option<T::PartialOutput>;

    fn one_shot_parse(&self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        let parse_result = self.inner.one_shot_parse(down_data.clone(), bytes);
        if self.greedy {
            match parse_result {
                Ok(one_shot_up_data) => Ok(OneShotUpData::new(one_shot_up_data.just_right_data())),
                Err(UnambiguousParseError::Fail) => Ok(OneShotUpData::new(down_data.right_data)),
                Err(UnambiguousParseError::Incomplete | UnambiguousParseError::Ambiguous) => parse_result,
            }
        } else {
            match parse_result {
                Ok(_) => Err(UnambiguousParseError::Ambiguous),
                Err(UnambiguousParseError::Fail) => Ok(OneShotUpData::new(down_data.right_data)),
                Err(UnambiguousParseError::Incomplete | UnambiguousParseError::Ambiguous) => parse_result,
            }
        }
    }

    fn old_parse(&self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let (parser, mut parse_results) = self.inner.parse(down_data.clone(), bytes);
        if !(self.greedy && parse_results.succeeds_decisively()) {
            // TODO: remove the condition below. It's a hack.
            // if parse_results.up_data_vec.is_empty() {  // TODO: remove this line
                parse_results.up_data_vec.push(UpData::new(down_data.right_data));
            // }
        }
        (parser, parse_results)
    }
}

impl<T: CombinatorTrait> BaseCombinatorTrait for Opt<T> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        f(&self.inner);
    }
}

pub fn opt<T: IntoCombinator>(a: T) -> Opt<T::Output> {
    Opt { inner: a.into_combinator(), greedy: false }
}

pub fn opt_greedy(a: impl IntoCombinator)-> impl CombinatorTrait {
    profile_internal("opt_greedy", Opt { inner: a.into_combinator(), greedy: true })
}