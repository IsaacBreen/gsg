use crate::Readu8::Readu8;
use crate::u8set::u8set;

pub trait ParserState: Clone {
    fn new(value: u8) -> Self;
    fn valid_next_u8set(&self) -> u8set;
}
 