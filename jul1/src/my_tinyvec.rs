use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{Index, IndexMut, RangeBounds, Bound};

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
        match self {
            FastVec::None => {
                // Transition from None to One
                *self = FastVec::One(item);
            }
            FastVec::One(existing_item) => {
                // Transition from One to Many
                let mut vec = Vec::with_capacity(2);
                unsafe {
                    // Move the existing item into the vector
                    vec.push(std::ptr::read(existing_item));
                }
                vec.push(item);
                let old_self = std::mem::replace(self, FastVec::Many(vec));
                if let FastVec::One(_) = old_self {
                    // The item has been moved, so we don't need to drop it
                    std::mem::forget(old_self);
                }
            }
            FastVec::Many(vec) => {
                // Simply push the item into the existing vector
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
        match (&mut *self, &mut *other) {
            (FastVec::None, _) => {
                // Move the contents of other into self
                std::mem::swap(self, other);
            }
            (_, FastVec::None) => {
                // Do nothing, as other is empty
            }
            (FastVec::One(self_item), FastVec::One(other_item)) => {
                // Move self's item to a new vector, then append other's item
                let mut vec = Vec::with_capacity(2);
                unsafe {
                    vec.push(std::ptr::read(self_item));
                    vec.push(std::ptr::read(other_item));
                }
                let old_self = std::mem::replace(self, FastVec::Many(vec));
                if let FastVec::One(_) = old_self {
                    // The item has been moved, so we don't need to drop it
                    std::mem::forget(old_self);
                }
                *other = FastVec::None;
            }
            (FastVec::One(self_item), FastVec::Many(other_vec)) => {
                // Move self's item to the front of other's vector, then take ownership
                let mut vec = std::mem::take(other_vec);
                unsafe {
                    vec.insert(0, std::ptr::read(self_item));
                }
                let old_self = std::mem::replace(self, FastVec::Many(vec));
                if let FastVec::One(_) = old_self {
                    // The item has been moved, so we don't need to drop it
                    std::mem::forget(old_self);
                }
                *other = FastVec::None;
            }
            (FastVec::Many(self_vec), FastVec::One(other_item)) => {
                // Append other's item to self's vector
                unsafe {
                    self_vec.push(std::ptr::read(other_item));
                }
                let old_other = std::mem::replace(other, FastVec::None);
                if let FastVec::One(_) = old_other {
                    // The item has been moved, so we don't need to drop it
                    std::mem::forget(old_other);
                }
            }
            (FastVec::Many(self_vec), FastVec::Many(other_vec)) => {
                // Append other's vector to self's vector
                self_vec.append(other_vec);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        match self {
            FastVec::None => [].iter(),
            FastVec::One(item) => std::slice::from_ref(item).iter(),
            FastVec::Many(vec) => vec.iter(),
        }
    }

    pub fn drain(&mut self, range: impl RangeBounds<usize>) -> impl Iterator<Item = T> {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end + 1,
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.len(),
        };

        match self {
            FastVec::None => Vec::new().into_iter(),
            FastVec::One(_) => {
                if start == 0 && end > 0 {
                    if let FastVec::One(item) = std::mem::replace(self, FastVec::None) {
                        vec![item].into_iter()
                    } else {
                        unreachable!()
                    }
                } else {
                    Vec::new().into_iter()
                }
            }
            FastVec::Many(vec) => {
                let drained: Vec<_> = vec.drain(start..end).collect();
                if vec.len() == 1 {
                    if let Some(last_item) = vec.pop() {
                        *self = FastVec::One(last_item);
                    }
                }
                drained.into_iter()
            }
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

impl<T> Index<usize> for FastVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            FastVec::None => panic!("Index out of bounds"),
            FastVec::One(item) => {
                if index == 0 {
                    item
                } else {
                    panic!("Index out of bounds")
                }
            }
            FastVec::Many(vec) => &vec[index],
        }
    }
}

impl<T> IndexMut<usize> for FastVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            FastVec::None => panic!("Index out of bounds"),
            FastVec::One(item) => {
                if index == 0 {
                    item
                } else {
                    panic!("Index out of bounds")
                }
            }
            FastVec::Many(vec) => &mut vec[index],
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

    let drained: Vec<_> = fast_vec.drain(..).collect();
    assert_eq!(drained, vec![3, 4, 5, 6, 7]);
    assert!(fast_vec.is_empty());
}