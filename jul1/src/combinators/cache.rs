use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::num::NonZero;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;

use derivative::Derivative;
use lru::LruCache;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, profile_internal, RightData, Squash, U8Set};
use crate::VecX;

#[derive(Derivative)]
#[derivative(Debug, Default, Clone, PartialEq, Eq)]
pub struct CacheData {
    #[derivative(Debug = "ignore", PartialEq = "ignore")]
    pub inner: Option<Rc<RefCell<CacheDataInner>>>,
}

#[derive(Debug)]
pub struct CacheDataInner {
    pub new_parsers: LruCache<CacheKey, Rc<RefCell<CacheEntry>>>,
    pub entries: Vec<Rc<RefCell<CacheEntry>>>,
}

#[derive(Debug, Clone, Eq)]
pub struct CacheKey {
    pub combinator: Rc<Combinator>,
    pub right_data: RightData,
}

#[derive(Derivative)]
#[derivative(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheEntry {
    pub parser: Option<Box<Parser>>,
    pub maybe_parse_results: Option<ParseResults>,
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
        // Hash the pointer
        std::ptr::hash(self.entry.as_ref() as *const RefCell<CacheEntry>, state);
    }
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

impl PartialEq for CachedParser {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.entry, &other.entry)
    }
}

#[derive(Derivative)]
#[derivative(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheContextParser {
    pub inner: Box<Parser>,
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    pub cache_data_inner: Rc<RefCell<CacheDataInner>>,
}

impl CacheContextParser {
    fn cleanup(&mut self) {
        let mut cache_data_inner = self.cache_data_inner.borrow_mut();
        cache_data_inner.new_parsers.clear();
        cache_data_inner.entries.retain(|entry| !entry.borrow().maybe_parse_results.as_ref().unwrap().done);
    }
}

impl CombinatorTrait for CacheContext {
    fn parse(&self, mut right_data: Box<RightData>, bytes: &[u8]) -> (Parser, ParseResults) {
        println!("RightData size: {}", std::mem::size_of::<RightData>());
        println!("ParseResults size: {}", std::mem::size_of::<ParseResults>());
        assert!(right_data.cache_data.inner.is_none(), "CacheContextParser already initialized");
        let cache_data_inner = Rc::new(RefCell::new(CacheDataInner {
            new_parsers: LruCache::new(NonZero::new(64).unwrap()),
            entries: Vec::new(),
        }));
        right_data.cache_data.inner = Some(cache_data_inner.clone());
        let (parser, results) = self.inner.parse(right_data, bytes);
        cache_data_inner.borrow_mut().entries.reverse();
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
        self.cache_data_inner.borrow_mut().entries.iter_mut().for_each(|entry| {
            entry.borrow_mut().maybe_parse_results.take();
        });
        let num_entries_initial = self.cache_data_inner.borrow().entries.len().clone();
        for i in (0..num_entries_initial).rev() {
            let entry = self.cache_data_inner.borrow().entries[i].clone();
            let parse_results = catch_unwind(AssertUnwindSafe(|| entry.borrow_mut().parser.as_mut().unwrap().parse(bytes))).expect("CacheContextParser.steps: parse_results is None");
            entry.borrow_mut().maybe_parse_results = Some(parse_results.clone());
        }
        let parse_result = self.inner.parse(bytes);
        let mut new_entries = self.cache_data_inner.borrow_mut().entries.split_off(num_entries_initial);
        new_entries.reverse();
        self.cache_data_inner.borrow_mut().entries.append(&mut new_entries);
        self.cleanup();
        parse_result
    }
}

impl CombinatorTrait for Cached {
    fn parse(&self, right_data: Box<RightData>, bytes: &[u8]) -> (Parser, ParseResults) {
        let key = CacheKey { combinator: self.inner.clone(), right_data: *right_data.clone() };
        if let Some(entry) = right_data.cache_data.inner.as_ref().unwrap().borrow_mut().new_parsers.get(&key).cloned() {
            let parse_results = entry.borrow().maybe_parse_results.clone().expect("CachedParser.parser: parse_results is None");
            return (Parser::CachedParser(CachedParser { entry }), parse_results);
        }
        let entry = Rc::new(RefCell::new(CacheEntry {
            parser: None,
            maybe_parse_results: None,
        }));
        let (parser, mut parse_results) = self.inner.parse(right_data.clone(), bytes);
        parse_results.squash();
        let mut cache_data_inner = right_data.cache_data.inner.as_ref().unwrap().borrow_mut();
        cache_data_inner.new_parsers.put(key.clone(), entry.clone());
        if !parse_results.done {
            cache_data_inner.entries.push(entry.clone());
        }
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
