use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::rc::Rc;

use derivative::Derivative;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, RightData, Squash};

#[derive(Clone, PartialEq, Default, Eq)]
pub struct CacheData {
    pub inner: Option<Rc<RefCell<CacheDataInner>>>,
}

#[derive(Debug, Clone, PartialEq, Default, Eq)]
pub struct CacheDataInner {
    pub new_parsers: HashMap<CacheKey, Rc<RefCell<CacheEntry>>>,
    pub entries: Vec<Rc<RefCell<CacheEntry>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheKey {
    pub combinator: Rc<Combinator>,
    pub right_data: RightData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachedParser {
    pub entry: Rc<RefCell<CacheEntry>>,
}

impl Hash for CachedParser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // self.entry.borrow().hash(state);
    }
}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self.combinator.as_ref()).hash(state);
        self.right_data.hash(state);
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
    fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
        assert!(right_data.cache_data.inner.is_none(), "CacheContextParser already initialized");
        let cache_data_inner = Rc::new(RefCell::new(CacheDataInner::default()));
        right_data.cache_data.inner = Some(cache_data_inner.clone());
        let (parser, results) = self.inner.parser(right_data);
        cache_data_inner.borrow_mut().entries.reverse();
        let mut cache_context_parser = CacheContextParser { inner: Box::new(parser), cache_data_inner };
        cache_context_parser.cleanup();
        (Parser::CacheContextParser(cache_context_parser), results)
    }
}

impl ParserTrait for CacheContextParser {
    fn step(&mut self, c: u8) -> ParseResults {
        self.cache_data_inner.borrow_mut().entries.iter_mut().for_each(|entry| {
            entry.borrow_mut().maybe_parse_results.take();
        });
        let num_entries_initial = self.cache_data_inner.borrow().entries.len().clone();
        for i in (0..num_entries_initial).rev() {
            let entry = self.cache_data_inner.borrow().entries[i].clone();
            let parse_results = catch_unwind(AssertUnwindSafe(|| entry.borrow_mut().parser.as_mut().unwrap().step(c))).expect("CacheContextParser.step: parse_results is None");
            entry.borrow_mut().maybe_parse_results = Some(parse_results.clone());
        }
        let parse_result = self.inner.step(c);
        let mut new_entries = self.cache_data_inner.borrow_mut().entries.split_off(num_entries_initial);
        new_entries.reverse();
        self.cache_data_inner.borrow_mut().entries.append(&mut new_entries);
        self.cleanup();
        parse_result
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        self.cache_data_inner.borrow_mut().entries.iter_mut().for_each(|entry| {
            entry.borrow_mut().maybe_parse_results.take();
        });
        let num_entries_initial = self.cache_data_inner.borrow().entries.len().clone();
        for i in (0..num_entries_initial).rev() {
            let entry = self.cache_data_inner.borrow().entries[i].clone();
            let parse_results = catch_unwind(AssertUnwindSafe(|| entry.borrow_mut().parser.as_mut().unwrap().steps(bytes))).expect("CacheContextParser.steps: parse_results is None");
            entry.borrow_mut().maybe_parse_results = Some(parse_results.clone());
        }
        let parse_result = self.inner.steps(bytes);
        let mut new_entries = self.cache_data_inner.borrow_mut().entries.split_off(num_entries_initial);
        new_entries.reverse();
        self.cache_data_inner.borrow_mut().entries.append(&mut new_entries);
        self.cleanup();
        parse_result
    }
}

impl CombinatorTrait for Cached {
    fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
        let key = CacheKey { combinator: self.inner.clone(), right_data: right_data.clone() };
        if let Some(entry) = right_data.cache_data.inner.as_ref().unwrap().borrow().new_parsers.get(&key).cloned() {
            let parse_results = entry.borrow().maybe_parse_results.clone().expect("CachedParser.parser: parse_results is None");
            return (Parser::CachedParser(CachedParser { entry }), parse_results);
        }
        let entry = Rc::new(RefCell::new(CacheEntry {
            parser: None,
            maybe_parse_results: None,
        }));
        let (parser, mut parse_results) = self.inner.parser(right_data.clone());
        parse_results.squash();
        let mut cache_data_inner = right_data.cache_data.inner.as_ref().unwrap().borrow_mut();
        cache_data_inner.new_parsers.insert(key.clone(), entry.clone());
        cache_data_inner.entries.push(entry.clone());
        entry.borrow_mut().parser = Some(Box::new(parser));
        entry.borrow_mut().maybe_parse_results = Some(parse_results.clone());
        (Parser::CachedParser(CachedParser { entry }), parse_results)
    }
}

impl ParserTrait for CachedParser {
    fn step(&mut self, _c: u8) -> ParseResults {
        self.entry.borrow().maybe_parse_results.clone().expect("CachedParser.step: parse_results is None")
    }

    fn steps(&mut self, _bytes: &[u8]) -> ParseResults {
        self.entry.borrow().maybe_parse_results.clone().expect("CachedParser.steps: parse_results is None")
    }
}
//
// pub fn cache_context(a: impl Into<Combinator>) -> CacheContext {
//     CacheContext { inner: Box::new(a.into()) }
// }
//
// pub fn cached(a: impl Into<Combinator>) -> Cached {
//     Cached { inner: Rc::new(a.into()) }
// }

pub fn cache_context<A: Into<Combinator>>(a: A) -> Combinator {
    a.into()
}

pub fn cached<A: Into<Combinator>>(a: A) -> Combinator {
    a.into()
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
