use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use crate::*;
use crate::VecX;

pub struct Tagged<T: CombinatorTrait> {
    pub inner: Box<T>,
    pub tag: String,
}

pub struct TaggedParser {
    pub inner: Box<Parser>,
    pub tag: String,
}

impl<T: CombinatorTrait> Debug for Tagged<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tagged")
            .field("tag", &self.tag)
            .finish_non_exhaustive()
    }
}

impl Debug for TaggedParser {
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

    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let result = catch_unwind(AssertUnwindSafe(|| self.inner.parse(right_data, bytes)));
        match result {
            Ok((parser, parse_results)) => (
                parser,
                parse_results,
            ),
            Err(err) => {
                eprintln!("Panic caught in parser with tag: {}", self.tag);
                resume_unwind(err);
            }
        }
    }
}

pub fn tag(tag: &str, a: impl CombinatorTrait)-> impl CombinatorTrait {
    // TODO: ffs
    // Tagged { inner: Box::new(a.into()), tag: tag.to_string() }.into()
    // Tagged { inner: Box::new(profile(tag, a).into()), tag: tag.to_string() }.into()
    a
}

 // impl From<Tagged> for Combinator {
 //     fn from(value: Tagged) -> Self {
 //         Combinator::Tagged(value)
 //     }
 // }
