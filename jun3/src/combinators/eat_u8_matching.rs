use crate::{Combinator, Parser, ParseResult, U8Set};
use crate::parse_data::ParseData;

#[derive(Clone)]
pub struct EatU8 {
    mask: U8Set,
}

pub struct EatU8Parser {
    mask: U8Set,
    parse_data: ParseData,
}

impl Combinator for EatU8 {
    type Parser = Option<EatU8Parser>;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        (Some(EatU8Parser {
            mask: self.mask.clone(),
            parse_data: parse_data,
        }), ParseResult::new(self.mask.clone(), None))
    }
}

impl Parser for Option<EatU8Parser> {
    fn step(&mut self, c: u8) -> ParseResult {
        let EatU8Parser { mask, parse_data } = self.take().expect("EatU8Parser::exhausted");
        ParseResult::new(U8Set::none(), mask.contains(c).then(|| parse_data.clone()))
    }
}

pub fn eat_u8_matching(mask: U8Set) -> EatU8 {
    EatU8 { mask }
}

pub fn eat_u8<T: Into<u8>>(c: T) -> EatU8 {
    EatU8 {
        mask: U8Set::from_u8(c.into()),
    }
}

pub fn eat_chars(chars: &str) -> EatU8 {
    EatU8 {
        mask: U8Set::from_chars(chars),
    }
}