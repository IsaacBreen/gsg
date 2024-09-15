use crate::tokenizer::u8set::U8Set;
use std::ops::{Index, IndexMut};

const CHARMAP_SIZE: usize = 256;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TrieMap<T> {
    data: Vec<Option<Box<T>>>,
    // TODO: what's the point of `children`? Is it for nondeterminism? If so, let's remove it.
    children: Vec<Vec<usize>>,
    u8set: U8Set,
}

impl<T> TrieMap<T> {
    pub fn new() -> Self {
        let mut data = Vec::with_capacity(CHARMAP_SIZE);
        for _ in 0..CHARMAP_SIZE {
            data.push(None);
        }

        Self {
            data,
            children: vec![Vec::new(); CHARMAP_SIZE],
            u8set: U8Set::none(),
        }
    }

    pub fn insert(&mut self, key: u8, value: T) -> Option<T> {
        let index = key as usize;
        let old_value = self.data[index].take();
        self.data[index] = Some(Box::new(value));
        self.u8set.insert(key);
        old_value.map(|v| *v)
    }

    pub fn get(&self, key: u8) -> Option<&T> {
        let index = key as usize;
        self.data[index].as_ref().map(|v| v.as_ref())
    }

    pub fn get_mut(&mut self, key: u8) -> Option<&mut T> {
        let index = key as usize;
        self.data[index].as_mut().map(|v| v.as_mut())
    }

    pub fn remove(&mut self, key: u8) -> Option<T> {
        let index = key as usize;
        let old_value = self.data[index].take();
        if old_value.is_some() {
            self.u8set.remove(key);
        }
        old_value.map(|v| *v)
    }

    pub fn contains_key(&self, key: u8) -> bool {
        let index = key as usize;
        self.data[index].is_some()
    }

    pub fn clear(&mut self) {
        for element in self.data.iter_mut() {
            *element = None;
        }
        self.u8set.clear();
    }

    pub fn drain(&mut self) -> impl Iterator<Item = (u8, T)> + '_ {
        self.u8set.clear();
        self.data.iter_mut().enumerate().filter_map(|(i, option)| {
            option.take().map(|boxed| (i as u8, *boxed))
        })
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(u8, &mut T) -> bool,
    {
        for (i, value) in self.data.iter_mut().enumerate() {
            if let Some(ref mut v) = value {
                let key = i as u8;
                if !f(key, v.as_mut()) {
                    *value = None;
                    self.u8set.remove(key);
                }
            }
        }
    }

    pub fn capacity(&self) -> usize {
        CHARMAP_SIZE
    }

    pub fn len(&self) -> usize {
        self.u8set.len()
    }

    pub fn is_empty(&self) -> bool {
        self.u8set.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (u8, &T)> {
        self.data.iter().enumerate().filter_map(|(i, option)| {
            option.as_ref().map(|boxed| (i as u8, boxed.as_ref()))
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u8, &mut T)> {
        self.data.iter_mut().enumerate().filter_map(|(i, option)| {
            option.as_mut().map(|boxed| (i as u8, boxed.as_mut()))
        })
    }

    pub fn keys(&self) -> impl Iterator<Item = u8> + '_ {
        self.iter().map(|(key, _)| key)
    }

    pub fn keys_as_u8set(&self) -> U8Set {
        debug_assert_eq!(self.data.len(), CHARMAP_SIZE);
        debug_assert_eq!(self.children.len(), CHARMAP_SIZE);
        self.u8set.clone()
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.iter().map(|(_, value)| value)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.iter_mut().map(|(_, value)| value)
    }

    pub fn entry(&mut self, key: u8) -> Entry<'_, T> {
        let index = key as usize;
        if self.data[index].is_some() {
            Entry::Occupied(OccupiedEntry { map: self, index })
        } else {
            Entry::Vacant(VacantEntry { map: self, index })
        }
    }

    pub fn transition(&self, key: u8) -> Option<&Vec<usize>> {
        let index = key as usize;
        if self.u8set.contains(key) {
            Some(&self.children[index])
        } else {
            None
        }
    }

    pub fn transition_mut(&mut self, key: u8) -> Option<&mut Vec<usize>> {
        let index = key as usize;
        if self.u8set.contains(key) {
            Some(&mut self.children[index])
        } else {
            None
        }
    }

    pub fn add_transition(&mut self, from: u8, to: usize) {
        let index = from as usize;
        self.children[index].push(to);
    }
}

impl<T> Default for TrieMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<u8> for TrieMap<T> {
    type Output = T;

    fn index(&self, key: u8) -> &Self::Output {
        self.get(key).expect("Key not found")
    }
}

impl<T> IndexMut<u8> for TrieMap<T> {
    fn index_mut(&mut self, key: u8) -> &mut Self::Output {
        self.get_mut(key).expect("Key not found")
    }
}

pub enum Entry<'a, T> {
    Occupied(OccupiedEntry<'a, T>),
    Vacant(VacantEntry<'a, T>),
}

pub struct OccupiedEntry<'a, T> {
    map: &'a mut TrieMap<T>,
    index: usize,
}

pub struct VacantEntry<'a, T> {
    map: &'a mut TrieMap<T>,
    index: usize,
}

impl<'a, T> Entry<'a, T> {
    pub fn key(&self) -> u8 {
        match self {
            Entry::Occupied(occupied) => occupied.index as u8,
            Entry::Vacant(vacant) => vacant.index as u8,
        }
    }

    pub fn or_insert(self, default: T) -> &'a mut T {
        match self {
            Entry::Occupied(occupied) => occupied.into_mut(),
            Entry::Vacant(vacant) => vacant.insert(default),
        }
    }

    pub fn or_insert_with<F>(self, default: F) -> &'a mut T
    where
        F: FnOnce() -> T,
    {
        match self {
            Entry::Occupied(occupied) => occupied.into_mut(),
            Entry::Vacant(vacant) => vacant.insert(default()),
        }
    }
}

impl<'a, T> OccupiedEntry<'a, T> {
    pub fn get(&self) -> &T {
        self.map.data[self.index].as_ref().unwrap().as_ref()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.map.data[self.index].as_mut().unwrap().as_mut()
    }

    pub fn into_mut(self) -> &'a mut T {
        self.map.data[self.index].as_mut().unwrap().as_mut()
    }

    pub fn insert(&mut self, value: T) -> T {
        *std::mem::replace(&mut self.map.data[self.index], Some(Box::new(value))).unwrap()
    }

    pub fn remove(self) -> T {
        *self.map.data[self.index].take().unwrap()
    }
}

impl<'a, T> VacantEntry<'a, T> {
    pub fn insert(self, value: T) -> &'a mut T {
        self.map.data[self.index] = Some(Box::new(value));
        self.map.data[self.index].as_mut().unwrap().as_mut()
    }
}

impl<T> IntoIterator for TrieMap<T> {
    type Item = (u8, T);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter().enumerate().filter_map(|(i, option)| {
            option.map(|boxed| (i as u8, *boxed))
        }).collect::<Vec<_>>().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a TrieMap<T> {
    type Item = (u8, &'a T);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter().collect::<Vec<_>>().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut TrieMap<T> {
    type Item = (u8, &'a mut T);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut().collect::<Vec<_>>().into_iter()
    }
}

impl<T> Extend<(u8, T)> for TrieMap<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (u8, T)>,
    {
        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}