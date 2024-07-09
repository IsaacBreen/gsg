use std::ops::Not;
use std::rc::Rc;
use crate::{Seq2, U8Set};
use crate::{Combinator, ParseData, ParseDataExt, Parser, ParseResult, seq};

pub struct BruteForce<F: Fn(&Vec<u8>, &ParseData) -> ParseResult> {
    f: Rc<F>,
}

pub struct BruteForceParser<F: Fn(&Vec<u8>, &ParseData) -> ParseResult> {
    f: Rc<F>,
    values: Vec<u8>,
    parse_data: Option<ParseData>,
}

impl<F: Fn(&Vec<u8>, &ParseData) -> ParseResult + 'static> Combinator for BruteForce<F> {
    type Parser = BruteForceParser<F>;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let result = (self.f)(&Vec::new(), &parse_data);
        (BruteForceParser {
            f: self.f.clone(),
            values: Vec::new(),
            parse_data: result.u8set.is_empty().not().then(|| parse_data),
        }, result)
    }
}

impl<F: Fn(&Vec<u8>, &ParseData) -> ParseResult> Parser for BruteForceParser<F> {
    fn step(&mut self, c: u8) -> ParseResult {
        self.values.push(c);
        let result = (self.f)(&self.values, self.parse_data.as_ref().expect("BruteForceParser::step called before parser"));
        if result.u8set.is_empty() {
            self.parse_data.take();
        }
        result
    }
}

pub fn brute_force<F: Fn(&Vec<u8>, &ParseData) -> ParseResult + 'static>(f: F) -> BruteForce<F> {
    BruteForce { f: Rc::new(f) }
}