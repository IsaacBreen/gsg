use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::rc::Rc;
use std::cell::RefCell;
use derivative::Derivative;
use crate::*;
use crate::GLOBAL_PROFILE_DATA; // Import the global profile data

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Profiled {
    pub inner: Box<Combinator>,
    pub tag: String,
}

#[derive(Derivative)]
#[derivative(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ProfiledParser {
    pub inner: Box<Parser>,
    pub tag: String,
}

impl CombinatorTrait for Profiled {
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        // Use the global profile data
        let mut profile_data = GLOBAL_PROFILE_DATA.lock().unwrap();
        profile_data.push_tag(self.tag.clone());
        let (parser, mut parse_results) = self.inner.parse(right_data.clone(), bytes);
        profile_data.pop_tag();

        (
            Parser::ProfiledParser(ProfiledParser {
                inner: Box::new(parser),
                tag: self.tag.clone(),
            }),
            parse_results,
        )
    }
}

impl ParserTrait for ProfiledParser {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        // Use the global profile data
        let mut profile_data = GLOBAL_PROFILE_DATA.lock().unwrap();
        profile_data.push_tag(self.tag.clone());
        let mut parse_results = self.inner.parse(bytes);
        profile_data.pop_tag();
        parse_results
    }
}

pub fn profile(tag: &str, a: impl Into<Combinator>) -> Combinator {
    Profiled { inner: Box::new(a.into()), tag: tag.to_string() }.into()
}

pub fn profile_internal(tag: &str, a: impl Into<Combinator>) -> Combinator {
    a.into()
}

impl From<Profiled> for Combinator {
    fn from(value: Profiled) -> Self {
        Combinator::Profiled(value)
    }
}