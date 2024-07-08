use crate::{Combinator, ParseData, ParseDataExt, Parser, ParseResult};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct IndentTrackers {
    indent_trackers: Vec<IndentTracker>
}

#[derive(Clone, PartialEq, Debug)]
pub struct IndentTracker {
    i: usize,
    indents: Vec<Vec<u8>>
}

impl ParseDataExt for IndentTrackers {
    fn merge(self, other: Self) -> Self {
        todo!()
    }

    fn forward(self, other: Self) -> Self {
        todo!()
    }
}

impl IndentTrackers {
    pub fn push(&mut self) {
        for indent_tracker in &mut self.indent_trackers {
            indent_tracker.push();
        }
    }
    pub fn pop(&mut self) {
        for indent_tracker in &mut self.indent_trackers {
            indent_tracker.pop();
        }
    }
}

impl Default for IndentTracker {
    fn default() -> Self {
        IndentTracker {
            i: 0,
            indents: vec![vec![]]
        }
    }
}


impl IndentTracker {
    pub fn push(&mut self) {
        self.indents.push(vec![]);
    }

    pub fn pop(&mut self) {
        self.indents.pop();
    }
}

pub struct WithIndent<A> {
    a: A,
}

pub struct WithIndentParser<ParserA> {
    parser: ParserA,
}

pub struct Newline {}
pub struct NewlineParser {}

impl<A, ParserA> Combinator for WithIndent<A>
where
    A: Combinator<Parser = ParserA>,
    ParserA: Parser,
{
    type Parser = WithIndentParser<ParserA>;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        todo!()
    }
}

impl<ParserA> Parser for WithIndentParser<ParserA>
where
    ParserA: Parser,
{
    fn step(&mut self, c: u8) -> ParseResult {
        todo!()
    }
}

impl Combinator for Newline {
    type Parser = NewlineParser;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        todo!()
    }
}

impl Parser for NewlineParser {
    fn step(&mut self, c: u8) -> ParseResult {
        todo!()
    }
}

pub fn with_indent<ParserA>(a: ParserA) -> WithIndent<ParserA> {
    todo!()
}

pub fn newline() -> Newline {
    todo!()
}