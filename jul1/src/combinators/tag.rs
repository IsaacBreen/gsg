use std::fmt::{Debug, Formatter};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use crate::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Tagged {
    pub inner: Box<Combinator>,
    pub tag: String,
}

#[derive(Clone, PartialEq, Eq, Hash)]
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
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let result = catch_unwind(AssertUnwindSafe(|| self.inner.parser(right_data)));
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

    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        if self.tag == "NAME" {
            println!("WHAT THE HECK");
        }
        let result = catch_unwind(AssertUnwindSafe(|| self.inner.parser_with_steps(right_data, bytes)));
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

impl ParserTrait for TaggedParser {
    fn step(&mut self, c: u8) -> ParseResults {
        let result = catch_unwind(AssertUnwindSafe(|| self.inner.step(c)));
        match result {
            Ok(parse_results) => parse_results,
            Err(err) => {
                eprintln!("Panic caught in step with tag: {}", self.tag);
                resume_unwind(err);
            }
        }
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        let result = catch_unwind(AssertUnwindSafe(|| self.inner.steps(bytes)));
        match result {
            Ok(parse_results) => parse_results,
            Err(err) => {
                eprintln!("Panic caught in steps with tag: {}", self.tag);
                resume_unwind(err);
            }
        }
    }
}

// pub fn tag(tag: &str, a: impl Into<Combinator>) -> Combinator {
//     a.into()
// }

pub fn tag(tag: &str, a: impl Into<Combinator>) -> Combinator {
    // TODO: ffs
    Tagged { inner: Box::new(a.into()), tag: tag.to_string() }.into()
}

 impl From<Tagged> for Combinator {
     fn from(value: Tagged) -> Self {
         Combinator::Tagged(value)
     }
 }
