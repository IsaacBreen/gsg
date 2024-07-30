use crate::Combinator;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatU8 {
    pub u8: u8,
}

pub fn eat_u8(u8: u8) -> EatU8 {
    EatU8 { u8 }
}

impl From<EatU8> for Combinator {
    fn from(eat_u8: EatU8) -> Self {
        Combinator::EatU8(eat_u8)
    }
}