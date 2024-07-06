use std::rc::Rc;
use std::cell::RefCell;
use crate::{Combinator, ParseData, Parser};

#[derive(Clone)]
pub struct ForwardRef {
    a: Rc<RefCell<Option<Rc<dyn Combinator<Parser = Box<dyn Parser>>>>>>,
}

impl Combinator for ForwardRef {
    type Parser = Box<dyn Parser>;

    fn parser(&self, parse_data: ParseData) -> Self::Parser {
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

    fn parser(&self, parse_data: ParseData) -> Self::Parser {
        Box::new(self.0.parser(parse_data))
    }
}