use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, U8Set};
use crate::parse_state::RightData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatString {
    pub(crate) string: Vec<u8>,
}

impl From<EatString> for Combinator {
    fn from(value: EatString) -> Self {
        Combinator::EatString(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatStringParser {
    pub(crate) string: Vec<u8>,
    index: usize,
    pub(crate) right_data: Option<RightData>,
}

impl CombinatorTrait for EatString {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let mut parser = EatStringParser {
            string: self.string.clone(),
            index: 0,
            right_data: Some(right_data),
        };
        let parse_results = parser.parse(bytes);
        (Parser::EatStringParser(parser), parse_results)
    }
}

impl ParserTrait for EatStringParser {
    fn get_u8set(&self) -> U8Set {
        self.string.get(self.index).map_or(U8Set::none(), |&b| U8Set::from_byte(b))
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        if bytes.is_empty() {
            return ParseResults::empty_unfinished();
        }

        let mut right_data_vec = Vec::new();
        let mut done = false;

        for &byte in bytes {
            if self.index >= self.string.len() {
                break;
            }

            if self.string[self.index] == byte {
                self.index += 1;
                if self.index == self.string.len() {
                    if let Some(mut right_data) = self.right_data.take() {
                        right_data.position += self.string.len();
                        right_data_vec.push(right_data);
                        done = true;
                        break;
                    }
                }
            } else {
                self.index = self.string.len();
                break;
            }
        }

        ParseResults { right_data_vec, done }
    }
}

pub fn eat_string(string: &str) -> EatString {
    EatString { string: string.as_bytes().to_vec() }
}

pub fn eat_bytes(bytes: &[u8]) -> EatString {
    EatString { string: bytes.to_vec() }
}

pub fn eat(string: impl Into<String>) -> EatString {
    EatString { string: string.into().into_bytes() }
}