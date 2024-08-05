use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::cell::RefCell;
use derivative::Derivative;
use crate::*;

thread_local! {
    static PROFILE_DATA_INNER: RefCell<ProfileDataInner> = RefCell::new(ProfileDataInner::default());
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

pub fn push_tag(tag: String) {
    PROFILE_DATA_INNER.with(|profile_data_inner| {
        let mut inner = profile_data_inner.borrow_mut();
        let elapsed = inner.start_time.elapsed();
        if let Some(current_tag) = inner.tag_stack.last().cloned() {
            *inner.timings.entry(current_tag.clone()).or_default() += elapsed;
        }
        inner.tag_stack.push(tag);
        inner.start_time = Instant::now();
    });
}

pub fn pop_tag() {
    PROFILE_DATA_INNER.with(|profile_data_inner| {
        let mut inner = profile_data_inner.borrow_mut();
        if let Some(tag) = inner.tag_stack.pop() {
            let elapsed = inner.start_time.elapsed();
            *inner.timings.entry(tag).or_default() += elapsed;
            inner.start_time = Instant::now();
        }
    });
}

pub fn get_timings() -> HashMap<String, Duration> {
    PROFILE_DATA_INNER.with(|profile_data_inner| {
        profile_data_inner.borrow().timings.clone()
    })
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
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        push_tag(self.tag.clone());
        let (parser, mut parse_results) = self.inner.parse(right_data.clone(), bytes);
        pop_tag();

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
        push_tag(self.tag.clone());
        let mut parse_results = self.inner.parse(bytes);
        pop_tag();
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
