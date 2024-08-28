use std::fmt::{Debug, Formatter};
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

use crate::BaseCombinatorTrait;
use crate::*;

pub struct Tagged<T: CombinatorTrait> {
    pub inner: T,
    pub tag: String,
}

pub struct TaggedParser<P> {
    pub inner: P,
    pub tag: String,
}

impl<T: CombinatorTrait> Debug for Tagged<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tagged")
            .field("tag", &self.tag)
            .finish_non_exhaustive()
    }
}

impl<P> Debug for TaggedParser<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaggedParser")
            .field("tag", &self.tag)
            .finish_non_exhaustive()
    }
}

impl<T: CombinatorTrait> DynCombinatorTrait for Tagged<T> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<'b, T: CombinatorTrait> CombinatorTrait for Tagged<T> where T: 'b {
    type Parser<'a> = TaggedParser<T::Parser<'a>> where Self: 'a;
    type Output = T::Output;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.inner.one_shot_parse(right_data, bytes)
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        count_hit!(self.tag);
        let result = catch_unwind(AssertUnwindSafe(|| self.inner.parse(right_data, bytes)));
        match result {
            Ok((parser, parse_results)) => (
                TaggedParser { inner: parser, tag: self.tag.clone() },
                parse_results,
            ),
            Err(err) => {
                eprintln!("Panic caught in parser with tag: {}", self.tag);
                resume_unwind(err);
            }
        }
    }
}

impl<T: CombinatorTrait> BaseCombinatorTrait for Tagged<T> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        f(&self.inner);
    }
}

impl<P: ParserTrait> ParserTrait for TaggedParser<P> {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.inner.parse(bytes)
    }
}

pub fn tag<T: IntoCombinator + 'static>(tag: &'static str, a: T)-> impl CombinatorTrait where T::Output: 'static {
    // TODO: ffs
    Tagged { inner: profile(tag, a), tag: tag.to_string() }
    // Tagged { inner: a.into_combinator(), tag: tag.to_string() }
    // a.into_combinator()
}
