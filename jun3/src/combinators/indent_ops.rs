use crate::Seq2;
use crate::{Combinator, ParseData, ParseDataExt, Parser, ParseResult, seq};

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

pub struct Indent {}
pub struct IndentParser {}
pub struct Dedent {}
pub struct DedentParser {}
pub struct Newline {}
pub struct NewlineParser {}
pub struct ValidateIndent {}
pub struct ValidateIndentParser {}

impl Combinator for Indent {
    type Parser = IndentParser;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        todo!();
    }
}

impl Parser for IndentParser {
    fn step(&mut self, c: u8) -> ParseResult {
        todo!();
    }
}

impl Combinator for Dedent {
    type Parser = DedentParser;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        todo!();
    }

}

impl Parser for DedentParser {
    fn step(&mut self, c: u8) -> ParseResult {
        todo!();
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

impl Combinator for ValidateIndent {
    type Parser = ValidateIndentParser;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        todo!()
    }
}

impl Parser for ValidateIndentParser {
    fn step(&mut self, c: u8) -> ParseResult {
        todo!()
    }
}

pub fn newline() -> Newline {
    Newline {}
}

pub fn indent() -> Indent {
    Indent {}
}

pub fn dedent() -> Dedent {
    Dedent {}
}

pub fn with_indent<ParserA>(a: ParserA) -> Seq2<Indent, Seq2<ParserA, Dedent>> {
    seq!(indent(), a, dedent())
}
