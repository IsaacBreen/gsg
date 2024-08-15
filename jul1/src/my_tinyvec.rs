pub enum FastVec<T> {
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
}