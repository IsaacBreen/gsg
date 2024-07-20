use crate::{CombinatorTrait, ParseResults, ParserTrait, Stats, U8Set};
use crate::parse_state::{RightData, UpData};

#[derive(Copy, Clone)]
pub struct EatU8 {
    u8set: U8Set,
}

pub struct EatU8Parser {
    u8set: U8Set,
    right_data: Option<RightData>,
}

impl CombinatorTrait for EatU8 {
    type Parser = EatU8Parser;
    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        let parser = EatU8Parser {
            u8set: self.u8set.clone(),
            right_data: Some(right_data),
        };
        (parser, ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData {
                u8set: self.u8set.clone(),
            }],
            cut: false,
        })
    }
}

impl ParserTrait for EatU8Parser {
    fn step(&mut self, c: u8) -> ParseResults {
        if self.u8set.contains(c) {
            if let Some(right_data) = self.right_data.take() {
                return ParseResults {
                    right_data_vec: vec![right_data],
                    up_data_vec: vec![],
                    cut: false,
                };
            }
        }
        ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            cut: false,
        }
    }
    fn collect_stats(&self, stats: &mut Stats) {
        stats.active_parser_type_counts.entry("EatU8Parser".to_string()).and_modify(|c| *c += 1).or_insert(1);
        stats.active_u8_matchers.entry(self.u8set.clone()).and_modify(|c| *c += 1).or_insert(1);
    }
}

pub fn eat_byte(byte: u8) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_byte(byte),
    }
}

pub fn eat_char(c: char) -> EatU8 {
    eat_byte(c as u8)
}

pub fn eat_char_choice(chars: &str) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_chars(chars),
    }
}

pub fn eat_char_negation_choice(chars: &str) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_chars_negation(chars),
    }
}

pub fn eat_byte_range(start: u8, end: u8) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_range(start, end),
    }
}

pub fn eat_char_negation(c: char) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_char_negation(c),
    }
}

pub fn eat_char_range(start: char, end: char) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_char_range(start, end),
    }
}

pub fn eat_char_negation_range(start: char, end: char) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_char_negation_range(start, end),
    }
}

pub fn eat_match_fn<F>(f: F) -> EatU8
where
    F: Fn(u8) -> bool,
{
    EatU8 {
        u8set: U8Set::from_match_fn(f),
    }
}