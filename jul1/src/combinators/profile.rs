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
    pub(crate) hit_counts: HashMap<String, usize>,
    tag_stack: Vec<String>,
    start_time: Instant,
}

impl Default for ProfileDataInner {
    fn default() -> Self {
        Self {
            timings: HashMap::new(),
            hit_counts: HashMap::new(),
            tag_stack: vec!["root".to_string()],
            start_time: Instant::now(),
        }
    }
}

impl ProfileDataInner {
    pub fn push_tag(tag: String) {
        let now = Instant::now();
        let mut profile_data = GLOBAL_PROFILE_DATA.try_lock().unwrap();
        let elapsed = now.duration_since(profile_data.start_time);
        if let Some(current_tag) = profile_data.tag_stack.last().cloned() {
            *profile_data.timings.entry(current_tag.clone()).or_default() += elapsed;
            *profile_data.hit_counts.entry(current_tag).or_default() += 1;
        }
        profile_data.tag_stack.push(tag);
        profile_data.start_time = Instant::now();
    }

    pub fn pop_tag() {
        let now = Instant::now();
        let mut profile_data = GLOBAL_PROFILE_DATA.try_lock().unwrap();
        if let Some(tag) = profile_data.tag_stack.pop() {
            let elapsed = now.duration_since(profile_data.start_time);
            *profile_data.timings.entry(tag.clone()).or_default() += elapsed;
            *profile_data.hit_counts.entry(tag).or_default() += 1;
        }
        profile_data.start_time = Instant::now();
    }
}


#[macro_export]
macro_rules! profile {
    ($tag:expr, $body:expr) => {{
        $crate::ProfileDataInner::push_tag($tag.to_string());
        let result = $body;
        $crate::ProfileDataInner::pop_tag();
        result
    }};
}

#[macro_export]
macro_rules! profile_block {
    ($body:expr) => {{
        // $crate::ProfileDataInner::push_tag(format!("{}:{}", file!(), line!()));
        let result = $body;
        // $crate::ProfileDataInner::pop_tag();
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
            let parser = Parser::ProfiledParser(ProfiledParser { inner: Box::new(parser), tag: self.tag.clone() });
            (parser, parse_results)
        })
    }
}

impl ParserTrait for ProfiledParser {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        profile!(&self.tag, self.inner.parse(bytes))
    }
}

pub fn profile(tag: &str, a: impl Into<Combinator>) -> Combinator {
    // Profiled { inner: Box::new(a.into()), tag: tag.to_string() }.into()
    a.into()
}

pub fn profile_internal(tag: &str, a: impl Into<Combinator>) -> Combinator {
    // Profiled { inner: Box::new(a.into()), tag: tag.to_string() }.into()
    a.into()
}

impl From<Profiled> for Combinator {
    fn from(value: Profiled) -> Self {
        Combinator::Profiled(value)
    }
}