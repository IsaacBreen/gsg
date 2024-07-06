use crate::{Combinator, ParseData, Parser, ParseResult, U8Set};

pub struct Eps;

pub enum EpsParser {
    Start(ParseData),
    Done,
}

impl Combinator for Eps {
    type Parser = EpsParser;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        (EpsParser::Start(parse_data.clone()), ParseResult::new(U8Set::none(), Some(parse_data)))
    }
}


impl Parser for EpsParser {
    fn step(&mut self, c: u8) -> ParseResult {
        let mut new_self: Option<Self> = None;
        let result = match self {
            EpsParser::Start(parse_data) => {
                new_self = Some(EpsParser::Done);
                ParseResult::new(U8Set::none(), Some(parse_data.clone()))
            }
            EpsParser::Done => panic!("EpsParser::Done"),
        };
        if let Some(new_self) = new_self {
            *self = new_self;
        }
        result
    }
}

pub fn eps() -> Eps {
    Eps
}