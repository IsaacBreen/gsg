use std::ops::{BitOr, BitOrAssign};
use crate::u8set::U8Set;

#[derive(Clone, PartialEq, Debug)]
pub struct ParserIterationResult {
    u8set: U8Set,
    signals: Signals,
}

impl ParserIterationResult {
    pub fn new(u8set: U8Set, signals: Signals) -> Self {
        Self { u8set, signals }
    }

    pub fn u8set(&self) -> &U8Set {
        &self.u8set
    }

    pub fn is_complete(&self) -> bool {
        self.signals.signals.contains(&Signal::Success)
    }

    pub fn signals(&self) -> &Signals {
        &self.signals
    }
}

impl BitOr for ParserIterationResult {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self {
            u8set: self.u8set | other.u8set,
            signals: other.signals,
        }
    }
}

impl BitOrAssign for ParserIterationResult {
    fn bitor_assign(&mut self, other: Self) {
        self.u8set |= other.u8set;
        self.signals |= other.signals;
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Signal {
    None,
    Success,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Signals {
    signals: Vec<Signal>,
}

impl Signals {
    pub fn success() -> Self {
        Self { signals: vec![Signal::Success] }
    }
}

impl BitOr for Signal {
    type Output = Signals;

    fn bitor(self, other: Self) -> Signals {
        Signals { signals: vec![self, other] }
    }
}

impl BitOr for Signals {
    type Output = Signals;

    fn bitor(self, other: Self) -> Signals {
        Signals { signals: self.signals.into_iter().chain(other.signals).collect() }
    }
}

impl BitOrAssign for Signals {
    fn bitor_assign(&mut self, other: Self) {
        self.signals.extend(other.signals);
    }
}