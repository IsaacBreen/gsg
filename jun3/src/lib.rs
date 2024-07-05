#![feature(assert_matches)]

mod combinator;
mod combinators;
mod parse_iteration_result;
mod u8set;
mod bitset256;

pub use combinator::*;
pub use combinators::*;

// Re-export common types and functions
pub use crate::parse_iteration_result::{Frame, FrameStack, ParseResult};
pub use crate::u8set::U8Set;

// Include tests
#[cfg(test)]
mod tests;