use crate::{Combinator, ParseData, Parser, ParseResult, U8Set};

struct EatU8 {
    mask: U8Set,
}

enum EatU8Parser {
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
            EatU8Parser::Predict { mask, parse_data } => ParseResult::new(mask.clone(), Some(parse_data.clone())),
            EatU8Parser::Match { parse_data } => ParseResult::new(U8Set::none(), Some(parse_data.clone())),
            EatU8Parser::Mismatch => ParseResult::new(U8Set::none(), None),
            EatU8Parser::Done => ParseResult::new(U8Set::none(), None),
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