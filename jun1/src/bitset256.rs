use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Not};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitSet256 {
    x: u128,
    y: u128,
}

impl BitSet256 {
    pub fn is_set(&self, index: u8) -> bool {
        if index < 128 {
            self.x & (1 << index) != 0
        } else {
            self.y & (1 << (index - 128)) != 0
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
        BitSet256 { x: u128::MAX, y: u128::MAX }
    }

    pub fn none() -> Self {
        BitSet256 { x: 0, y: 0 }
    }
}

impl BitOr for BitSet256 {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        BitSet256 {
            x: self.x | other.x,
            y: self.y | other.y,
        }
    }
}

impl BitAnd for BitSet256 {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        BitSet256 {
            x: self.x & other.x,
            y: self.y & other.y,
        }
    }
}

impl BitXor for BitSet256 {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        BitSet256 {
            x: self.x ^ other.x,
            y: self.y ^ other.y,
        }
    }
}

impl Not for BitSet256 {
    type Output = Self;

    fn not(self) -> Self {
        BitSet256 {
            x: !self.x,
            y: !self.y,
        }
    }
}

impl BitAndAssign for BitSet256 {
    fn bitand_assign(&mut self, other: Self) {
        self.x &= other.x;
        self.y &= other.y;
    }
}

impl BitOrAssign for BitSet256 {
    fn bitor_assign(&mut self, other: Self) {
        self.x |= other.x;
        self.y |= other.y;
    }
}