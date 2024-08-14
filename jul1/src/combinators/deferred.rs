use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::{LazyCell, RefCell, UnsafeCell};
use std::collections::HashMap;
use crate::*;
use crate::VecX;

thread_local! {
    static DEFERRED_CACHE: RefCell<HashMap<usize, Deferred>> = RefCell::new(HashMap::new());
}

#[derive(Clone)]
pub struct Deferred {
    pub(crate) inner: Rc<LazyCell<Combinator, Box<dyn FnOnce() -> Combinator>>>,
}

impl Hash for Deferred {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::addr_of!(self.inner) as usize.hash(state);
    }
}

impl PartialEq for Deferred {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Deferred {}

impl Debug for Deferred {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Deferred")
            .finish()
    }
}

impl CombinatorTrait for Deferred {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        self.inner.parse(right_data, bytes)
    }
}

pub fn deferred(f: impl FnOnce() -> Combinator + 'static) -> Combinator {
    DEFERRED_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        let f_addr = std::ptr::addr_of!(f) as usize;
        cache.entry(f_addr)
            .or_insert_with(|| Deferred { inner: Rc::new(LazyCell::new(Box::new(f))) })
            .clone().into()
    })
}

impl<T> From<T> for Combinator
where
    T: FnOnce() -> Combinator + 'static
{
    fn from(value: T) -> Self {
        deferred(value)
    }
}

impl From<Deferred> for Combinator {
    fn from(value: Deferred) -> Self {
        Combinator::Deferred(value)
    }
}
