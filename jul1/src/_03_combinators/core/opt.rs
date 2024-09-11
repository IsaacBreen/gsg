use crate::BaseCombinatorTrait;
use crate::*;

#[derive(Debug)]
pub struct Opt<T: CombinatorTrait> {
    pub(crate) inner: T,
    pub(crate) greedy: bool,
}

impl<T: CombinatorTrait> DynCombinatorTrait for Opt<T> {
    fn parse_dyn<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Box<dyn ParserTrait<'a> + 'a>, ParseResults<'b, Self::Output>) where Self::Output: 'b {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<'b, Self::Output> where Self::Output: 'b {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<T: CombinatorTrait> CombinatorTrait for Opt<T> {
    type Parser<'a> = T::Parser<'a> where Self: 'a;
    type Output = Option<T::Output>;

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<'b, Self::Output> where Self::Output: 'b {
        let parse_result = self.inner.one_shot_parse(right_data.clone(), bytes);
        if self.greedy {
            match parse_result {
                Ok(one_shot_up_data) => Ok(OneShotUpData::new(one_shot_up_data.just_right_data(), Some(one_shot_up_data.output))),
                Err(UnambiguousParseError::Fail) => Ok(OneShotUpData::new(right_data, None)),
                Err(UnambiguousParseError::Incomplete | UnambiguousParseError::Ambiguous) => parse_result,
            }
        } else {
            match parse_result {
                Ok(_) => Err(UnambiguousParseError::Ambiguous),
                Err(UnambiguousParseError::Fail) => Ok(OneShotUpData::new(right_data, None)),
                Err(UnambiguousParseError::Incomplete | UnambiguousParseError::Ambiguous) => parse_result,
            }
        }
    }

    fn old_parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<'b, Self::Output>) where Self::Output: 'b {
        let (parser, mut parse_results) = self.inner.parse(right_data.clone(), bytes);
        if !(self.greedy && parse_results.succeeds_decisively()) {
            // TODO: remove the condition below. It's a hack.
            // if parse_results.up_data_vec.is_empty() {  // TODO: remove this line
                parse_results.up_data_vec.push(UpData::new(right_data, None));
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