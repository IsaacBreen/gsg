use std::rc::Rc;
use crate::parse_state::{HorizontalData, UpData};

pub trait CombinatorTrait where Self: 'static {
    type Parser: ParserTrait;
    fn parser(&self, horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<UpData>);
    fn into_boxed(self) -> Box<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>> where Self: Sized {
        Box::new(DynWrapper(self))
    }
}

pub trait ParserTrait {
    fn step(&mut self, c: u8) -> (Vec<HorizontalData>, Vec<UpData>);
}

impl ParserTrait for Box<dyn ParserTrait> {
    fn step(&mut self, c: u8) -> (Vec<HorizontalData>, Vec<UpData>) {
        (**self).step(c)
    }
}

impl<C> CombinatorTrait for Rc<C> where C: CombinatorTrait {
    type Parser = C::Parser;

    fn parser(&self, horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<UpData>) {
        (**self).parser(horizontal_data)
    }
}

struct DynWrapper<T>(T);

impl<T, P> CombinatorTrait for DynWrapper<T>
where
    T: CombinatorTrait<Parser = P>,
    P: ParserTrait + 'static,
{
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<UpData>) {
        let (parser, horizontal_data, up_data) = self.0.parser(horizontal_data);
        (Box::new(parser), horizontal_data, up_data)
    }
}

impl CombinatorTrait for Box<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>> {
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<UpData>) {
        (**self).parser(horizontal_data)
    }
}

pub type DynCombinator = dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>;