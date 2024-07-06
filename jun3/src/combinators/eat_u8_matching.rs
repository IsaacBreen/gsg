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

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        (EatU8Parser::Predict {
            mask: self.mask.clone(),
            parse_data,
        }, ParseResult::new(self.mask.clone(), None))
    }
}

impl Parser for EatU8Parser {
    fn step(&mut self, c: u8) -> ParseResult {
        let mut new_self: Option<Self> = None;
        let result = match self {
            EatU8Parser::Predict { mask, parse_data } => {
                if mask.contains(c) {
                    new_self = Some( EatU8Parser::Match { parse_data: parse_data.clone() });
                    ParseResult::new(U8Set::none(), Some(parse_data.clone()))
                } else {
                    new_self = Some( EatU8Parser::Mismatch);
                    ParseResult::empty()
                }
            }
            EatU8Parser::Match { parse_data } => {
                new_self = Some( EatU8Parser::Done);
                ParseResult::new(U8Set::none(), Some(parse_data.clone()))
            }
            EatU8Parser::Mismatch => {
                new_self = Some( EatU8Parser::Done);
                ParseResult::empty()
            }
            EatU8Parser::Done => panic!("EatU8Parser::Done"),
        };
        if let Some(new_self) = new_self {
            *self = new_self;
        }
        result
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