use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{Index, IndexMut, RangeBounds};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FastVec<T> {
    #[default]
    None,
    One(T),
    Many(Vec<T>),
}

impl<T> FastVec<T> {
    pub fn new() -> Self {
        FastVec::None
    }

    pub fn push(&mut self, item: T) {
        todo!()
    }

    pub fn len(&self) -> usize {
        match self {
            FastVec::None => 0,
            FastVec::One(_) => 1,
            FastVec::Many(vec) => vec.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        match self {
            FastVec::None => None,
            FastVec::One(item) => {
                if index == 0 {
                    Some(item)
                } else {
                    None
                }
            }
            FastVec::Many(vec) => vec.get(index),
        }
    }

    pub fn clear(&mut self) {
        *self = FastVec::None;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            FastVec::None => None,
            FastVec::One(_) => {
                if let FastVec::One(item) = std::mem::replace(self, FastVec::None) {
                    Some(item)
                } else {
                    unreachable!()
                }
            }
            FastVec::Many(vec) => {
                let item = vec.pop();
                if vec.len() == 1 {
                    if let Some(last_item) = vec.pop() {
                        *self = FastVec::One(last_item);
                    }
                }
                item
            }
        }
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        match self {
            FastVec::None => {}
            FastVec::One(item) => {
                if !f(item) {
                    *self = FastVec::None;
                }
            }
            FastVec::Many(vec) => {
                vec.retain(f);
                if vec.len() == 1 {
                    if let Some(last_item) = vec.pop() {
                        *self = FastVec::One(last_item);
                    }
                }
            }
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        todo!()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        todo!()
    }

    pub fn drain(&mut self, range: impl RangeBounds<usize>) -> impl Iterator<Item = T> {
        todo!()
    }
}

impl<T> FromIterator<T> for FastVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut fast_vec = FastVec::new();
        for item in iter {
            fast_vec.push(item);
        }
        fast_vec
    }
}

impl<T> Extend<T> for FastVec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T> IntoIterator for FastVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            FastVec::None => Vec::new().into_iter(),
            FastVec::One(item) => vec![item].into_iter(),
            FastVec::Many(vec) => vec.into_iter(),
        }
    }
}

impl<T> Index<usize> for FastVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl<T> IndexMut<usize> for FastVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        todo!()
    }
}

fn main() {
    let mut fast_vec = FastVec::new();
    assert!(fast_vec.is_empty());

    fast_vec.push(1);
    assert_eq!(fast_vec.len(), 1);
    assert_eq!(fast_vec.get(0), Some(&1));

    fast_vec.push(2);
    assert_eq!(fast_vec.len(), 2);
    assert_eq!(fast_vec.get(0), Some(&1));
    assert_eq!(fast_vec.get(1), Some(&2));

    fast_vec.clear();
    assert!(fast_vec.is_empty());

    fast_vec.push(3);
    fast_vec.push(4);
    fast_vec.push(5);

    for item in fast_vec.iter() {
        println!("{}", item);
    }

    let mut other_vec = FastVec::new();
    other_vec.push(6);
    other_vec.push(7);

    fast_vec.append(&mut other_vec);
    assert_eq!(fast_vec.len(), 5);
    assert!(other_vec.is_empty());

    let drained: Vec<_> = fast_vec.drain().collect();
    assert_eq!(drained, vec![3, 4, 5, 6, 7]);
    assert!(fast_vec.is_empty());
}