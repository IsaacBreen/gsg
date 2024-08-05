use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::rc::Rc;
use std::cell::RefCell;
use derivative::Derivative;
use crate::*;

const SQUASH: bool = true;

lazy_static::lazy_static! {
    pub static ref GLOBAL_PROFILE_DATA: Mutex<ProfileDataInner> = Mutex::new(ProfileDataInner::default());
}

#[derive(Clone)]
pub struct ProfileDataInner {
    pub(crate) timings: HashMap<String, Duration>,
    tag_stack: Vec<String>,
    start_time: Instant,
}

impl Default for ProfileDataInner {
    fn default() -> Self {
        Self {
            timings: HashMap::new(),
            tag_stack: vec!["root".to_string()],
            start_time: Instant::now(),
        }
    }
}

impl ProfileDataInner {
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

    pub fn get_timings(&self) -> HashMap<String, Duration> {
        self.timings.clone()
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
}

impl CombinatorTrait for Profiled {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let mut profile_data = GLOBAL_PROFILE_DATA.lock().unwrap();
        profile_data.push_tag(self.tag.clone());
        drop(profile_data);

        let (parser, mut parse_results) = self.inner.parse(right_data, bytes);

        let mut profile_data = GLOBAL_PROFILE_DATA.lock().unwrap();
        profile_data.pop_tag();

        if SQUASH {
            profile_data.push_tag("squash".to_string());
            parse_results.squash();
            profile_data.pop_tag();
        }

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
        let mut profile_data = GLOBAL_PROFILE_DATA.lock().unwrap();
        profile_data.push_tag(self.tag.clone());
        drop(profile_data);

        let mut parse_results = self.inner.parse(bytes);

        let mut profile_data = GLOBAL_PROFILE_DATA.lock().unwrap();
        profile_data.pop_tag();

        if SQUASH {
            profile_data.push_tag("squash".to_string());
            parse_results.squash();
            profile_data.pop_tag();
        }

        parse_results
    }
}

pub fn profile(tag: &str, a: impl Into<Combinator>) -> Combinator {
    Profiled { inner: Box::new(a.into()), tag: tag.to_string() }.into()
    // a.into()
}

pub fn profile_internal(tag: &str, a: impl Into<Combinator>) -> Combinator {
    // profile(tag, a)
    a.into()
}

impl From<Profiled> for Combinator {
    fn from(value: Profiled) -> Self {
        Combinator::Profiled(value)
    }
}