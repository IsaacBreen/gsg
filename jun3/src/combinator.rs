use std::rc::Rc;
use crate::ParseData;
use crate::ParseResult;

pub trait Combinator where Self: 'static {
    type Parser: Parser;
    fn parser(&self, parse_data: ParseData) -> (Self::Parser, Vec<ParseResult>);
    fn into_boxed(self) -> Box<dyn Combinator<Parser=Box<dyn Parser>>> where Self: Sized {
        Box::new(DynWrapper(self))
    }
}

pub trait Parser {
    fn step(&mut self, c: u8) -> Vec<ParseResult>;
}

impl Parser for Box<dyn Parser> {
    fn step(&mut self, c: u8) -> Vec<ParseResult> {
        (**self).step(c)
    }
}

struct DynWrapper<T>(T);

impl<T, P> Combinator for DynWrapper<T>
where
    T: Combinator<Parser = P>,
    P: Parser + 'static,
{
    type Parser = Box<dyn Parser>;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, Vec<ParseResult>) {
        let (parser, result) = self.0.parser(parse_data);
        (Box::new(parser), result)
    }
}

impl Combinator for Box<dyn Combinator<Parser=Box<dyn Parser>>> {
    type Parser = Box<dyn Parser>;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, Vec<ParseResult>) {
        (**self).parser(parse_data)
    }
}

pub type DynParser = Box<dyn Combinator<Parser=Box<dyn Parser>>>;