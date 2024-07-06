use crate::{Combinator, ParseData, Parser, ParseResult, U8Set};

pub struct Eps;

pub enum EpsParser {
    Start(ParseData),
    Done,
}

impl Combinator for Eps {
    type Parser = EpsParser;

    fn parser(&self, parse_data: ParseData) -> (ParseResult, Self::Parser) {
        (
            ParseResult::new(U8Set::none(), Some(parse_data.clone())),
            EpsParser::Start(parse_data)
        )
    }
}

impl Parser for EpsParser {
    fn step(self, _c: u8) -> (ParseResult, Self::Parser) {
        match self {
            EpsParser::Start(parse_data) => (
                ParseResult::new(U8Set::none(), Some(parse_data.clone())),
                EpsParser::Done
            ),
            EpsParser::Done => panic!("EpsParser::Done"),
        }
    }
}

pub fn eps() -> Eps {
    Eps
}