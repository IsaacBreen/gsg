use std::rc::Rc;
use crate::DownData;
use crate::parse_state::{RightData, UpData};

pub trait CombinatorTrait where Self: 'static {
    type Parser: ParserTrait;
    fn parser(&self, right_data: RightData, down_data: DownData) -> (Self::Parser, Vec<RightData>, Vec<UpData>);
    fn into_boxed(self) -> Box<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>> where Self: Sized {
        Box::new(DynWrapper(self))
    }
}

pub trait ParserTrait {
    fn step(&mut self, c: u8, down_data: DownData) -> (Vec<RightData>, Vec<UpData>);
}

impl ParserTrait for Box<dyn ParserTrait> {
    fn step(&mut self, c: u8, down_data: DownData) -> (Vec<RightData>, Vec<UpData>) {
        (**self).step(c, down_data)
    }
}

impl<C> CombinatorTrait for Rc<C> where C: CombinatorTrait {
    type Parser = C::Parser;

    fn parser(&self, right_data: RightData, down_data: DownData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        (**self).parser(right_data, down_data)
    }
}

struct DynWrapper<T>(T);

impl<T, P> CombinatorTrait for DynWrapper<T>
where
    T: CombinatorTrait<Parser = P>,
    P: ParserTrait + 'static,
{
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, right_data: RightData, down_data: DownData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        let (parser, right_data, up_data) = self.0.parser(right_data, down_data);
        (Box::new(parser), right_data, up_data)
    }
}

impl CombinatorTrait for Box<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>> {
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, right_data: RightData, down_data: DownData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        (**self).parser(right_data, down_data)
    }
}

pub type DynCombinator = dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>;