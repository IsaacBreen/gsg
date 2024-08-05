use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::*;

lazy_static! {
    static ref PROFILE_DATA: Mutex<HashMap<String, Duration>> = Mutex::new(HashMap::new());
    static ref TAG_STACK: Mutex<Vec<String>> = Mutex::new(vec!["root".to_string()]);
    static ref START_TIME: Mutex<Instant> = Mutex::new(Instant::now());
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Profiled {
    pub inner: Box<Combinator>,
    pub tag: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ProfiledParser {
    pub inner: Box<Parser>,
    pub tag: String,
}

impl CombinatorTrait for Profiled {
    fn parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, ParseResults) {
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

fn push_tag(tag: String) {
    let mut profile_data = PROFILE_DATA.lock().unwrap();
    let mut tag_stack = TAG_STACK.lock().unwrap();
    let mut start_time = START_TIME.lock().unwrap();
    let elapsed = start_time.elapsed();
    if let Some(current_tag) = tag_stack.last().cloned() {
        *profile_data.entry(current_tag.clone()).or_default() += elapsed;
    }
    tag_stack.push(tag);
    *start_time = Instant::now();
}

fn pop_tag() {
    let mut profile_data = PROFILE_DATA.lock().unwrap();
    let mut tag_stack = TAG_STACK.lock().unwrap();
    let mut start_time = START_TIME.lock().unwrap();
    if let Some(tag) = tag_stack.pop() {
        let elapsed = start_time.elapsed();
        *profile_data.entry(tag).or_default() += elapsed;
        *start_time = Instant::now();
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
