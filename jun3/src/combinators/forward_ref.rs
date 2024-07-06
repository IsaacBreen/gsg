use std::rc::Rc;
use crate::{Combinator, ParseData, Parser};

#[derive(Clone)]
pub struct ForwardRef {
    a: Option<Rc<dyn Combinator<Parser = dyn Parser>>>,
}

impl Combinator for ForwardRef {
    type Parser = Box<dyn Parser>;

    fn parser(&self, parse_data: ParseData) -> Self::Parser {
        let x = self.a.as_ref().expect("ForwardRef::parser called before parser");
        let p = x.as_ref().parser(parse_data);
        // Downcast is safe because we know that the parser is a Box<dyn Parser>
        Box::new(x)
    }
}

pub fn forward_ref() -> ForwardRef {
    ForwardRef { a: None, }
}

impl ForwardRef {
    pub fn set<A: Combinator<Parser = P>, P: Parser>(&mut self, a: A) {
        self.a = Some(Rc::new(a));
    }
}