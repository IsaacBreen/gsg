#![allow(warnings)]
#![feature(assert_matches)]
extern crate core;

use std::rc::Rc;
use std::sync::Mutex; // Import Mutex for global state
use std::collections::HashMap;
use std::time::{Instant, Duration};
pub use combinator::*;
pub use combinators::*;
// Re-export common types and functions
pub use frame_stack::Frame;
// Re-export common types and functions
pub use frame_stack::FrameStack;

pub use crate::combinators::*;
pub use crate::parse_state::*;
pub use crate::python::*;
pub use crate::tests::*;
// Re-export common types and functions
pub use crate::u8set::U8Set;
pub use compiler::Compile;

mod combinator;
mod combinators;
mod parse_state;
mod u8set;
mod bitset256;

// Include tests
mod tests;
mod frame_stack;
mod python;
mod unicode;
mod compiler;
mod stats;
mod unicode_categories;

// Global profile data
lazy_static::lazy_static! {
    static ref GLOBAL_PROFILE_DATA: Mutex<ProfileDataInner> = Mutex::new(ProfileDataInner::default());
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
        if let Some(current_tag) = self.tag_stack.last().cloned() {
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