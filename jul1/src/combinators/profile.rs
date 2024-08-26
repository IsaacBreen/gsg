use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::sync::Mutex;
use derivative::Derivative;
use crate::*;
use crate::BaseCombinatorTrait;

lazy_static::lazy_static! {
    pub static ref GLOBAL_PROFILE_DATA: Mutex<ProfileDataInner> = Mutex::new(ProfileDataInner::default());
}

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
        let mut profile_data = GLOBAL_PROFILE_DATA.lock().unwrap();
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
        let mut profile_data = GLOBAL_PROFILE_DATA.lock().unwrap();
        if let Some(tag) = profile_data.tag_stack.pop() {
            let elapsed = now.duration_since(profile_data.start_time);
            *profile_data.timings.entry(tag.clone()).or_default() += elapsed;
            *profile_data.hit_counts.entry(tag).or_default() += 1;
        }
        profile_data.start_time = Instant::now();
    }

    pub fn count_hit(tag: String) {
        let mut profile_data = GLOBAL_PROFILE_DATA.lock().unwrap();
        *profile_data.hit_counts.entry(tag).or_default() += 1;
    }
}

pub fn clear_profile_data() {
    let mut profile_data = GLOBAL_PROFILE_DATA.lock().unwrap();
    profile_data.timings.clear();
    profile_data.hit_counts.clear();
    profile_data.tag_stack.clear();
    profile_data.start_time = Instant::now();
}

#[macro_export]
macro_rules! profile {
    ($tag:expr, $body:expr) => {{
        $crate::ProfileDataInner::push_tag($tag.to_string());
        let result = $body;
        $crate::ProfileDataInner::pop_tag();
        result
        // $body
    }};
}

#[macro_export]
macro_rules! profile_block {
    ($body:expr) => {{
        // $crate::ProfileDataInner::push_tag(format!("{}:{}", file!(), line!()));
        // let result = $body;
        // $crate::ProfileDataInner::pop_tag();
        // result
        $body
    }};
}

#[macro_export]
macro_rules! count_hit {
    ($tag:expr) => {
        // $crate::ProfileDataInner::count_hit($tag.to_string());
    };
}

#[derive(Debug)]
pub struct Profiled<T> {
    pub inner: T,
    pub tag: String,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ProfiledParser<P: ParserTrait> {
    pub inner: P,
    pub tag: String,
}

impl<T: CombinatorTrait + 'static> DynCombinatorTrait for Profiled<T> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<T: CombinatorTrait + 'static> CombinatorTrait for Profiled<T> {
    type Parser<'a> = ProfiledParser<T::Parser<'a>> where Self: 'a;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        profile!(&self.tag, self.inner.one_shot_parse(right_data, bytes))
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        profile!(&self.tag, {
            let (parser, parse_results) = self.inner.parse(right_data, bytes);
            let parser = ProfiledParser { inner: parser, tag: self.tag.clone() };
            (parser, parse_results)
        })
    }
}

impl<T: CombinatorTrait + 'static> BaseCombinatorTrait for Profiled<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        f(&self.inner);
    }
}

impl<P: ParserTrait> ParserTrait for ProfiledParser<P> {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        profile!(&self.tag, self.inner.parse(bytes))
    }
}

pub fn profile<T: IntoCombinator>(tag: &str, a: T)-> impl CombinatorTrait where T::Output: 'static {
    Profiled { inner: a.into_combinator(), tag: tag.to_string() }
}

pub fn profile_internal<'a, T: IntoCombinator>(tag: &str, a: T)-> impl CombinatorTrait where T::Output: 'static {
    profile(tag, a)
}