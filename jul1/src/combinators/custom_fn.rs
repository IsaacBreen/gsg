use crate::*;

pub struct CustomFn<Parser: ParserTrait> {
    pub run: fn(&mut RightData) -> (Parser, ParseResults),
}

impl<Parser: ParserTrait + 'static> CombinatorTrait for CustomFn<Parser> {
    type Parser = Parser;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        (self.run)(&mut right_data)
    }
}

impl<Parser: ParserTrait> ParserTrait for CustomFn<Parser> {
    fn step(&mut self, c: u8) -> ParseResults {
        ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            cut: false,
        }
    }
}

pub fn custom_fn<Parser: ParserTrait>(run: fn(&mut RightData) -> (Parser, ParseResults)) -> CustomFn<Parser> {
    CustomFn { run }
}