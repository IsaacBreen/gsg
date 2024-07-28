use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, Stats, U8Set};
use crate::parse_state::{RightData, UpData};

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatU8 {
    u8set: U8Set,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatU8Parser {
    u8set: U8Set,
    right_data: Option<RightData>,
}

impl CombinatorTrait for EatU8 {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let parser = EatU8Parser {
            u8set: self.u8set.clone(),
            right_data: Some(right_data),
        };
        (Parser::EatU8Parser(parser), ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![UpData {
                u8set: self.u8set.clone(),
            }],
            done: false,
        })
    }
}

impl ParserTrait for EatU8Parser {
    fn step(&mut self, c: u8) -> ParseResults {
        if self.u8set.contains(c) {
            if let Some(mut right_data) = self.right_data.take() {
                right_data.position += 1;
                return ParseResults {
                    right_data_vec: vec![right_data],
                    up_data_vec: vec![],
                    done: true,
                };
            }
        }
        if let Some(mut right_data) = self.right_data.take() {
            return ParseResults::empty_finished()
        } else {
            panic!("EatU8Parser already consumed")
        }
    }
    fn collect_stats(&self, stats: &mut Stats) {
        stats.active_parser_type_counts.entry("EatU8Parser".to_string()).and_modify(|c| *c += 1).or_insert(1);
        stats.active_u8_matchers.entry(self.u8set.clone()).and_modify(|c| *c += 1).or_insert(1);
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Parser> + 'a> {
        todo!()
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut Parser> + 'a> {
        todo!()
    }
}

pub fn eat_byte(byte: u8) -> EatU8 {
    EatU8 {
        u8set: U8Set::from_byte(byte),
    }
}

pub fn eat_char(c: char) -> Combinator {
    Combinator::EatU8(eat_byte(c as u8))
}

pub fn eat_char_choice(chars: &str) -> Combinator {
    Combinator::EatU8(EatU8 {
        u8set: U8Set::from_chars(chars),
    })
}

pub fn eat_char_negation_choice(chars: &str) -> Combinator {
    Combinator::EatU8(EatU8 {
        u8set: U8Set::from_chars_negation(chars),
    })
}

pub fn eat_byte_range(start: u8, end: u8) -> Combinator {
    Combinator::EatU8(EatU8 {
        u8set: U8Set::from_range(start, end),
    })
}

pub fn eat_char_negation(c: char) -> Combinator {
    Combinator::EatU8(EatU8 {
        u8set: U8Set::from_char_negation(c),
    })
}

pub fn eat_char_range(start: char, end: char) -> Combinator {
    Combinator::EatU8(EatU8 {
        u8set: U8Set::from_char_range(start, end),
    })
}

pub fn eat_char_negation_range(start: char, end: char) -> Combinator {
    Combinator::EatU8(EatU8 {
        u8set: U8Set::from_char_negation_range(start, end),
    })
}

pub fn eat_match_fn<F>(f: F) -> Combinator
where
    F: Fn(u8) -> bool,
{
    Combinator::EatU8(EatU8 {
        u8set: U8Set::from_match_fn(f),
    })
}