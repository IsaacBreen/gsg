use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FastVec<T> {
    None,
    One(T),
    Many(Vec<T>),
}

impl<T> FastVec<T> {
    pub fn new() -> Self {
        FastVec::None
    }

    pub fn push(&mut self, item: T) {
        match self {
            FastVec::None => {
                *self = FastVec::One(item);
            }
            FastVec::One(existing_item) => {
                let mut vec = Vec::with_capacity(2);
                vec.push(std::mem::replace(existing_item, item));
                vec.push(item);
                *self = FastVec::Many(vec);
            }
            FastVec::Many(vec) => {
                vec.push(item);
            }
        }
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
        match (self, other) {
            (FastVec::None, FastVec::None) => {}
            (FastVec::None, FastVec::One(item)) => {
                *self = FastVec::One(std::mem::replace(item, unsafe { std::mem::zeroed() }));
                *other = FastVec::None;
            }
            (FastVec::None, FastVec::Many(vec)) => {
                *self = FastVec::Many(std::mem::replace(vec, Vec::new()));
                *other = FastVec::None;
            }
            (FastVec::One(item), FastVec::None) => {}
            (FastVec::One(item), FastVec::One(other_item)) => {
                let mut vec = Vec::with_capacity(2);
                vec.push(std::mem::replace(item, unsafe { std::mem::zeroed() }));
                vec.push(std::mem::replace(other_item, unsafe { std::mem::zeroed() }));
                *self = FastVec::Many(vec);
                *other = FastVec::None;
            }
            (FastVec::One(item), FastVec::Many(vec)) => {
                let mut new_vec = Vec::with_capacity(1 + vec.len());
                new_vec.push(std::mem::replace(item, unsafe { std::mem::zeroed() }));
                new_vec.append(vec);
                *self = FastVec::Many(new_vec);
                *other = FastVec::None;
            }
            (FastVec::Many(vec), FastVec::None) => {}
            (FastVec::Many(vec), FastVec::One(other_item)) => {
                vec.push(std::mem::replace(other_item, unsafe { std::mem::zeroed() }));
                *other = FastVec::None;
            }
            (FastVec::Many(vec), FastVec::Many(other_vec)) => {
                vec.append(other_vec);
                *other = FastVec::None;
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        match self {
            FastVec::None => Vec::new().iter(),
            FastVec::One(item) => vec![item].iter(),
            FastVec::Many(vec) => vec.iter(),
        }
    }

    pub fn drain(&mut self) -> impl Iterator<Item = T> {
        match std::mem::replace(self, FastVec::None) {
            FastVec::None => Vec::new().into_iter(),
            FastVec::One(item) => vec![item].into_iter(),
            FastVec::Many(vec) => vec.into_iter(),
        }
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

impl<T: Hash> Hash for FastVec<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            FastVec::None => 0.hash(state),
            FastVec::One(item) => {
                1.hash(state);
                item.hash(state);
            }
            FastVec::Many(vec) => {
                2.hash(state);
                vec.hash(state);
            }
        }
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