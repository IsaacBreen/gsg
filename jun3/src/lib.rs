#![feature(assert_matches)]

pub use combinator::*;
pub use combinators::*;
pub use parse_data::*;

// Re-export common types and functions
pub use crate::parse_iteration_result::{Frame, FrameStack, ParseResult};
pub use crate::u8set::U8Set;

mod combinator;
mod combinators;
mod parse_iteration_result;
mod u8set;
mod bitset256;

// Include tests
#[cfg(test)]
mod tests;
mod parse_data;