use std::collections::{BTreeSet, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// A frozen set implementation in Rust, similar to Python's frozenset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrozenSet<T: Eq + Hash> {
    inner: HashSet<T>,
    cached_hash: Arc<u64>, // Atomic reference counted hash cache
}

impl<T: Eq + Hash> FrozenSet<T> {
    /// Creates a new empty FrozenSet.
    pub fn new() -> Self {
        let inner = HashSet::new();
        let cached_hash = Arc::new(Self::calculate_hash(&inner));
        FrozenSet { inner, cached_hash }
    }

    /// Constructs a FrozenSet from an iterator.
    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let inner = HashSet::from_iter(iter);
        let cached_hash = Arc::new(Self::calculate_hash(&inner));
        FrozenSet { inner, cached_hash }
    }

    /// Helper function to calculate hash of the inner HashSet.
    fn calculate_hash(inner: &HashSet<T>) -> u64 {
        let mut accum = 0;
        for item in inner {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            item.hash(&mut hasher);
            accum ^= hasher.finish(); // Combine hashes using XOR
        }
        accum
    }

    /// Checks if the set contains a value.
    pub fn contains(&self, value: &T) -> bool {
        self.inner.contains(value)
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T> Default for FrozenSet<T>
where
    T: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Eq + Hash> Hash for FrozenSet<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cached_hash.hash(state);
    }
}

impl<T: Eq + Hash> FromIterator<T> for FrozenSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_iter(iter)
    }
}

impl<T: Eq + Hash> From<HashSet<T>> for FrozenSet<T> {
    fn from(inner: HashSet<T>) -> Self {
        Self::from_iter(inner.into_iter())
    }
}

/// Extension trait for HashSet to allow conversion into a FrozenSet.
pub trait FreezeHashSet<T: Eq + Hash> {
    fn freeze(self) -> FrozenSet<T>;
}

impl<T: Eq + Hash> FreezeHashSet<T> for HashSet<T> {
    fn freeze(self) -> FrozenSet<T> {
        FrozenSet::from_iter(self)
    }
}
/// Extension trait for BTreeSet to allow conversion into a FrozenSet.
pub trait FreezeBTreeSet<T: Eq + Hash> {
    fn freeze(self) -> FrozenSet<T>;
}

impl<T: Eq + Hash> FreezeBTreeSet<T> for BTreeSet<T> {
    fn freeze(self) -> FrozenSet<T> {
        FrozenSet::from_iter(self)
    }
}

/// Unfreeze a FrozenSet into a HashSet.
pub trait UnfreezeHashSet<T: Eq + Hash> {
    fn unfreeze(self) -> HashSet<T>;
}

impl<T: Eq + Hash> UnfreezeHashSet<T> for FrozenSet<T> {
    fn unfreeze(self) -> HashSet<T> {
        self.inner
    }
}

/// An iterator over the elements of a `FrozenSet`.
pub struct Iter<'a, T: 'a> {
    inner: std::collections::hash_set::Iter<'a, T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, T: Eq + Hash> IntoIterator for &'a FrozenSet<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self.inner.iter(),
        }
    }
}

/// An iterator over the elements of a `FrozenSet`.
pub struct IntoIter<T> {
    inner: std::collections::hash_set::IntoIter<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<T: Eq + Hash> IntoIterator for FrozenSet<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.inner.into_iter(),
        }
    }
}

impl<T: Eq + Hash> FrozenSet<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            inner: self.inner.iter(),
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            inner: self.inner.into_iter(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_set() {
        let fs: FrozenSet<i32> = FrozenSet::new();
        assert!(fs.is_empty());
    }

    #[test]
    fn test_from_iter() {
        let data = [1, 2, 3, 4, 5];
        let fs = FrozenSet::from_iter(data.iter().cloned());
        assert_eq!(fs.len(), 5);
        for i in 1..=5 {
            assert!(fs.contains(&i));
        }
    }

    #[test]
    fn test_contains() {
        let fs = FrozenSet::from_iter(vec![1, 2, 3]);
        assert!(fs.contains(&1));
        assert!(!fs.contains(&4));
    }

    #[test]
    fn test_hash() {
        let fs1 = FrozenSet::from_iter(vec![1, 2, 3]);
        let fs2 = FrozenSet::from_iter(vec![3, 2, 1]);
        let fs3 = FrozenSet::from_iter(vec![1, 2, 4]);

        let mut hasher1 = std::collections::hash_map::DefaultHasher::new();
        fs1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = std::collections::hash_map::DefaultHasher::new();
        fs2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        let mut hasher3 = std::collections::hash_map::DefaultHasher::new();
        fs3.hash(&mut hasher3);
        let hash3 = hasher3.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_freeze_hashset() {
        let hs: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
        let fs = hs.freeze();
        assert_eq!(fs.len(), 5);
        for i in 1..=5 {
            assert!(fs.contains(&i));
        }
    }

    #[test]
    fn test_freeze_btreeset() {
        let bs: BTreeSet<i32> = [5, 4, 3, 2, 1].iter().cloned().collect();
        let fs = bs.freeze();
        assert_eq!(fs.len(), 5);
        for i in 1..=5 {
            assert!(fs.contains(&i));
        }
    }
}
