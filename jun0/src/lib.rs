use bitvec::prelude::*;

pub mod u8set;
mod u256;

pub trait ParserState: Clone {
    fn new() -> Self;
    fn parse<F: Readu8>(&mut self, reader: &F);
    fn valid_next_u8set(&self) -> u8set::u8set;
    fn is_valid(&self) -> bool {
        !self.valid_next_u8set().is_empty()
    }
}

pub trait Readu8 {
    fn read_u8(&self, index: usize) -> Option<u8>;
}

// Implement Readu8 for Vec<u8>
impl Readu8 for Vec<u8> {
    fn read_u8(&self, index: usize) -> Option<u8> {
        self.get(index).cloned()
    }
}

// Implement Readu8 for &[u8]
impl Readu8 for &[u8] {
    fn read_u8(&self, index: usize) -> Option<u8> {
        self.get(index).cloned()
    }
}

// Implement Readu8 for String
impl Readu8 for String {
    fn read_u8(&self, index: usize) -> Option<u8> {
        self.as_bytes().get(index).cloned()
    }
}

// Implement Readu8 for &str
impl Readu8 for &str {
    fn read_u8(&self, index: usize) -> Option<u8> {
        self.as_bytes().get(index).cloned()
    }
}

// Implement Readu8 for closures
impl<F: Fn(usize) -> Option<u8>> Readu8 for F {
    fn read_u8(&self, index: usize) -> Option<u8> {
        self(index)
    }
}
