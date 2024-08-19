use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, ParseResultTrait, RightData, U8Set, VecX, VecY, UnambiguousParseResults, UnambiguousParseError};
use std::rc::Rc;

#[derive(Debug)]
pub struct EatString {
    pub string: Vec<u8>,
}

#[derive(Debug)]
pub struct EatStringParser<'a> {
    pub string: &'a [u8],
    pub index: usize,
    pub right_data: Option<RightData>,
}

impl CombinatorTrait for EatString {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(self);
    }

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        let mut parser = EatStringParser {
            string: self.string.as_slice(),
            index: 0,
            right_data: Some(right_data),
        };
        let parse_results = parser.parse(bytes);
        if parse_results.done() && parse_results.right_data_vec.len() == 1 {
            UnambiguousParseResults::Ok(parse_results.right_data_vec.into_iter().next().unwrap())
        } else {
            UnambiguousParseResults::Err(UnambiguousParseError::Fail)
        }
    }

    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        (
            Parser::EatStringParser(EatStringParser {
                string: self.string.as_slice(),
                index: 0,
                right_data: Some(right_data),
            }),
            ParseResults::new(VecY::new(), false),
        )
    }
}

impl ParserTrait for EatStringParser<'_> {
    fn get_u8set(&self) -> U8Set {
        if self.index < self.string.len() {
            U8Set::from_byte(self.string[self.index])
        } else {
            U8Set::none()
        }
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        if bytes.is_empty() {
            return ParseResults::empty_unfinished();
        }

        if self.index < self.string.len() {
            if bytes[0] == self.string[self.index] {
                self.index += 1;
                if self.index == self.string.len() {
                    let mut right_data = self.right_data.take().unwrap();
                    let right_data_inner = Rc::make_mut(&mut right_data.right_data_inner);
                    right_data_inner.fields1.position += 1;
                    return ParseResults::new_single(right_data, true);
                } else {
                    let mut right_data = self.right_data.as_mut().unwrap();
                    let right_data_inner = Rc::make_mut(&mut right_data.right_data_inner);
                    right_data_inner.fields1.position += 1;
                    return ParseResults::new_single(right_data.clone(), false);
                }
            }
        }

        ParseResults::empty_finished()
    }
}

pub fn eat_string(string: &str) -> EatString {
    EatString {
        string: string.as_bytes().to_vec(),
    }
}

pub fn eat_bytes(bytes: &[u8]) -> EatString {
    EatString {
        string: bytes.to_vec(),
    }
}

pub fn eat(s: impl Into<String>) -> EatString {
    EatString {
        string: s.into().into_bytes(),
    }
}

// impl From<EatString> for Combinator {
//     fn from(value: EatString) -> Self {
//         Combinator::EatString(value)
//     }
// }
