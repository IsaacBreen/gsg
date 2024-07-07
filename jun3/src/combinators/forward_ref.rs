use std::cell::RefCell;
use std::rc::Rc;

use crate::{Combinator, Parser, ParseResult};
use crate::parse_data::ParseData;

#[derive(Clone)]
pub struct ForwardRef {
    a: Rc<RefCell<Option<Rc<dyn Combinator<Parser = Box<dyn Parser>>>>>>,  
}

impl Combinator for ForwardRef {
    type Parser = Box<dyn Parser>;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        self.a.borrow().as_ref().expect("ForwardRef::parser called before parser").parser(parse_data)
    }
}

pub fn forward_ref() -> ForwardRef {
    ForwardRef { a: Rc::new(RefCell::new(None)) }
}

impl ForwardRef {
    pub fn set<A: Combinator<Parser = P> + 'static, P: Parser + 'static>(&mut self, a: A) {
        let boxed: Rc<dyn Combinator<Parser=Box<dyn Parser>>> = Rc::new(Wrapper(a));
        *self.a.borrow_mut() = Some(boxed);
    }
}

struct Wrapper<T>(T);

impl<T, P> Combinator for Wrapper<T>
where
    T: Combinator<Parser = P>,
    P: Parser + 'static,
{
    type Parser = Box<dyn Parser>;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let (parser, result) = self.0.parser(parse_data);
        (Box::new(parser), result)
    }
}