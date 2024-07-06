use std::rc::Rc;
use crate::{Combinator, ParseData, Parser};

#[derive(Clone)]
pub struct ForwardRef {
    a: Option<Rc<dyn Combinator<Parser = Box<dyn Parser>>>>,
}

impl Combinator for ForwardRef {
    type Parser = Box<dyn Parser>;

    fn parser(&self, parse_data: ParseData) -> Self::Parser {
        self.a.as_ref().expect("ForwardRef::parser called before parser").parser(parse_data)
    }
}

pub fn forward_ref() -> ForwardRef {
    ForwardRef { a: None, }
}

impl ForwardRef {
    pub fn set<A: Combinator<Parser = P> + 'static, P: Parser + 'static>(&mut self, a: A) {
        self.a = Some(Rc::new(a));
    }
}
