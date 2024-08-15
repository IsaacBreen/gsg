use std::fmt::{Debug, Formatter};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use crate::*;
use crate::VecX;

#[derive(Clone, PartialEq, Eq)]
pub struct Tagged {
    pub inner: Box<Combinator>,
    pub tag: String,
}

#[derive(Clone, PartialEq, Eq)]
pub struct TaggedParser {
    pub inner: Box<Parser>,
    pub tag: String,
}

impl Debug for Tagged {
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

impl CombinatorTrait for Tagged {
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

pub fn tag(tag: &str, a: impl Into<Combinator>) -> Combinator {
    // TODO: ffs
    // Tagged { inner: Box::new(a.into()), tag: tag.to_string() }.into()
    // Tagged { inner: Box::new(profile(tag, a).into()), tag: tag.to_string() }.into()
    a.into()
}

 impl From<Tagged> for Combinator {
     fn from(value: Tagged) -> Self {
         Combinator::Tagged(value)
     }
 }
