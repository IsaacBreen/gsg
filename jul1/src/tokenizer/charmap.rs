use std::ops::{Index, IndexMut};

const CHARMAP_SIZE: usize = 256;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CharMap<T> {
    data: [Option<Box<T>>; CHARMAP_SIZE],
}

impl<T> CharMap<T> {
    pub fn new() -> Self {
        Self {
            data: std::array::from_fn(|_| None),
        }
    }

    pub fn insert(&mut self, key: char, value: T) -> Option<T> {
        let index = key as usize;
        let old_value = self.data[index].take();
        self.data[index] = Some(Box::new(value));
        old_value.map(|v| *v)
    }

    pub fn get(&self, key: char) -> Option<&T> {
        self.data[key as usize].as_ref().map(|v| v.as_ref())
    }

    pub fn get_mut(&mut self, key: char) -> Option<&mut T> {
        self.data[key as usize].as_mut().map(|v| v.as_mut())
    }

    pub fn remove(&mut self, key: char) -> Option<T> {
        self.data[key as usize].take().map(|v| *v)
    }

    pub fn contains_key(&self, key: char) -> bool {
        self.data[key as usize].is_some()
    }

    pub fn clear(&mut self) {
        for element in self.data.iter_mut() {
            *element = None;
        }
    }

    pub fn drain(&mut self) -> impl Iterator<Item = (char, T)> + '_ {
        self.data.iter_mut().enumerate().filter_map(|(i, option)| {
            option.take().map(|boxed| (i as u8 as char, *boxed))
        })
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(char, &mut T) -> bool,
    {
        for (i, value) in self.data.iter_mut().enumerate() {
            if let Some(ref mut v) = value {
                let key = i as u8 as char;
                if !f(key, v.as_mut()) {
                    *value = None;
                }
            }
        }
    }

    pub fn capacity(&self) -> usize {
        CHARMAP_SIZE
    }

    pub fn len(&self) -> usize {
        self.data.iter().filter(|x| x.is_some()).count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = (char, &T)> {
        self.data.iter().enumerate().filter_map(|(i, option)| {
            option.as_ref().map(|boxed| (i as u8 as char, boxed.as_ref()))
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (char, &mut T)> {
        self.data.iter_mut().enumerate().filter_map(|(i, option)| {
            option.as_mut().map(|boxed| (i as u8 as char, boxed.as_mut()))
        })
    }

    pub fn keys(&self) -> impl Iterator<Item = char> + '_ {
        self.iter().map(|(key, _)| key)
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.iter().map(|(_, value)| value)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.iter_mut().map(|(_, value)| value)
    }

    pub fn entry(&mut self, key: char) -> Entry<'_, T> {
        let index = key as usize;
        if self.data[index].is_some() {
            Entry::Occupied(OccupiedEntry { map: self, index })
        } else {
            Entry::Vacant(VacantEntry { map: self, index })
        }
    }
}

impl<T> Default for CharMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<char> for CharMap<T> {
    type Output = T;

    fn index(&self, key: char) -> &Self::Output {
        self.get(key).expect("Key not found")
    }
}

impl<T> IndexMut<char> for CharMap<T> {
    fn index_mut(&mut self, key: char) -> &mut Self::Output {
        self.get_mut(key).expect("Key not found")
    }
}

pub enum Entry<'a, T> {
    Occupied(OccupiedEntry<'a, T>),
    Vacant(VacantEntry<'a, T>),
}

pub struct OccupiedEntry<'a, T> {
    map: &'a mut CharMap<T>,
    index: usize,
}

pub struct VacantEntry<'a, T> {
    map: &'a mut CharMap<T>,
    index: usize,
}

impl<'a, T> Entry<'a, T> {
    pub fn key(&self) -> char {
        match self {
            Entry::Occupied(occupied) => occupied.index as u8 as char,
            Entry::Vacant(vacant) => vacant.index as u8 as char,
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

    pub fn insert(&mut self, value: T) -> T where T: Clone {
        std::mem::replace(&mut self.map.data[self.index], Some(Box::new(value))).unwrap().as_mut().clone()
    }

    pub fn remove(self) -> T where T: Clone {
        self.map.data[self.index].take().unwrap().as_mut().clone()
    }
}

impl<'a, T> VacantEntry<'a, T> {
    pub fn insert(self, value: T) -> &'a mut T {
        self.map.data[self.index] = Some(Box::new(value));
        self.map.data[self.index].as_mut().unwrap().as_mut()
    }
}

impl<T> IntoIterator for CharMap<T> where T: Clone {
    type Item = (char, T);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter().enumerate().filter_map(|(i, option)| {
            <Option<Box<T>> as Clone>::clone(&option).map(|boxed| (i as u8 as char, *boxed))
        }).collect::<Vec<_>>().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a CharMap<T> {
    type Item = (char, &'a T);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter().collect::<Vec<_>>().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut CharMap<T> {
    type Item = (char, &'a mut T);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut().collect::<Vec<_>>().into_iter()
    }
}

impl<T> Extend<(char, T)> for CharMap<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (char, T)>,
    {
        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}