use crate::{U8Set, CombinatorTrait, ParserTrait};
use crate::parse_state::{HorizontalData, VerticalData};

pub struct EatU8 {
    u8set: U8Set,
}

pub struct EatU8Parser {
    u8set: U8Set,
    horizontal_data: Option<HorizontalData>,
}

impl CombinatorTrait for EatU8 {
    type Parser = EatU8Parser;
    fn parser(&self, horizontal_data: HorizontalData) -> (EatU8Parser, Vec<HorizontalData>, Vec<VerticalData>) {
        let parser = EatU8Parser {
            u8set: self.u8set.clone(),
            horizontal_data: Some(horizontal_data),
        };
        (parser, vec![], vec![VerticalData {
                u8set: self.u8set.clone(),
        }])
    }
}

impl ParserTrait for EatU8Parser {
    fn step(&mut self, c: u8) -> (Vec<HorizontalData>, Vec<VerticalData>) {
        if self.u8set.contains(c) {
            if let Some(horizontal_data) = self.horizontal_data.take() {
                return (vec![horizontal_data], vec![])
            }
        }
        (vec![], vec![])
    }
}

pub fn eat_char_choice(chars: &str) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_chars(chars),
    }
}