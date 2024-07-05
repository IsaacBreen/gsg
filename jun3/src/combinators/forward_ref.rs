use std::cell::RefCell;
use std::rc::Rc;
use crate::{Combinator, ParseData, Parser, ParseResult, U8Set};

pub struct ForwardRef {
    a: Option<Box<dyn Combinator<Parser=Box<dyn Parser>>>>,
}

pub struct ForwardRefParser {
    a: Box<dyn Parser>,
}

impl Combinator for ForwardRef {
    type Parser = ForwardRefParser;

    fn parser(&self, parse_data: ParseData) -> Self::Parser {
        let a = self.a.as_ref().expect("ForwardRef::parser called before parser").as_ref();
        ForwardRefParser { a: a.parser(parse_data) }
    }
}


impl Parser for ForwardRefParser {
    fn result(&self) -> ParseResult {
        self.a.result()
    }

    fn step(&mut self, c: u8) {
        self.a.step(c);
    }
}

pub fn forward_ref() -> ForwardRef {
    ForwardRef { a: None, }
}

impl ForwardRef{
    pub fn set(&mut self, a: Box<dyn Combinator<Parser=Box<dyn Parser>>>) {
        self.a = Some(a);
    }
}