#![feature(assert_matches)]

mod combinator;
mod combinators;
mod state;
mod helper_functions;
mod active_combinator;
mod parse_iteration_result;
mod u8set;
mod bitset256;

pub use combinator::*;
pub use combinators::*;
pub use state::*;
pub use helper_functions::*;
pub use active_combinator::*;

// Re-export common types and functions
pub use crate::parse_iteration_result::{Frame, FrameStack, ParserIterationResult};
pub use crate::u8set::U8Set;

// Include tests
#[cfg(test)]
mod tests;