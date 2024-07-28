use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tagged {
    pub inner: Box<Combinator>,
    pub tag: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaggedParser {
    pub inner: Box<Parser>,
    pub tag: String,
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

    fn collect_stats(&self, stats: &mut Stats) {
        todo!()
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Parser> + 'a> {
        todo!()
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut Parser> + 'a> {
        todo!()
    }
}

pub fn tag(tag: &str, a: Combinator) -> Combinator {
    Combinator::Tagged(Tagged { inner: Box::new(a), tag: tag.to_string() })
}
