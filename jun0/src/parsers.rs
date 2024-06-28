use crate::Readu8::Readu8;
use crate::u8set::u8set;

pub trait ParserState: Clone {
    fn new(position: usize) -> Self;
    fn parse<F: Readu8>(&mut self, reader: &F);
    fn valid_next_u8set(&self) -> u8set;
    fn is_valid(&self) -> bool { !self.valid_next_u8set().is_empty() }
    fn position(&self) -> usize;
}
