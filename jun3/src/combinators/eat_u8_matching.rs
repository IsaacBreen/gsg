use crate::{Combinator, ParseData, Parser, ParseResult, U8Set};

#[derive(Clone)]
pub struct EatU8 {
    mask: U8Set,
}

pub enum EatU8Parser {
    Predict { mask: U8Set, parse_data: ParseData },
    Match { parse_data: ParseData },
    Mismatch,
    Done,
}

impl Combinator for EatU8 {
    type Parser = EatU8Parser;

    fn parser(&self, parse_data: ParseData) -> Self::Parser {
        EatU8Parser::Predict {
            mask: self.mask.clone(),
            parse_data,
        }
    }
}

impl Parser for EatU8Parser {
    fn result(&self) -> ParseResult {
        match self {
            EatU8Parser::Predict { mask, parse_data } => ParseResult::new(mask.clone(), None),
            EatU8Parser::Match { parse_data } => ParseResult::new(U8Set::none(), Some(parse_data.clone())),
            EatU8Parser::Mismatch => ParseResult::new(U8Set::none(), None),
            EatU8Parser::Done => panic!("EatU8Parser::Done"),
        }
    }

    fn step(&mut self, c: u8) {
        *self = match self {
            EatU8Parser::Predict { mask, parse_data } => {
                if mask.contains(c) {
                    EatU8Parser::Match { parse_data: parse_data.clone() }
                } else {
                    EatU8Parser::Mismatch
                }
            }
            EatU8Parser::Match { .. } | EatU8Parser::Mismatch => EatU8Parser::Done,
            EatU8Parser::Done => EatU8Parser::Done,
        }
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