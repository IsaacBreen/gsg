use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use crate::*;
use crate::VecX;

pub struct Tagged<T: CombinatorTrait> {
    pub inner: Box<T>,
    pub tag: String,
}

pub struct TaggedParser<'a> {
    pub inner: Box<Parser<'a>>,
    pub tag: String,
}

impl<T: CombinatorTrait> Debug for Tagged<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tagged")
            .field("tag", &self.tag)
            .finish_non_exhaustive()
    }
}

impl Debug for TaggedParser<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaggedParser")
            .field("tag", &self.tag)
            .finish_non_exhaustive()
    }
}

impl<T: CombinatorTrait + 'static> CombinatorTrait for Tagged<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(&self.inner);
    }

    fn parse<'a>(&'a self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'a>, ParseResults) {
        let result = catch_unwind(AssertUnwindSafe(|| self.inner.parse(right_data, bytes)));
        match result {
            Ok((parser, parse_results)) => (
                Parser::TaggedParser(TaggedParser { inner: Box::new(parser), tag: self.tag.clone() }),
                parse_results,
            ),
            Err(err) => {
                eprintln!("Panic caught in parser with tag: {}", self.tag);
                resume_unwind(err);
            }
        }
    }
}

impl ParserTrait for TaggedParser<'_> {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.inner.parse(bytes)
    }
}

pub fn tag<T: IntoCombinator>(tag: &str, a: T)-> impl CombinatorTrait where T::Output: 'static {
    // TODO: ffs
    // Tagged { inner: Box::new(profile(tag, a)), tag: tag.to_string() }
    a.into_combinator()
}

 // impl From<Tagged> for Combinator {
 //     fn from(value: Tagged) -> Self {
 //         Combinator::Tagged(value)
 //     }
 // }
