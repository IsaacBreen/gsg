use std::rc::Rc;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, ParseState, Continuation};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatString {
    pub string: Rc<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatStringParser {
    pub string: Rc<Vec<u8>>,
    pub index: usize,
    pub state: ParseState,
}

impl CombinatorTrait for EatString {
    fn init_parser(&self, state: ParseState) -> ParseResults {
        let parser = EatStringParser { string: Rc::clone(&self.string), index: 0, state };
        let continuation = Continuation { head: parser.into(), tail: vec![] };
        ParseResults { continuations: vec![continuation], waiting_continuations: vec![], states: vec![] }
    }
}

impl ParserTrait for EatStringParser {
    fn step(self, c: u8) -> ParseResults {
        if self.index < self.string.len() && c == self.string[self.index] {
            if self.index == self.string.len() - 1 {
                ParseResults { continuations: vec![], waiting_continuations: vec![], states: vec![self.state] }
            } else {
                let parser = EatStringParser { string: self.string, index: self.index + 1, state: self.state };
                let continuation = Continuation { head: parser.into(), tail: vec![] };
                ParseResults { continuations: vec![continuation], waiting_continuations: vec![], states: vec![] }
            }
        } else {
            ParseResults::default()
        }
    }
}

pub fn eat_string(string: Vec<u8>) -> EatString {
    EatString { string: Rc::new(string) }
}

impl From<EatString> for Combinator {
    fn from(eat_string: EatString) -> Self {
        Combinator::EatString(eat_string)
    }
}

impl From<EatStringParser> for Parser {
    fn from(eat_string_parser: EatStringParser) -> Self {
        Parser::EatStringParser(eat_string_parser)
    }
}
