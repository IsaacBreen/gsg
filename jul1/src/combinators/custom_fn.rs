use std::any::Any;
use crate::*;

pub struct CustomFn<Parser: ParserTrait> {
    pub run: fn(&mut RightData) -> (Parser, ParseResults),
}

impl<Parser: ParserTrait + 'static> CombinatorTrait for CustomFn<Parser> {
    type Parser = Parser;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        (self.run)(&mut right_data)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<Parser: ParserTrait + 'static> ParserTrait for CustomFn<Parser> {
    fn step(&mut self, c: u8) -> ParseResults {
        ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            cut: false,
        }
    }

    fn dyn_eq(&self, other: &dyn ParserTrait) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            std::ptr::eq(self, other)
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn custom_fn<Parser: ParserTrait>(run: fn(&mut RightData) -> (Parser, ParseResults)) -> CustomFn<Parser> {
    CustomFn { run }
}