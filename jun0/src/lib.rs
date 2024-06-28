pub mod u8set;
mod u256;

use bitvec::prelude::*;

pub trait ParserState: Clone {
    fn new() -> Self;
    fn parse<F: Readu8>(&mut self, read_u8: &F);
    fn valid_next_u8set(&self) -> u8set::u8set;
    fn is_valid(&self) -> bool {
        !self.valid_next_u8set().is_empty()
    }
}

pub trait Readu8: Fn(usize) -> Option<u8> {}
impl<F: Fn(usize) -> Option<u8>> Readu8 for F {}

