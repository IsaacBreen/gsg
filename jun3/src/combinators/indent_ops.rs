use crate::{Seq2, U8Set};
use crate::{Combinator, ParseData, ParseDataExt, Parser, ParseResult, seq};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct IndentTrackers {
    indent_trackers: Vec<IndentTracker>
}

#[derive(Clone, PartialEq, Debug)]
pub struct IndentTracker {
    i: usize,
    indents: Vec<Vec<u8>>,
    dedent_counter: usize,
}

impl ParseDataExt for IndentTrackers {
    fn merge(self, other: Self) -> Self {
        let mut merged = self.indent_trackers;
        merged.extend(other.indent_trackers);
        IndentTrackers { indent_trackers: merged }
    }

    fn forward(self, other: Self) -> Self {
        other
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
            indents: vec![vec![]],
            dedent_counter: 0,
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

    pub fn track(&mut self, c: u8) -> bool {
        if self.i < self.indents.len() {
            let indent_level = &self.indents[self.i];
            if self.i == indent_level.len() {
                self.i += 1;
                true
            } else if indent_level[self.i] == c {
                self.i += 1;
                true
            } else {
                self.dedent_counter += self.indents.len() - self.i;
                self.i = self.indents.len();
                false
            }
        } else {
            false
        }
    }
}

pub struct Indent {}
pub struct IndentParser {
    level: Vec<u8>,
    pos: usize,
    parse_data: Option<ParseData>,
}
pub struct Dedent {}
pub struct DedentParser {
    parse_data: Option<ParseData>,
}
pub struct Newline {}
pub struct NewlineParser {
    parse_data: Option<ParseData>,
}
pub struct ValidateIndent {}
pub struct ValidateIndentParser {
    parse_data: Option<ParseData>,
}

impl Combinator for Indent {
    type Parser = IndentParser;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let indent_tracker = parse_data.indent_tracker.clone().unwrap();
        let mut new_tracker = indent_tracker.clone();
        new_tracker.push();
        let mut new_parse_data = parse_data.clone();
        new_parse_data.indent_tracker = Some(new_tracker);
        (
            IndentParser {
                level: vec![],
                pos: 0,
                parse_data: Some(new_parse_data),
            },
            ParseResult::new(U8Set::none(), Some(parse_data)),
        )
    }
}

impl Parser for IndentParser {
    fn step(&mut self, c: u8) -> ParseResult {
        if self.pos < self.level.len() {
            if self.level[self.pos] == c {
                self.pos += 1;
                ParseResult::new(U8Set::none(), self.parse_data.clone())
            } else {
                ParseResult::default()
            }
        } else {
            self.level.push(c);
            ParseResult::new(U8Set::none(), self.parse_data.clone())
        }
    }
}

impl Combinator for Dedent {
    type Parser = DedentParser;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let indent_tracker = parse_data.indent_tracker.clone().unwrap();
        let mut new_tracker = indent_tracker.clone();
        new_tracker.pop();
        let mut new_parse_data = parse_data.clone();
        new_parse_data.indent_tracker = Some(new_tracker);
        (
            DedentParser {
                parse_data: Some(new_parse_data),
            },
            ParseResult::new(U8Set::none(), Some(parse_data)),
        )
    }
}

impl Parser for DedentParser {
    fn step(&mut self, _c: u8) -> ParseResult {
        if let Some(parse_data) = &mut self.parse_data {
            if let Some(indent_tracker) = &mut parse_data.indent_tracker {
                for tracker in &mut indent_tracker.indent_trackers {
                    if tracker.dedent_counter > 0 {
                        tracker.dedent_counter -= 1;
                        return ParseResult::new(U8Set::none(), self.parse_data.clone());
                    } else {
                        return ParseResult::default();
                    }
                }
            }
        }
        ParseResult::default()
    }
}

impl Combinator for Newline {
    type Parser = NewlineParser;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        (
            NewlineParser {
                parse_data: Some(parse_data.clone()),
            },
            ParseResult::new(U8Set::from_u8(b'\n'), Some(parse_data)),
        )
    }
}

impl Parser for NewlineParser {
    fn step(&mut self, c: u8) -> ParseResult {
        if c == b'\n' {
            if let Some(parse_data) = &mut self.parse_data {
                if let Some(indent_tracker) = &mut parse_data.indent_tracker {
                    for tracker in &mut indent_tracker.indent_trackers {
                        if tracker.dedent_counter == 0 {
                            tracker.i = 0;
                        } else {
                            return ParseResult::default();
                        }
                    }
                }
            }
            ParseResult::new(U8Set::from_chars(" \t"), self.parse_data.clone())
        } else {
            if let Some(parse_data) = &mut self.parse_data {
                if let Some(indent_tracker) = &mut parse_data.indent_tracker {
                    for tracker in &mut indent_tracker.indent_trackers {
                        if !tracker.track(c) {
                            return ParseResult::default();
                        }
                    }
                }
            }
            ParseResult::new(U8Set::none(), self.parse_data.clone())
        }
    }
}

impl Combinator for ValidateIndent {
    type Parser = ValidateIndentParser;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        (
            ValidateIndentParser {
                parse_data: Some(parse_data.clone()),
            },
            ParseResult::new(U8Set::none(), Some(parse_data)),
        )
    }
}

impl Parser for ValidateIndentParser {
    fn step(&mut self, _c: u8) -> ParseResult {
        if let Some(parse_data) = &self.parse_data {
            if let Some(indent_tracker) = &parse_data.indent_tracker {
                for tracker in &indent_tracker.indent_trackers {
                    if tracker.dedent_counter != 0 {
                        return ParseResult::default();
                    }
                }
            }
        }
        ParseResult::new(U8Set::none(), self.parse_data.clone())
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

pub fn with_indent<A: Combinator>(a: A) -> Seq2<Indent, Seq2<A, Dedent>> {
    seq!(indent(), a, dedent())
}