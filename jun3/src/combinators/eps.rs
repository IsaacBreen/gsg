use crate::{Combinator, ParseData, Parser, ParseResult, U8Set};

pub struct Eps;

pub enum EpsParser {
    Start(ParseData),
    Done,
}

impl Combinator for Eps {
    type Parser = EpsParser;

    fn parser(&self, parse_data: ParseData) -> Self::Parser {
        EpsParser::Start(parse_data)
    }
}


impl Parser for EpsParser {
    fn result(&self) -> ParseResult {
        match self {
            EpsParser::Start(parse_data) => ParseResult::new(U8Set::none(), Some(parse_data.clone())),
            EpsParser::Done => panic!("EpsParser::Done"),
        }
    }

    fn step(&mut self, c: u8) {
        *self = match self {
            EpsParser::Start(_) => EpsParser::Done,
            EpsParser::Done => panic!("EpsParser::Done"),
        }
    }
}

pub fn eps() -> Eps {
    Eps
}