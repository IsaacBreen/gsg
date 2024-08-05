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
