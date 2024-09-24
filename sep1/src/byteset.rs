use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ByteSet {
    x: u128,
    y: u128,
}

impl ByteSet {
    pub fn is_set(&self, index: u8) -> bool {
        if index < 128 {
            self.x & (1 << index) != 0
        } else {
            self.y & (1 << (index - 128)) != 0
        }
    }

    pub fn count_bits_before(&self, index: u8) -> u32 {
        if index < 128 {
            (self.x & ((1u128 << index) - 1)).count_ones()
        } else {
            self.x.count_ones() + ((self.y & ((1u128 << (index - 128)) - 1)).count_ones())
        }
    }

    pub fn set_bit(&mut self, index: u8) {
        if index < 128 {
            self.x |= 1 << index;
        } else {
            self.y |= 1 << (index - 128);
        }
    }

    pub fn clear_bit(&mut self, index: u8) {
        if index < 128 {
            self.x &= !(1 << index);
        } else {
            self.y &= !(1 << (index - 128));
        }
    }

    pub fn update(&mut self, other: &Self) {
        self.x |= other.x;
        self.y |= other.y;
    }

    pub fn len(&self) -> usize {
        self.x.count_ones() as usize + self.y.count_ones() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    pub fn clear(&mut self) {
        self.x = 0;
        self.y = 0;
    }

    pub fn all() -> Self {
        ByteSet { x: u128::MAX, y: u128::MAX }
    }

    pub fn new() -> Self {
        ByteSet { x: 0, y: 0 }
    }

    pub fn none() -> Self {
        ByteSet { x: 0, y: 0 }
    }

    pub fn union(&self, other: &Self) -> Self {
        ByteSet {
            x: self.x | other.x,
            y: self.y | other.y,
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        ByteSet {
            x: self.x & other.x,
            y: self.y & other.y,
        }
    }

    pub fn complement(&self) -> Self {
        ByteSet {
            x: !self.x,
            y: !self.y,
        }
    }
}

impl BitOr for ByteSet {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        self.union(&other)
    }
}

impl BitAnd for ByteSet {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        self.intersection(&other)
    }
}

impl Not for ByteSet {
    type Output = Self;

    fn not(self) -> Self {
        ByteSet {
            x: !self.x,
            y: !self.y,
        }
    }
}

impl BitAndAssign for ByteSet {
    fn bitand_assign(&mut self, other: Self) {
        self.x &= other.x;
        self.y &= other.y;
    }
}

impl BitOrAssign for ByteSet {
    fn bitor_assign(&mut self, other: Self) {
        self.x |= other.x;
        self.y |= other.y;
    }
}
