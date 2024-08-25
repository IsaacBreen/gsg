use std::iter::FromIterator;

const CHARMAP_SIZE: usize = 256;
const BIT_ARRAY_SIZE: usize = CHARMAP_SIZE / 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharSet {
    data: [u8; BIT_ARRAY_SIZE],
}

impl CharSet {
    pub fn new() -> Self {
        Self {
            data: [0; BIT_ARRAY_SIZE],
        }
    }

    pub fn insert(&mut self, key: char) -> bool {
        let index = key as usize;
        let byte_index = index / 8;
        let bit_index = index % 8;
        let old_value = (self.data[byte_index] >> bit_index) & 1;
        self.data[byte_index] |= 1 << bit_index;
        old_value != 0
    }

    pub fn remove(&mut self, key: char) -> bool {
        let index = key as usize;
        let byte_index = index / 8;
        let bit_index = index % 8;
        let old_value = (self.data[byte_index] >> bit_index) & 1;
        self.data[byte_index] &= !(1 << bit_index);
        old_value != 0
    }

    pub fn contains(&self, key: char) -> bool {
        let index = key as usize;
        let byte_index = index / 8;
        let bit_index = index % 8;
        (self.data[byte_index] >> bit_index) & 1 != 0
    }

    pub fn clear(&mut self) {
        self.data = [0; BIT_ARRAY_SIZE];
    }

    pub fn len(&self) -> usize {
        self.data.iter().map(|byte| byte.count_ones() as usize).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|&byte| byte == 0)
    }

    pub fn iter(&self) -> impl Iterator<Item = char> + '_ {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(byte_index, &byte)| {
                (0..8).filter_map(move |bit_index| {
                    if (byte >> bit_index) & 1 != 0 {
                        Some((byte_index * 8 + bit_index) as u8 as char)
                    } else {
                        None
                    }
                })
            })
    }
}

impl Default for CharSet {
    fn default() -> Self {
        Self::new()
    }
}

impl Extend<char> for CharSet {
    fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I) {
        for key in iter {
            self.insert(key);
        }
    }
}

impl<'a> Extend<&'a char> for CharSet {
    fn extend<I: IntoIterator<Item = &'a char>>(&mut self, iter: I) {
        for &key in iter {
            self.insert(key);
        }
    }
}

impl FromIterator<char> for CharSet {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut set = CharSet::new();
        set.extend(iter);
        set
    }
}

impl<'a> FromIterator<&'a char> for CharSet {
    fn from_iter<I: IntoIterator<Item = &'a char>>(iter: I) -> Self {
        let mut set = CharSet::new();
        set.extend(iter);
        set
    }
}

impl IntoIterator for CharSet {
    type Item = char;
    type IntoIter = std::vec::IntoIter<char>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter().collect::<Vec<_>>().into_iter()
    }
}

impl<'a> IntoIterator for &'a CharSet {
    type Item = char;
    type IntoIter = std::vec::IntoIter<char>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter().collect::<Vec<_>>().into_iter()
    }
}

impl std::ops::Index<char> for CharSet {
    type Output = bool;

    fn index(&self, index: char) -> &Self::Output {
        if self.contains(index) {
            &true
        } else {
            &false
        }
    }
}
