use crate::u256::u256;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct u8set {
    bitset: u256,
}

impl u8set {
    pub fn new() -> Self {
        u8set {
            bitset: u256::zero(),
        }
    }

    pub fn insert(&mut self, value: u8) -> bool {
        let was_set = self.bitset.is_set(value as usize);
        self.bitset.set_bit(value as usize);
        !was_set
    }

    pub fn remove(&mut self, value: u8) -> bool {
        let was_set = self.bitset.is_set(value as usize);
        self.bitset.clear_bit(value as usize);
        was_set
    }

    pub fn contains(&self, value: u8) -> bool {
        self.bitset.is_set(value as usize)
    }

    pub fn len(&self) -> usize {
        self.bitset.0.iter().map(|&x| x.count_ones() as usize).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.bitset = u256::zero();
    }

    pub fn iter(&self) -> U8SetIter {
        U8SetIter {
            bitset: self.bitset,
            index: 0,
        }
    }
}

pub struct U8SetIter {
    bitset: u256,
    index: usize,
}

impl Iterator for U8SetIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 256 {
            if self.bitset.is_set(self.index) {
                let value = self.index as u8;
                self.index += 1;
                return Some(value);
            }
            self.index += 1;
        }
        None
    }
}
