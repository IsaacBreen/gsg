use std::hash::Hash;
use crate::ParseData;
use crate::u8set::U8Set;

#[derive(Clone, PartialEq, Debug)]
pub struct ParseResult {
    pub u8set: U8Set,
    pub parse_data: Option<ParseData>,
}

impl ParseResult {
    pub fn new(u8set: U8Set, parse_data: Option<ParseData>) -> Self {
        Self { u8set, parse_data }
    }

    pub fn u8set(&self) -> &U8Set {
        &self.u8set
    }
}

impl Default for ParseResult {
    fn default() -> Self {
        Self::new(U8Set::none(), None)
    }
}

impl ParseResult {
    pub fn merge(self, other: Self) -> Self {
        let merged_data = match (self.parse_data, other.parse_data) {
            (Some(data1), Some(data2)) => Some(data1.merge(data2)),
            (data, None) | (None, data) => data,
        };
        Self {
            u8set: self.u8set | other.u8set,
            parse_data: merged_data,
        }
    }

    pub fn merge_assign(&mut self, other: Self) {
        *self = self.clone().merge(other);
    }

    pub fn forward(self, other: Self) -> Self {
        Self {
            u8set: self.u8set | other.u8set,
            parse_data: other.parse_data,
        }
    }

    pub fn forward_assign(&mut self, other: Self) {
        *self = self.clone().forward(other);
    }
}

