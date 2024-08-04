use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::AddAssign;
use std::panic::resume_unwind;

use derivative::Derivative;

use crate::*;

#[derive(Clone)]
pub struct ProfileData {
    pub(crate) inner: Rc<RefCell<ProfileDataInner>>,
}

impl Default for ProfileData {
    fn default() -> Self {
        Self { inner: Rc::new(RefCell::new(ProfileDataInner::default())) }
    }
}

pub struct ProfileDataInner {
    pub(crate) timings: HashMap<String, u128>,
    pub(crate) tag_stack: Vec<String>,
    pub(crate) prev_time: u128,
}

impl Default for ProfileDataInner {
    fn default() -> Self {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros();
        let tag_stack = vec!["root".to_string()];
        Self {
            timings: HashMap::new(),
            tag_stack,
            prev_time: now,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Profiled {
    pub inner: Box<Combinator>,
    pub tag: String,
}

// #[derive(Clone, PartialEq, Eq, Hash)]
#[derive(Derivative)]
#[derivative(Clone, PartialEq, Eq, Hash)]
pub struct ProfiledParser {
    pub inner: Box<Parser>,
    pub tag: String,
    #[derivative(Debug = "ignore", PartialEq = "ignore", Hash = "ignore")]
    pub profile_data: ProfileData,
}

impl Debug for Profiled {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Profiled")
            .field("tag", &self.tag)
            .finish_non_exhaustive()
    }
}

impl Debug for ProfiledParser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProfiledParser")
            .field("tag", &self.tag)
            .finish_non_exhaustive()
    }
}

impl CombinatorTrait for Profiled {
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let top_tag = right_data.profile_data.inner.borrow().tag_stack.last().unwrap().clone();
        let start_time = right_data.profile_data.inner.borrow().prev_time;
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros();
        let elapsed = now.saturating_sub(start_time);
        right_data.profile_data.inner.borrow_mut().timings.entry(top_tag).or_insert(0).add_assign(elapsed);
        right_data.profile_data.inner.borrow_mut().tag_stack.push(self.tag.clone());
        right_data.profile_data.inner.borrow_mut().prev_time = now;
        let result = self.inner.parse(right_data.clone(), bytes);
        right_data.profile_data.inner.borrow_mut().tag_stack.pop();
        let start_time = right_data.profile_data.inner.borrow().prev_time;
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros();;
        let elapsed = now.saturating_sub(start_time);
        right_data.profile_data.inner.borrow_mut().timings.entry(self.tag.clone()).or_insert(0).add_assign(elapsed);
        match result {
            Ok((parser, parse_results)) => (
                Parser::ProfiledParser(ProfiledParser { inner: Box::new(parser), tag: self.tag.clone(), profile_data: right_data.profile_data.clone() }),
                parse_results,
            ),
            Err(err) => {
                eprintln!("Panic caught in parser with tag: {}", self.tag);
                resume_unwind(err);
            }
        }
    }
}

impl ParserTrait for ProfiledParser {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let top_tag = self.profile_data.inner.borrow().tag_stack.last().unwrap().clone();
        let start_time = self.profile_data.inner.borrow().prev_time;
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros();
        let elapsed = now.saturating_sub(start_time);
        self.profile_data.inner.borrow_mut().timings.entry(top_tag).or_insert(0).add_assign(elapsed);
        self.profile_data.inner.borrow_mut().tag_stack.push(self.tag.clone());
        self.profile_data.inner.borrow_mut().prev_time = now;
        let result = self.inner.parse(bytes);
        self.profile_data.inner.borrow_mut().tag_stack.pop();
        let start_time = self.profile_data.inner.borrow().prev_time;
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros();
        let elapsed = now.saturating_sub(start_time);
        self.profile_data.inner.borrow_mut().timings.entry(self.tag.clone()).or_insert(0).add_assign(elapsed);
        match result {
            Ok(parse_results) => parse_results,
            Err(err) => {
                eprintln!("Panic caught in steps with tag: {}", self.tag);
                resume_unwind(err);
            }
        }
    }
}

// pub fn tag(tag: &str, a: impl Into<Combinator>) -> Combinator {
//     a.into()
// }

pub fn profile(tag: &str, a: impl Into<Combinator>) -> Combinator {
    // TODO: ffs
    Profiled { inner: Box::new(a.into()), tag: tag.to_string() }.into()
}

 impl From<Profiled> for Combinator {
     fn from(value: Profiled) -> Self {
         Combinator::Profiled(value)
     }
 }
