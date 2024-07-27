use std::any::Any;
use std::rc::Rc;
use crate::parse_state::{RightData, ParseResults};
use crate::combinators::{CombinatorTrait, ParserTrait, DynCombinator};

struct DynWrapper<T>(T);

impl<T, P> CombinatorTrait for DynWrapper<T>
where
    T: CombinatorTrait<Parser=P>,
    P: ParserTrait + 'static,
{
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        let (parser, parse_results) = self.0.parser(right_data);
        (Box::new(parser), parse_results)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CombinatorTrait for Box<DynCombinator> {
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        (**self).parser(right_data)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<C> CombinatorTrait for Rc<C>
where
    C: CombinatorTrait + ?Sized,
{
    type Parser = C::Parser;

    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        (**self).parser(right_data)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}