use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

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

#[derive(Default, Clone)]
pub struct ProfileDataInner {
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Profiled {
    pub inner: Box<Combinator>,
    pub tag: String,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ProfiledParser {
    pub inner: Box<Parser>,
    pub tag: String,
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
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let result = catch_unwind(AssertUnwindSafe(|| self.inner.parse(right_data, bytes)));
        match result {
            Ok((parser, parse_results)) => (
                Parser::ProfiledParser(ProfiledParser { inner: Box::new(parser), tag: self.tag.clone() }),
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
        let result = catch_unwind(AssertUnwindSafe(|| self.inner.parse(bytes)));
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
