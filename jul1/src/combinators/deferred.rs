use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::*;
use crate::VecX;

thread_local! {
    static COMBINATOR_CACHE: RefCell<HashMap<usize, Rc<Combinator>>> = RefCell::new(HashMap::new());
}

#[derive(Clone)]
pub struct Deferred {
    pub(crate) f: fn() -> Combinator,
    cache_key: usize,
}

impl Hash for Deferred {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cache_key.hash(state);
    }
}

impl PartialEq for Deferred {
    fn eq(&self, other: &Self) -> bool {
        self.cache_key == other.cache_key
    }
}

impl Eq for Deferred {}

impl Debug for Deferred {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Deferred({})", self.cache_key)
    }
}

impl CombinatorTrait for Deferred {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let combinator = COMBINATOR_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            cache.entry(self.cache_key)
                .or_insert_with(|| Rc::new((self.f)()))
                .clone()
        });
        assert_eq!(&(self.f)(), combinator.as_ref());
        combinator.parse(right_data, bytes)
    }
}

pub fn deferred(f: fn() -> Combinator) -> Combinator {
    let cache_key = &f as *const _ as usize;
    Deferred { f, cache_key }.into()
}

impl From<&fn () -> Combinator> for Combinator {
    fn from(value: &fn() -> Combinator) -> Self {
        deferred(*value)
    }
}

impl From<Deferred> for Combinator {
    fn from(value: Deferred) -> Self {
        Combinator::Deferred(value)
    }
}