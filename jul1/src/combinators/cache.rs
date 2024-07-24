use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{DynCombinator, ParseResults, ParserTrait};

#[derive(Debug, Clone, PartialEq, Default, PartialOrd, Ord, Eq)]
pub struct CacheData {
    pub inner: Rc<RefCell<CacheDataInner>>,
}

impl Hash for CacheData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

#[derive(Default)]
pub struct CacheDataInner {
    pub memoed: BTreeMap<CachedCombinator, Rc<RefCell<ParseResults>>>,
}

impl Debug for CacheDataInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GlobalDataInner:")?;
        Ok(())
    }
}

// #[derive(Clone, PartialEq, Default, Hash, PartialOrd, Ord, Eq)]
impl PartialEq for CacheDataInner {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for CacheDataInner {}

impl PartialOrd for CacheDataInner {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CacheDataInner {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}

#[derive(Clone)]
pub struct CachedCombinator {
    pub(crate) inner: Rc<DynCombinator>,
}

impl Hash for CachedCombinator {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl PartialEq for CachedCombinator {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for CachedCombinator {}

impl PartialOrd for CachedCombinator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CachedCombinator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}