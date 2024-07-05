use crate::{Combinator, ParseData, Parser};

pub struct ForwardRef {
    a: Option<Box<dyn Combinator<Parser=Box<dyn Parser>>>>,
}

impl Combinator for ForwardRef {
    type Parser = Box<dyn Parser>;

    fn parser(&self, parse_data: ParseData) -> Self::Parser {
        self.a.as_ref().expect("ForwardRef::parser called before parser").as_ref().parser(parse_data)
    }
}

pub fn forward_ref() -> ForwardRef {
    ForwardRef { a: None, }
}

impl ForwardRef {
    pub fn set(&mut self, a: Box<dyn Combinator<Parser=Box<dyn Parser>>>) {
        self.a = Some(a);
    }
}