use std::rc::Rc;
use crate::{Combinator, ParseData, Parser};

#[derive(Clone)]
pub struct ForwardRef<P> {
    a: Option<Rc<dyn Combinator<Parser = P>>>,
}

impl<P: Parser> Combinator for ForwardRef<P> {
    type Parser = Box<dyn Parser>;

    fn parser(&self, parse_data: ParseData) -> Self::Parser {
        let x = self.a.as_ref().expect("ForwardRef::parser called before parser").as_ref().parser(parse_data);
        // Downcast is safe because we know that the parser is a Box<dyn Parser>
        Box::new(x)
    }
}

pub fn forward_ref<P>() -> ForwardRef<P> {
    ForwardRef { a: None, }
}

impl<P> ForwardRef<P> {
    pub fn set<A: Combinator<Parser = P>>(&mut self, a: A) {
        self.a = Some(Rc::new(a));
    }
}