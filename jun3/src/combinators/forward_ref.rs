use std::cell::RefCell;
use std::rc::Rc;
use crate::{Combinator, ParseData, Parser, ParseResult, U8Set};

pub struct ForwardRef {
    a: Option<Rc<RefCell<dyn Combinator<Parser = Box<dyn Parser>>>>>,
}

pub struct ForwardRefParser {
    a: Rc<RefCell<dyn Parser>>,
}

impl Combinator for ForwardRef {
    type Parser = ForwardRefParser;

    fn parser(&self, parse_data: ParseData) -> Self::Parser {
        let a = self.a.as_ref().expect("ForwardRef::parser called before parser");
        let parser = a.borrow().parser(parse_data);
        ForwardRefParser { a: Rc::new(RefCell::new(parser)) }
    }
}


impl Parser for ForwardRefParser {
    fn result(&self) -> ParseResult {
        self.a.borrow().result()
    }

    fn step(&mut self, c: u8) {
        self.a.borrow_mut().step(c);
    }
}

pub fn forward_ref() -> ForwardRef {
    ForwardRef { a: None }
}

impl ForwardRef {
    pub fn set(&mut self, a: Rc<RefCell<dyn Combinator<Parser = Box<dyn Parser>>>>) {
        self.a = Some(a);
    }
}