use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};
use crate::bitset256::BitSet256;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct U8Set {
    bitset: BitSet256,
}

impl U8Set {
    pub fn insert(&mut self, value: u8) -> bool {
        if self.contains(value) {
            false
        } else {
            self.bitset.set_bit(value);
            true
        }
    }

    pub fn remove(&mut self, value: u8) -> bool {
        if !self.contains(value) {
            false
        } else {
            self.bitset.clear_bit(value);
            true
        }
    }

    pub fn update(&mut self, other: &U8Set) {
        self.bitset.update(&other.bitset);
    }

    pub fn contains(&self, value: impl Into<u8>) -> bool {
        self.bitset.is_set(value.into())
    }

    pub fn len(&self) -> usize {
        self.bitset.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bitset.is_empty()
    }

    pub fn clear(&mut self) {
        self.bitset.clear();
    }

    pub fn all() -> Self {
        U8Set { bitset: BitSet256::all() }
    }

    pub fn none() -> Self {
        U8Set { bitset: BitSet256::none() }
    }

    pub fn from_chars(chars: &str) -> Self {
        let mut result = Self::none();
        for c in chars.chars() {
            result.insert(c as u8);
        }
        result
    }

    pub fn from_match_fn<F>(f: F) -> Self
    where
        F: Fn(u8) -> bool,
    {
        let mut result = Self::none();
        for i in 0..=255 {
            if f(i) {
                result.insert(i);
            }
        }
        result
    }

    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        (0..=255).filter(move |&i| self.contains(i))
    }
}

impl BitOr for &U8Set {
    type Output = U8Set;

    fn bitor(self, other: &U8Set) -> U8Set {
        U8Set { bitset: self.bitset.clone() | other.bitset.clone() }
    }
}

impl BitAnd for &U8Set {
    type Output = U8Set;

    fn bitand(self, other: &U8Set) -> U8Set {
        U8Set {
            bitset: self.bitset.clone() & other.bitset.clone(),
        }
    }
}

impl BitOr for U8Set {
    type Output = U8Set;

    fn bitor(self, other: U8Set) -> U8Set {
        &self | &other
    }
}

impl BitAnd for U8Set {
    type Output = U8Set;

    fn bitand(self, other: U8Set) -> U8Set {
        &self & &other
    }
}

impl BitOrAssign for U8Set {
    fn bitor_assign(&mut self, other: U8Set) {
        self.update(&other);
    }
}

impl BitAndAssign for U8Set {
    fn bitand_assign(&mut self, other: U8Set) {
        self.bitset &= other.bitset;
    }
}

impl std::fmt::Debug for U8Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "U8Set({:?})", self.iter().map(|i| i as char).collect::<Vec<_>>())
    }
}

impl std::fmt::Display for U8Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8set() {
        let mut set = U8Set::none();
        assert!(set.insert(b'a'));
        assert!(set.insert(b'b'));
        assert!(!set.insert(b'a'));
        assert!(set.contains(b'a'));
        assert!(set.contains(b'b'));
        assert!(!set.contains(b'c'));
        assert_eq!(set.len(), 2);
        assert!(set.remove(b'a'));
        assert!(!set.remove(b'c'));
        assert_eq!(set.len(), 1);
        assert!(!set.is_empty());
        set.clear();
        assert!(set.is_empty());

        let set1 = U8Set::from_chars("abc");
        let set2 = U8Set::from_chars("bcd");
        let union = &set1 | &set2;
        let intersection = &set1 & &set2;
        assert_eq!(union.len(), 4);
        assert_eq!(intersection.len(), 2);

        let even_set = U8Set::from_match_fn(|x| x % 2 == 0);
        assert!(even_set.contains(0));
        assert!(even_set.contains(2));
        assert!(!even_set.contains(1));
        assert_eq!(even_set.len(), 128);
    }
}
