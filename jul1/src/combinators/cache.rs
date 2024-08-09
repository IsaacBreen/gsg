use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::num::NonZero;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;

use derivative::Derivative;
use lru::LruCache;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, profile, profile_internal, RightData, Squash, U8Set};
use crate::VecX;

#[derive(Derivative)]
#[derivative(Debug, Default, Clone, PartialEq, Eq)]
pub struct CacheData<'a> {
    #[derivative(Debug = "ignore", PartialEq = "ignore")]
    pub inner: Option<Rc<RefCell<CacheDataInner<'a>>>>,
}

#[derive(Debug)]
pub struct CacheDataInner<'a> {
    pub new_parsers: LruCache<CacheKey<'a>, Rc<RefCell<CacheEntry<'a>>>>,
    pub entries: Vec<Rc<RefCell<CacheEntry<'a>>>>,
}

#[derive(Debug, Clone, Eq)]
pub struct CacheKey<'a> {
    pub combinator: Rc<Combinator<'a>>,
    pub right_data: RightData,
}

#[derive(Derivative)]
#[derivative(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheEntry<'a> {
    pub parser: Option<Box<Parser<'a>>>,
    pub maybe_parse_results: Option<ParseResults>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheContext<'a> {
    pub inner: Box<Combinator<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cached<'a> {
    pub inner: Rc<Combinator<'a>>,
}

#[derive(Debug, Clone, Eq)]
pub struct CachedParser<'a> {
    pub entry: Rc<RefCell<CacheEntry<'a>>>,
}

impl Hash for CachedParser<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the pointer
        std::ptr::hash(self.entry.as_ref() as *const RefCell<CacheEntry>, state);
    }
}

impl Hash for CacheKey<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self.combinator.as_ref()).hash(state);
        self.right_data.hash(state);
    }
}

impl PartialEq for CacheKey<'_> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.combinator, &other.combinator) && self.right_data == other.right_data
    }
}

impl PartialEq for CachedParser<'_> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.entry, &other.entry)
    }
}

#[derive(Derivative)]
#[derivative(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheContextParser<'a> {
    pub inner: Box<Parser<'a>>,
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    pub cache_data_inner: Rc<RefCell<CacheDataInner<'a>>>,
}

impl CacheDataInner<'_> {
    fn cleanup(&mut self) {
        self.new_parsers.clear();
        self.entries.retain(|entry| !entry.borrow().maybe_parse_results.as_ref().unwrap().done());
    }
}

impl CombinatorTrait<'_> for CacheContext<'_> {
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        println!("RightData size in bytes: {}", std::mem::size_of::<RightData>());
        println!("ParseResults size in bytes: {}", std::mem::size_of::<ParseResults>());
        assert!(right_data.right_data_inner.cache_data.inner.is_none(), "CacheContextParser already initialized");
        let cache_data_inner = Rc::new(RefCell::new(CacheDataInner {
            new_parsers: LruCache::new(NonZero::new(64).unwrap()),
            entries: Vec::new(),
        }));
        Rc::make_mut(&mut right_data.right_data_inner).cache_data.inner = Some(cache_data_inner.clone());
        let (parser, results) = self.inner.parse(right_data, bytes);
        let mut inner = cache_data_inner.borrow_mut();
        inner.entries.reverse();
        inner.cleanup();
        drop(inner);
        let cache_context_parser = CacheContextParser { inner: Box::new(parser), cache_data_inner };
        (Parser::CacheContextParser(cache_context_parser), results)
    }
}

impl ParserTrait for CacheContextParser<'_> {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.cache_data_inner.borrow_mut().entries.iter_mut().for_each(|entry| {
            entry.borrow_mut().maybe_parse_results.take();
        });
        let num_entries_initial = self.cache_data_inner.borrow().entries.len().clone();
        for i in (0..num_entries_initial).rev() {
            let entry_refcell = self.cache_data_inner.borrow().entries[i].clone();
            let mut entry = entry_refcell.borrow_mut();
            let parse_results = entry.parser.as_mut().unwrap().parse(bytes);
            entry.maybe_parse_results = Some(parse_results);
        }
        let parse_result = self.inner.parse(bytes);
        let mut inner = self.cache_data_inner.borrow_mut();
        let mut new_entries = inner.entries.split_off(num_entries_initial);
        new_entries.reverse();
        inner.entries.append(&mut new_entries);
        inner.cleanup();
        parse_result
    }
}

impl CombinatorTrait<'_> for Cached<'_> {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let cache_data_inner_refcell = right_data.right_data_inner.cache_data.inner.as_ref().unwrap().clone();
        let key = CacheKey { combinator: self.inner.clone(), right_data };
        profile!("Cached.parse: check cache", {
            if let Some(entry) = cache_data_inner_refcell.borrow_mut().new_parsers.get(&key).cloned() {
                profile!("Cached.parse: cache hit", {});
                let parse_results = entry.borrow().maybe_parse_results.clone().expect("CachedParser.parser: parse_results is None");
                return (Parser::CachedParser(CachedParser { entry }), parse_results);
            }
        });
        profile!("Cached.parse: cache miss", {});
        let entry = Rc::new(RefCell::new(CacheEntry {
            parser: None,
            maybe_parse_results: None,
        }));
        let (parser, mut parse_results) = profile!("Cached.parse: inner.parse", self.inner.parse(key.right_data.clone(), bytes));
        profile!("Cached.parse: parse_results.squash", parse_results.squash());
        let mut cache_data_inner = cache_data_inner_refcell.borrow_mut();
        cache_data_inner.new_parsers.put(key, entry.clone());
        if !parse_results.done() {
            cache_data_inner.entries.push(entry.clone());
        }
        *entry.borrow_mut() = CacheEntry { parser: Some(Box::new(parser)), maybe_parse_results: Some(parse_results.clone()) };
        (Parser::CachedParser(CachedParser { entry }), parse_results)
    }
}

impl ParserTrait for CachedParser<'_> {
    fn get_u8set(&self) -> U8Set {
        self.entry.borrow().parser.as_ref().unwrap().get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.entry.borrow().maybe_parse_results.clone().expect("CachedParser.steps: parse_results is None")
    }
}

pub fn cache_context<'a>(a: impl Into<Combinator<'a>>) -> Combinator<'a> {
    profile_internal("cache_context", CacheContext { inner: Box::new(a.into()) })
}

pub fn cached<'a>(a: impl Into<Combinator<'a>>) -> Combinator<'a> {
    profile_internal("cached", Cached { inner: Rc::new(a.into()) })
}

impl From<CacheContext<'_>> for Combinator<'_> {
    fn from(value: CacheContext) -> Self {
        Combinator::CacheContext(value)
    }
}

impl From<Cached<'_>> for Combinator<'_> {
    fn from(value: Cached) -> Self {
        Combinator::Cached(value)
    }
}
