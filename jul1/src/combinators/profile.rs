use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::sync::Mutex;
use derivative::Derivative;
use crate::*;

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
    pub fn push_tag(&mut self, tag: String) {
        let elapsed = self.start_time.elapsed();
        if let Some(current_tag) = self.tag_stack.last() {
            *self.timings.entry(current_tag.clone()).or_default() += elapsed;
        }
        self.tag_stack.push(tag);
        self.start_time = Instant::now();
    }

    pub fn pop_tag(&mut self) {
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

#[macro_export]
macro_rules! profile {
    ($tag:expr, $body:expr) => {{
        let mut profile_data = $crate::GLOBAL_PROFILE_DATA.try_lock().unwrap();
        profile_data.push_tag($tag.to_string());
        drop(profile_data);

        let result = $body;

        let mut profile_data = $crate::GLOBAL_PROFILE_DATA.try_lock().unwrap();
        profile_data.pop_tag();
        drop(profile_data);
        result
    }};
}

#[macro_export]
macro_rules! profile_block {
    ($body:expr) => {{
        let start_time = std::time::Instant::now();
        let result = $body;
        let elapsed = start_time.elapsed();

        let mut profile_data = $crate::GLOBAL_PROFILE_DATA.try_lock().unwrap();
        let tag = format!("{}:{}", file!(), line!());
        *profile_data.timings.entry(tag).or_default() += elapsed;
        drop(profile_data);

        result
    }};
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
        profile!(&self.tag, {
            let (parser, parse_results) = self.inner.parse(right_data, bytes);
            (parser, parse_results.squashed())
        })
    }
}

impl ParserTrait for ProfiledParser {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        profile!(&self.tag, self.inner.parse(bytes).squashed())
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