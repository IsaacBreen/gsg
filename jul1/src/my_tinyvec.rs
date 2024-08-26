use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{Bound, Deref, Index, IndexMut, RangeBounds};

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

    unsafe fn take_unchecked(&mut self) -> T {
        match self {
            FastVec::One(item) => {
                // Replace self with None first, then read the item
                let item = std::ptr::read(item);
                std::ptr::write(self as *mut _, Self::None);
                item
            }
            _ => panic!("Cannot take from empty FastVec"),
        }
    }

    pub fn push(&mut self, item: T) {
        match self {
            FastVec::None => {
                // Transition from None to One
                *self = FastVec::One(item);
            }
            FastVec::One(_) => {
                // Transition from One to Many
                let mut vec = Vec::with_capacity(2);
                vec.push(unsafe { self.take_unchecked() });
                vec.push(item);
                *self = FastVec::Many(vec);
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
        matches!(self, FastVec::None)
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
            (FastVec::One(_), FastVec::One(_)) => {
                // Move self's item to a new vector, then append other's item
                let mut vec = Vec::with_capacity(2);
                vec.push(unsafe { self.take_unchecked() });
                vec.push(unsafe { other.take_unchecked() });
                *self = FastVec::Many(vec);
            }
            (FastVec::One(_), FastVec::Many(other_vec)) => {
                // Move self's item to the front of other's vector, then take ownership
                let mut vec = std::mem::take(other_vec);
                vec.insert(0, unsafe { self.take_unchecked() });
                *self = FastVec::Many(vec);
            }
            (FastVec::Many(self_vec), FastVec::One(_)) => {
                // Append other's item to self's vector
                self_vec.push(unsafe { other.take_unchecked() });
            }
            (FastVec::Many(self_vec), FastVec::Many(other_vec)) => {
                // Append other's vector to self's vector
                self_vec.append(other_vec);
            }
        }
        // Ensure other is set to None after appending
        *other = FastVec::None;
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

    pub fn last(&self) -> Option<&T> {
        match self {
            FastVec::None => None,
            FastVec::One(item) => Some(item),
            FastVec::Many(vec) => vec.last(),
        }
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        match self {
            FastVec::None => None,
            FastVec::One(item) => Some(item),
            FastVec::Many(vec) => vec.last_mut(),
        }
    }

    pub fn truncate(&mut self, len: usize) {
        match self {
            FastVec::None => {}
            FastVec::One(_) => {
                if len == 0 {
                    *self = FastVec::None;
                }
            }
            FastVec::Many(vec) => {
                vec.truncate(len);
                if vec.len() == 1 {
                    if let Some(last_item) = vec.pop() {
                        *self = FastVec::One(last_item);
                    }
                } else if vec.is_empty() {
                    *self = FastVec::None;
                }
            }
        }
    }

    pub fn as_slice(&self) -> &[T] {
        match self {
            FastVec::None => &[],
            FastVec::One(item) => std::slice::from_ref(item),
            FastVec::Many(vec) => vec.deref(),
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
        let mut iterator = iter.into_iter();

        // Optimize for the common case where the iterator is empty
        if iterator.size_hint().0 == 0 {
            return;
        }

        match self {
            FastVec::None => {
                // If `self` is `None`, we'll start from scratch
                if let Some(item) = iterator.next() {
                    *self = FastVec::One(item);
                    if let Some(item2) = iterator.next() {
                        // We have more than one item; transition to `Many`
                        let mut vec = Vec::with_capacity(2 + iterator.size_hint().0);
                        vec.push(unsafe { self.take_unchecked() });
                        vec.push(item2);
                        vec.extend(iterator);
                        *self = FastVec::Many(vec);
                    }
                }
            }
            FastVec::One(_) => {
                // If `self` is `One`, start with the existing item
                if let Some(new_item) = iterator.next() {
                    let mut vec = Vec::with_capacity(2 + iterator.size_hint().0);
                    vec.push(unsafe { self.take_unchecked() });
                    vec.push(new_item);
                    vec.extend(iterator);
                    *self = FastVec::Many(vec);
                }
            }
            FastVec::Many(vec) => {
                // If `self` is `Many`, reserve and extend directly
                let hint = iterator.size_hint().0;
                vec.reserve(hint); // Reserve the minimum hint capacity
                vec.extend(iterator);
            }
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

// Implement IntoIterator for &FastVec<T>
impl<'a, T> IntoIterator for &'a FastVec<T> {
    type Item = &'a T;
    type IntoIter = Box<dyn Iterator<Item = &'a T> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            FastVec::None => Box::new([].iter()),
            FastVec::One(item) => Box::new(std::slice::from_ref(item).iter()),
            FastVec::Many(vec) => Box::new(vec.iter()),
        }
    }
}

// Implement IntoIterator for &mut FastVec<T>
impl<'a, T> IntoIterator for &'a mut FastVec<T> {
    type Item = &'a mut T;
    type IntoIter = Box<dyn Iterator<Item = &'a mut T> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            FastVec::None => Box::new([].iter_mut()),
            FastVec::One(item) => Box::new(std::slice::from_mut(item).iter_mut()),
            FastVec::Many(vec) => Box::new(vec.iter_mut()),
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

impl<T> Deref for FastVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        match self {
            FastVec::None => &[],
            FastVec::One(item) => std::slice::from_ref(item),
            FastVec::Many(vec) => vec.deref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vec: FastVec<i32> = FastVec::new();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_push() {
        let mut vec = FastVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
    }

    #[test]
    fn test_len() {
        let mut vec = FastVec::new();
        assert_eq!(vec.len(), 0);
        vec.push(1);
        assert_eq!(vec.len(), 1);
        vec.push(2);
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_is_empty() {
        let mut vec = FastVec::new();
        assert!(vec.is_empty());
        vec.push(1);
        assert!(!vec.is_empty());
    }

    #[test]
    fn test_get() {
        let mut vec = FastVec::new();
        vec.push(1);
        vec.push(2);
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(2), None);
    }

    #[test]
    fn test_clear() {
        let mut vec = FastVec::new();
        vec.push(1);
        vec.push(2);
        vec.clear();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_pop() {
        let mut vec = FastVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.pop(), Some(1));
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn test_retain() {
        let mut vec = FastVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.retain(|&x| x % 2 == 0);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 2);
    }

    #[test]
    fn test_append() {
        let mut vec1 = FastVec::new();
        vec1.push(1);
        vec1.push(2);
        let mut vec2 = FastVec::new();
        vec2.push(3);
        vec2.push(4);
        vec1.append(&mut vec2);
        assert_eq!(vec1.len(), 4);
        assert_eq!(vec1[0], 1);
        assert_eq!(vec1[1], 2);
        assert_eq!(vec1[2], 3);
        assert_eq!(vec1[3], 4);
        assert!(vec2.is_empty());
    }

    #[test]
    fn test_iter() {
        let mut vec = FastVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        let mut iter = vec.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_drain() {
        let mut vec = FastVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        let drained: Vec<_> = vec.drain(1..).collect();
        assert_eq!(drained, vec![2, 3]);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 1);
    }

    #[test]
    fn test_last() {
        let mut vec = FastVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.last(), Some(&3));
    }

    #[test]
    fn test_last_mut() {
        let mut vec = FastVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        if let Some(last) = vec.last_mut() {
            *last = 4;
        }
        assert_eq!(vec.last(), Some(&4));
    }

    #[test]
    fn test_truncate() {
        let mut vec = FastVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.truncate(1);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 1);

        let mut vec = FastVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.truncate(0);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_as_slice() {
        let mut vec = FastVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        let slice = vec.as_slice();
        assert_eq!(slice, &[1, 2, 3]);
    }

    #[test]
    fn test_from_iter() {
        let vec: FastVec<i32> = (1..4).collect();
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
    }

    #[test]
    fn test_extend() {
        let mut vec = FastVec::new();
        vec.extend(1..4);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
    }

    #[test]
    fn test_into_iter() {
        let vec: FastVec<i32> = (1..4).collect();
        let mut iter = vec.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter_ref() {
        let vec: FastVec<i32> = (1..4).collect();
        let mut iter = (&vec).into_iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter_mut() {
        let mut vec: FastVec<i32> = (1..4).collect();
        let mut iter = (&mut vec).into_iter();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_index() {
        let mut vec = FastVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
    }

    #[test]
    fn test_index_mut() {
        let mut vec = FastVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec[0] = 4;
        vec[1] = 5;
        vec[2] = 6;
        assert_eq!(vec[0], 4);
        assert_eq!(vec[1], 5);
        assert_eq!(vec[2], 6);
    }

    #[test]
    fn test_deref() {
        let mut vec = FastVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        let slice: &[i32] = &vec;
        assert_eq!(slice, &[1, 2, 3]);
    }

    // Extensive testing with different scenarios
    #[test]
    fn test_extensive() {
        let mut vec = FastVec::new();

        // Test empty vector
        assert!(vec.is_empty());
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.pop(), None);

        // Test single element
        vec.push(1);
        assert!(!vec.is_empty());
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 1);
        assert_eq!(vec.pop(), Some(1));
        assert!(vec.is_empty());

        // Test multiple elements
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);

        // Test pop
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.pop(), Some(1));
        assert_eq!(vec.len(), 0);
        assert!(vec.is_empty());

        // Test append
        vec.push(1);
        vec.push(2);
        let mut other = FastVec::new();
        other.push(3);
        other.push(4);
        vec.append(&mut other);
        assert_eq!(vec.len(), 4);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
        assert_eq!(vec[3], 4);

        // Test drain
        let drained: Vec<_> = vec.drain(1..3).collect();
        assert_eq!(drained, vec![2, 3]);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 4);

        // Test retain
        vec.push(2);
        vec.push(3);
        vec.retain(|&x| x % 2 == 0);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 4);
        assert_eq!(vec[1], 2);

        // Test clear
        vec.clear();
        assert!(vec.is_empty());

        // Test extend
        vec.extend(1..4);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
    }

    #[test]
    fn test_against_vec() {
        let mut fast_vec = FastVec::new();
        let mut vec = Vec::new();

        // Test push
        for i in 0..10 {
            fast_vec.push(i);
            vec.push(i);
        }
        assert_eq!(fast_vec.len(), vec.len());
        assert_eq!(fast_vec.as_slice(), vec.as_slice());

        // Test pop
        for _ in 0..5 {
            assert_eq!(fast_vec.pop(), vec.pop());
        }
        assert_eq!(fast_vec.len(), vec.len());
        assert_eq!(fast_vec.as_slice(), vec.as_slice());

        // Test append
        let mut other_fast_vec = FastVec::new();
        let mut other_vec = Vec::new();
        for i in 10..15 {
            other_fast_vec.push(i);
            other_vec.push(i);
        }
        fast_vec.append(&mut other_fast_vec);
        vec.append(&mut other_vec);
        assert_eq!(fast_vec.len(), vec.len());
        assert_eq!(fast_vec.as_slice(), vec.as_slice());

        // Test drain
        let fast_vec_drained: Vec<_> = fast_vec.drain(2..4).collect();
        let vec_drained: Vec<_> = vec.drain(2..4).collect();
        assert_eq!(fast_vec_drained, vec_drained);
        assert_eq!(fast_vec.len(), vec.len());
        assert_eq!(fast_vec.as_slice(), vec.as_slice());

        // Test retain
        fast_vec.retain(|&x| x % 2 == 0);
        vec.retain(|&x| x % 2 == 0);
        assert_eq!(fast_vec.len(), vec.len());
        assert_eq!(fast_vec.as_slice(), vec.as_slice());
    }
}