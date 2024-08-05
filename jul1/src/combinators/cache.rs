use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;
use derivative::Derivative;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, profile_internal, RightData, Squash, U8Set};

#[derive(Clone, Default)]
#[derive(Derivative)]
#[derivative(Debug, PartialEq, Eq, Hash)]
pub struct CacheData {
    #[derivative(Debug = "ignore", PartialEq = "ignore", Hash = "ignore")]
    pub inner: Option<Rc<RefCell<CacheDataInner>>>,
}

pub struct CacheDataInner {
    pub new_parsers: HashMap<CacheKey, Rc<RefCell<CacheEntry>>>,
    pub entries: LruCache<CacheKey, Rc<RefCell<CacheEntry>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    pub combinator: Rc<Combinator>,
    pub right_data: RightData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheEntry {
    pub parser: Option<Box<Parser>>,
    pub maybe_parse_results: Option<ParseResults>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheContext {
    pub inner: Box<Combinator>,
    pub capacity: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cached {
    pub inner: Rc<Combinator>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachedParser {
    pub entry: Rc<RefCell<CacheEntry>>,
}

impl Hash for CachedParser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.entry.as_ref() as *const RefCell<CacheEntry>, state);
    }
}

#[derive(Clone)]
#[derive(Derivative)]
#[derivative(Debug, PartialEq, Eq)]
pub struct CacheContextParser {
    pub inner: Box<Parser>,
    #[derivative(Debug = "ignore", PartialEq = "ignore")]
    pub cache_data_inner: Rc<RefCell<CacheDataInner>>,
}

impl Hash for CacheContextParser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.cache_data_inner.as_ref() as *const RefCell<CacheDataInner>, state);
    }
}

impl CacheContextParser {
    fn cleanup(&mut self) {
        let mut cache_data_inner = self.cache_data_inner.borrow_mut();
        cache_data_inner.new_parsers.clear();
        cache_data_inner.entries.retain(|_, entry| !entry.borrow().maybe_parse_results.as_ref().unwrap().done);
    }
}

impl CombinatorTrait for CacheContext {
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        assert!(right_data.cache_data.inner.is_none(), "CacheContextParser already initialized");
        let cache_data_inner = Rc::new(RefCell::new(CacheDataInner {
            new_parsers: HashMap::new(),
            entries: LruCache::new(self.capacity),
        }));
        right_data.cache_data.inner = Some(cache_data_inner.clone());
        let (parser, results) = self.inner.parse(right_data, bytes);
        let mut cache_context_parser = CacheContextParser { inner: Box::new(parser), cache_data_inner };
        cache_context_parser.cleanup();
        (Parser::CacheContextParser(cache_context_parser), results)
    }
}

impl ParserTrait for CacheContextParser {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut cache_data_inner = self.cache_data_inner.borrow_mut();
        for entry in cache_data_inner.entries.values_mut() {
            entry.borrow_mut().maybe_parse_results.take();
        }
        let keys: Vec<_> = cache_data_inner.entries.keys().cloned().collect();
        for key in keys.iter().rev() {
            let entry = cache_data_inner.entries.get(key).unwrap().clone();
            let parse_results = catch_unwind(AssertUnwindSafe(|| entry.borrow_mut().parser.as_mut().unwrap().parse(bytes))).expect("CacheContextParser.steps: parse_results is None");
            entry.borrow_mut().maybe_parse_results = Some(parse_results.clone());
            cache_data_inner.entries.get(key); // Move to front
        }
        drop(cache_data_inner);
        let parse_result = self.inner.parse(bytes);
        self.cleanup();
        parse_result
    }
}

impl CombinatorTrait for Cached {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let key = CacheKey { combinator: self.inner.clone(), right_data: right_data.clone() };
        let mut cache_data_inner = right_data.cache_data.inner.as_ref().unwrap().borrow_mut();
        if let Some(entry) = cache_data_inner.entries.get(&key).cloned() {
            let parse_results = entry.borrow().maybe_parse_results.clone().expect("CachedParser.parser: parse_results is None");
            return (Parser::CachedParser(CachedParser { entry }), parse_results);
        }
        let entry = Rc::new(RefCell::new(CacheEntry {
            parser: None,
            maybe_parse_results: None,
        }));
        drop(cache_data_inner);
        let (parser, mut parse_results) = self.inner.parse(right_data.clone(), bytes);
        parse_results.squash();
        let mut cache_data_inner = right_data.cache_data.inner.as_ref().unwrap().borrow_mut();
        cache_data_inner.new_parsers.insert(key.clone(), entry.clone());
        cache_data_inner.entries.insert(key, entry.clone());
        entry.borrow_mut().parser = Some(Box::new(parser));
        entry.borrow_mut().maybe_parse_results = Some(parse_results.clone());
        (Parser::CachedParser(CachedParser { entry }), parse_results)
    }
}

impl ParserTrait for CachedParser {
    fn get_u8set(&self) -> U8Set {
        self.entry.borrow().parser.as_ref().unwrap().get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.entry.borrow().maybe_parse_results.clone().expect("CachedParser.steps: parse_results is None")
    }
}

pub fn cache_context(a: impl Into<Combinator>) -> Combinator {
    profile_internal("cache_context", CacheContext { inner: Box::new(a.into()), capacity: 100 })
}

pub fn cached(a: impl Into<Combinator>) -> Combinator {
    profile_internal("cached", Cached { inner: Rc::new(a.into()) })
}

impl From<CacheContext> for Combinator {
    fn from(value: CacheContext) -> Self {
        Combinator::CacheContext(value)
    }
}

impl From<Cached> for Combinator {
    fn from(value: Cached) -> Self {
        Combinator::Cached(value)
    }
}

// LRU Cache implementation
pub struct LruCache<K, V> {
    map: HashMap<K, (V, usize)>,
    order: Vec<K>,
    capacity: usize,
    counter: usize,
}

impl<K: Clone + Eq + Hash, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        LruCache {
            map: HashMap::new(),
            order: Vec::new(),
            capacity,
            counter: 0,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some((value, count)) = self.map.get_mut(key) {
            self.counter += 1;
            *count = self.counter;
            // Update the order after releasing the borrow on 'value'
            drop(value);
            self.update_order();
            // Get the value again after the order update
            self.map.get(key).map(|(v, _)| v)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.counter += 1;
        if self.map.len() >= self.capacity {
            if let Some(lru_key) = self.order.pop() {
                self.map.remove(&lru_key);
            }
        }
        self.map.insert(key.clone(), (value, self.counter));
        self.order.push(key);
        self.update_order();
    }

    pub fn update_order(&mut self) {
        self.order.sort_by(|a, b| {
            let count_a = self.map.get(a).map(|(_, count)| count).unwrap_or(&0);
            let count_b = self.map.get(b).map(|(_, count)| count).unwrap_or(&0);
            count_b.cmp(count_a)
        });
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.map.values().map(|(v, _)| v)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.map.values_mut().map(|(v, _)| v)
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.order.iter()
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.map.retain(|k, (v, _)| f(k, v));
        self.order.retain(|k| self.map.contains_key(k));
    }
}