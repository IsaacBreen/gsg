use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::rc::Rc;
use std::cell::RefCell;
use derivative::Derivative;
use crate::*;

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

#[derive(Clone)]
pub struct ProfileData {
    pub(crate) inner: Rc<RefCell<ProfileDataInner>>,
}

impl Default for ProfileData {
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(ProfileDataInner::default())),
        }
    }
}

impl ProfileData {
    fn push_tag(&self, tag: String) {
        let mut inner = self.inner.borrow_mut();
        let elapsed = inner.start_time.elapsed();
        if let Some(current_tag) = inner.tag_stack.last().cloned() {
            *inner.timings.entry(current_tag.clone()).or_default() += elapsed;
        }
        inner.tag_stack.push(tag);
        inner.start_time = Instant::now();
    }

    fn pop_tag(&self) {
        let mut inner = self.inner.borrow_mut();
        if let Some(tag) = inner.tag_stack.pop() {
            let elapsed = inner.start_time.elapsed();
            *inner.timings.entry(tag).or_default() += elapsed;
            inner.start_time = Instant::now();
        }
    }

    pub fn get_timings(&self) -> HashMap<String, Duration> {
        self.inner.borrow().timings.clone()
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
        let (parser, mut parse_results) = self.inner.parse(right_data.clone(), bytes);
        right_data.profile_data.push_tag("squash".to_string());
        parse_results.squash();
        right_data.profile_data.pop_tag();

        (
            Parser::ProfiledParser(ProfiledParser {
                inner: Box::new(parser),
                tag: self.tag.clone(),
                profile_data: right_data.profile_data.clone(),
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
        let mut parse_results = self.inner.parse(bytes);
        self.profile_data.push_tag("squash".to_string());
        parse_results.squash();
        self.profile_data.pop_tag();
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