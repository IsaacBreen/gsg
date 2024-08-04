use std::collections::HashMap;
use std::time::{Instant, Duration};
use derivative::Derivative;
use crate::*;

#[derive(Clone)]
pub struct ProfileData {
    timings: HashMap<String, Duration>,
    tag_stack: Vec<String>,
    start_time: Instant,
}

impl Default for ProfileData {
    fn default() -> Self {
        Self {
            timings: HashMap::new(),
            tag_stack: vec!["root".to_string()],
            start_time: Instant::now(),
        }
    }
}

impl ProfileData {
    fn push_tag(&mut self, tag: String) {
        let elapsed = self.start_time.elapsed();
        if let Some(current_tag) = self.tag_stack.last() {
            *self.timings.entry(current_tag.clone()).or_default() += elapsed;
        }
        self.tag_stack.push(tag);
        self.start_time = Instant::now();
    }

    fn pop_tag(&mut self) {
        if let Some(tag) = self.tag_stack.pop() {
            let elapsed = self.start_time.elapsed();
            *self.timings.entry(tag).or_default() += elapsed;
            self.start_time = Instant::now();
        }
    }

    pub fn get_timings(&self) -> &HashMap<String, Duration> {
        &self.timings
    }
}

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
    #[derivative(PartialEq = "ignore", Hash = "ignore", Debug = "ignore")]
    pub profile_data: ProfileData,
}

impl CombinatorTrait for Profiled {
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        right_data.profile_data.push_tag(self.tag.clone());
        let (parser, parse_results) = self.inner.parse(right_data.clone(), bytes);
        right_data.profile_data.pop_tag();

        (
            Parser::ProfiledParser(ProfiledParser {
                inner: Box::new(parser),
                tag: self.tag.clone(),
                profile_data: right_data.profile_data.clone()
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
        self.profile_data.push_tag(self.tag.clone());
        let parse_results = self.inner.parse(bytes);
        self.profile_data.pop_tag();
        parse_results
    }
}

pub fn profile(tag: &str, a: impl Into<Combinator>) -> Combinator {
    Profiled { inner: Box::new(a.into()), tag: tag.to_string() }.into()
}

impl From<Profiled> for Combinator {
    fn from(value: Profiled) -> Self {
        Combinator::Profiled(value)
    }
}