// pub type VecX<T> = smallvec::SmallVec<[T; 5]>;
pub type VecX<T> = Vec<T>;
// pub type VecX<T> = crate::my_tinyvec::FastVec<T>;

// pub type VecY<T> = Vec<T>;
// pub type VecY<T> = smallvec::SmallVec<[T; 1]>;
// pub type VecY<T> = tinyvec::TinyVec<[T; 10]>;
// pub type VecY<T> = FakeVec<T>;
pub type VecY<T> = crate::my_tinyvec::FastVec<T>;

pub type VecZ<T> = Vec<T>;
// pub type VecZ<T> = smallvec::SmallVec<[T; 1]>;
// pub type VecZ<T> = crate::my_tinyvec::FastVec<T>;

use std::iter::FromIterator;
use std::ops::{Index, IndexMut, RangeBounds};
use arrayvec::ArrayVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FakeVec<T> {
    item: Option<T>,
}

impl<T: PartialEq> FakeVec<T> {
    pub fn new() -> Self {
        FakeVec { item: None }
    }

    pub fn push(&mut self, value: T)  {
        if let Some(item) = &self.item {
            if item != &value {
                // todo: This is a hack to get FakeVec working
                // println!("FakeVec can only store one item");
                panic!("FakeVec can only store one item")
            }
        } else {
            self.item = Some(value);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.item.take()
    }

    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter.into_iter() {
            self.push(item)
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        self.extend(other.drain(..));
    }

    pub fn clear(&mut self) {
        self.item = None;
    }

    pub fn drain<R: RangeBounds<usize>>(&mut self, _range: R) -> Drain<T> {
        Drain { vec: self }
    }

    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
        if let Some(item) = &mut self.item {
            if !f(item) {
                self.item = None;
            }
        }
    }

    pub fn retain_mut<F: FnMut(&mut T) -> bool>(&mut self, mut f: F) {
        if let Some(ref mut item) = self.item {
            if !f(item) {
                self.item = None;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.item.is_some() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.item.is_none()
    }

    pub fn iter(&self) -> std::option::Iter<T> {
        self.item.iter()
    }

    pub fn iter_mut(&mut self) -> std::option::IterMut<T> {
        self.item.iter_mut()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index == 0 {
            self.item.as_ref()
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index == 0 {
            self.item.as_mut()
        } else {
            None
        }
    }
}

#[macro_export]
macro_rules! vecx {
    ($($x:expr),*) => {
        vec![$($x),*].into_iter().collect()
    };

    ($x:expr; $n:expr) => {
        vec![$x; $n].into_iter().collect()
    };
}

#[macro_export]
macro_rules! vecy {
    ($($x:expr),*) => {
        [$($x),*].into_iter().collect()
    };

    ($x:expr; $n:expr) => {
        [$x; $n].into_iter().collect()
    };
}

impl<T: PartialEq> Default for FakeVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq> FromIterator<T> for FakeVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut fake_vec = FakeVec::new();
        for item in iter {
            fake_vec.push(item);
        }
        fake_vec
    }
}

impl<T: PartialEq> Extend<T> for FakeVec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T: PartialEq> IntoIterator for FakeVec<T> {
    type Item = T;
    type IntoIter = std::option::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.item.into_iter()
    }
}

impl<'a, T: PartialEq> IntoIterator for &'a FakeVec<T> {
    type Item = &'a T;
    type IntoIter = std::option::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: PartialEq> IntoIterator for &'a mut FakeVec<T> {
    type Item = &'a mut T;
    type IntoIter = std::option::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: PartialEq> From<Vec<T>> for FakeVec<T> {
    fn from(value: Vec<T>) -> Self {
        FakeVec {
            item: value.into_iter().next(),
        }
    }
}

pub struct Drain<'a, T> {
    vec: &'a mut FakeVec<T>,
}

impl<'a, T: PartialEq> Iterator for Drain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.vec.pop()
    }
}

impl<T: PartialEq> Index<usize> for FakeVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T: PartialEq> IndexMut<usize> for FakeVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

