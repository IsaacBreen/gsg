use crate::parse_state::{HorizontalData, VerticalData};

pub trait CombinatorTrait where Self: 'static {
    type Parser: ParserTrait;
    fn parser(&self, horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<VerticalData>);
    fn into_boxed(self) -> Box<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>> where Self: Sized {
        Box::new(DynWrapper(self))
    }
}

pub trait ParserTrait {
    fn step(&mut self, c: u8) -> (Vec<HorizontalData>, Vec<VerticalData>);
}

impl ParserTrait for Box<dyn ParserTrait> {
    fn step(&mut self, c: u8) -> (Vec<HorizontalData>, Vec<VerticalData>) {
        (**self).step(c)
    }
}

struct DynWrapper<T>(T);

impl<T, P> CombinatorTrait for DynWrapper<T>
where
    T: CombinatorTrait<Parser = P>,
    P: ParserTrait + 'static,
{
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<VerticalData>) {
        let (parser, horizontal_data, vertical_data) = self.0.parser(horizontal_data);
        (Box::new(parser), horizontal_data, vertical_data)
    }
}

impl CombinatorTrait for Box<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>> {
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<VerticalData>) {
        (**self).parser(horizontal_data)
    }
}

pub type DynParser = Box<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>>;