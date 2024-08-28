#![allow(warnings)]
#![feature(assert_matches)]
#![feature(let_chains)]
extern crate core;

use std::rc::Rc;
use std::sync::Mutex;
pub use combinator::*;
pub use combinators::*;
// Re-export common types and functions
pub use frame_stack::Frame;
// Re-export common types and functions
pub use frame_stack::FrameStack;
// Import Mutex for global state
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub use crate::combinators::*;
pub use crate::parse_state::*;
pub use crate::python::*;
pub use crate::tests::*;
// Re-export common types and functions
pub use crate::u8set::U8Set;
pub use convert::*;
pub use internal_vec::{VecX, VecY};
pub use tokenizer::tokenizer_combinators;
pub use tokenizer::tokenizer_combinators::*;

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
mod stats;
mod unicode_categories;
mod internal_vec;
mod tokenizer;
mod my_tinyvec;
mod convert;
