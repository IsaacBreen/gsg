use bitvec::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct u8set {
    lower: u128,
    upper: u128,
}

impl u8set {
    pub fn new() -> Self {
        u8set { lower: 0, upper: 0 }
    }

    pub fn insert(&mut self, value: u8) {
        if value < 128 {
            self.lower |= 1u128 << value;
        } else {
            self.upper |= 1u128 << (value - 128);
        }
    }

    pub fn remove(&mut self, value: u8) {
        if value < 128 {
            self.lower &= !(1u128 << value);
        } else {
            self.upper &= !(1u128 << (value - 128));
        }
    }

    pub fn contains(&self, value: u8) -> bool {
        if value < 128 {
            (self.lower & (1u128 << value)) != 0
        } else {
            (self.upper & (1u128 << (value - 128))) != 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.lower == 0 && self.upper == 0
    }

    pub fn clear(&mut self) {
        self.lower = 0;
        self.upper = 0;
    }

    pub fn len(&self) -> usize {
        self.lower.count_ones() as usize + self.upper.count_ones() as usize
    }

    pub fn union(&self, other: &u8set) -> u8set {
        u8set {
            lower: self.lower | other.lower,
            upper: self.upper | other.upper,
        }
    }

    pub fn intersection(&self, other: &u8set) -> u8set {
        u8set {
            lower: self.lower & other.lower,
            upper: self.upper & other.upper,
        }
    }

    pub fn difference(&self, other: &u8set) -> u8set {
        u8set {
            lower: self.lower & !other.lower,
            upper: self.upper & !other.upper,
        }
    }

    pub fn symmetric_difference(&self, other: &u8set) -> u8set {
        u8set {
            lower: self.lower ^ other.lower,
            upper: self.upper ^ other.upper,
        }
    }

    pub fn is_subset(&self, other: &u8set) -> bool {
        (self.lower & other.lower == self.lower) && (self.upper & other.upper == self.upper)
    }

    pub fn is_superset(&self, other: &u8set) -> bool {
        other.is_subset(self)
    }

    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        (0..=255u8).filter(move |&x| self.contains(x))
    }
}

impl Default for u8set {
    fn default() -> Self {
        Self::new()
    }
}

impl FromIterator<u8> for u8set {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut set = u8set::new();
        for value in iter {
            set.insert(value);
        }
        set
    }
}

// Trait for the parser state
pub trait ParserState: Clone {
    fn new() -> Self;
    fn parse(&mut self, read_char: &impl ReadChar);
    fn valid_next_chars(&self) -> u8set;
    fn is_valid(&self) -> bool {
        !self.valid_next_chars().is_empty()
    }
}

// Trait for reading characters
pub trait ReadChar: Fn(usize) -> Option<char> {}
impl<F: Fn(usize) -> Option<char>> ReadChar for F {}
