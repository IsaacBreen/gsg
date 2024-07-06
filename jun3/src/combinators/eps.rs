use crate::{Combinator, Parser, ParseResult, U8Set};
use crate::parse_data::ParseData;

#[derive(Clone)]
pub struct Eps;

pub struct EpsParser {
    parse_data: Option<ParseData>,
}

impl Combinator for Eps {
    type Parser = EpsParser;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        (EpsParser { parse_data: Some(parse_data.clone()) }, ParseResult::new(U8Set::none(), Some(parse_data)))
    }
}


impl Parser for EpsParser {
    fn step(&mut self, c: u8) -> ParseResult {
        ParseResult::new(U8Set::none(), self.parse_data.take())
    }
}

pub fn eps() -> Eps {
    Eps
}