use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::num::NonZeroUsize;
use std::rc::Rc;

use derivative::Derivative;
use lru::LruCache;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, profile, profile_internal, RightData, Squash, U8Set};

thread_local! {
    pub static GLOBAL_CACHE: RefCell<GlobalCache> = RefCell::new(GlobalCache::new());
}

#[derive(Debug)]
struct GlobalCache {
    new_parsers: LruCache<CacheKey, Rc<RefCell<CacheEntry>>>,
    pub(crate) entries: Vec<Rc<RefCell<CacheEntry>>>,
}

impl GlobalCache {
    fn new() -> Self {
        Self {
            new_parsers: LruCache::new(NonZeroUsize::new(64).unwrap()),
            entries: Vec::new(),
        }
    }

    fn cleanup(&mut self) {
        self.new_parsers.clear();
        self.entries.retain(|entry| !entry.borrow().maybe_parse_results.as_ref().unwrap().done());
    }
}

#[derive(Debug, Clone, Eq)]
struct CacheKey {
    combinator: Rc<Combinator>,
    right_data: RightData,
}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self.combinator.as_ref()).hash(state);
        self.right_data.hash(state);
    }
}

impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.combinator, &other.combinator) && self.right_data == other.right_data
    }
}

#[derive(Derivative)]
#[derivative(Debug, Clone, PartialEq, Eq, Hash)]
struct CacheEntry {
    pub(crate) parser: Option<Box<Parser>>,
    maybe_parse_results: Option<ParseResults>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheContext {
    pub inner: Box<Combinator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cached {
    pub inner: Rc<Combinator>,
}

#[derive(Debug, Clone, Eq)]
pub struct CachedParser {
    pub entry: Rc<RefCell<CacheEntry>>,
}

impl Hash for CachedParser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.entry.as_ref() as *const RefCell<CacheEntry>, state);
    }
}

impl PartialEq for CachedParser {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.entry, &other.entry)
    }
}

#[derive(Derivative)]
#[derivative(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheContextParser {
    pub inner: Box<Parser>,
}

impl CombinatorTrait for CacheContext {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        GLOBAL_CACHE.with(|cache| {
            let (parser, results) = self.inner.parse(right_data, bytes);
            let mut global_cache = cache.borrow_mut();
            global_cache.entries.reverse();
            global_cache.cleanup();
            let cache_context_parser = CacheContextParser { inner: Box::new(parser) };
            (Parser::CacheContextParser(cache_context_parser), results)
        })
    }
}

impl ParserTrait for CacheContextParser {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        GLOBAL_CACHE.with(|cache| {
            cache.borrow_mut().entries.iter_mut().for_each(|entry| {
                entry.borrow_mut().maybe_parse_results.take();
            });
            let num_entries_initial = cache.borrow_mut().entries.len();
            for i in (0..num_entries_initial).rev() {
                let entry_refcell = cache.borrow_mut().entries[i].clone();
                let mut entry = entry_refcell.borrow_mut();
                let parse_results = entry.parser.as_mut().unwrap().parse(bytes);
                entry.maybe_parse_results = Some(parse_results);
            }

            let parse_result = self.inner.parse(bytes);

            let mut global_cache = cache.borrow_mut();
            let mut new_entries = global_cache.entries.split_off(num_entries_initial);
            new_entries.reverse();
            global_cache.entries.append(&mut new_entries);
            global_cache.cleanup();
            parse_result
        })
    }
}

impl CombinatorTrait for Cached {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        GLOBAL_CACHE.with(|cache| {
            let key = CacheKey { combinator: self.inner.clone(), right_data: right_data.clone() };

            let mut global_cache = cache.borrow_mut();
            profile!("Cached.parse: check cache", {
                if let Some(entry) = global_cache.new_parsers.get(&key).cloned() {
                    profile!("Cached.parse: cache hit", {});
                    let parse_results = entry.borrow().maybe_parse_results.clone().expect("CachedParser.parser: parse_results is None");
                    return (Parser::CachedParser(CachedParser { entry }), parse_results);
                }
            });
            drop(global_cache);

            profile!("Cached.parse: cache miss", {});
            let entry = Rc::new(RefCell::new(CacheEntry {
                parser: None,
                maybe_parse_results: None,
            }));
            let (parser, mut parse_results) = profile!("Cached.parse: inner.parse", self.inner.parse(right_data, bytes));
            profile!("Cached.parse: parse_results.squash", parse_results.squash());

            let mut global_cache = cache.borrow_mut();
            global_cache.new_parsers.put(key, entry.clone());
            if !parse_results.done() {
                global_cache.entries.push(entry.clone());
            }
            *entry.borrow_mut() = CacheEntry { parser: Some(Box::new(parser)), maybe_parse_results: Some(parse_results.clone()) };
            (Parser::CachedParser(CachedParser { entry }), parse_results)
        })
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
    profile_internal("cache_context", CacheContext { inner: Box::new(a.into()) })
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