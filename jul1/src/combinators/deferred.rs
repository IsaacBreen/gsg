use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::*;
use crate::VecX;

thread_local! {
    static COMBINATOR_CACHE: RefCell<HashMap<Deferred, Rc<Combinator>>> = RefCell::new(HashMap::new());
}

#[derive(Clone)]
pub struct Deferred {
    pub(crate) f: &'static dyn Fn() -> Combinator,
}

impl Hash for Deferred {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.f, state);
    }
}

impl PartialEq for Deferred {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.f, other.f)
    }
}

impl Eq for Deferred {}

impl Debug for Deferred {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Deferred")
            .field("f", &std::ptr::addr_of!(self.f))
            .finish()
    }
}

impl CombinatorTrait for Deferred {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let combinator = COMBINATOR_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            cache.entry(self.clone())
                .or_insert_with(|| profile!("Deferred init", Rc::new((self.f)())))
                .clone()
        });
        combinator.parse(right_data, bytes)
    }
}

pub fn deferred(f: &'static impl Fn() -> Combinator) -> Combinator {
    Deferred { f }.into()
}

impl<T> From<&'static T> for Combinator
where
    T: Fn() -> Combinator
{
    fn from(value: &'static T) -> Self {
        deferred(value)
    }
}

impl From<Deferred> for Combinator {
    fn from(value: Deferred) -> Self {
        Combinator::Deferred(value)
    }
}