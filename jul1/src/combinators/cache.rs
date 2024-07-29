use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;

use derivative::Derivative;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, RightData, Squash, Stats};

#[derive(Clone, PartialEq, Default, Eq)]
pub struct CacheData {
    pub inner: Option<Rc<RefCell<CacheDataInner>>>,
}

#[derive(Debug, Clone, PartialEq, Default, Eq)]
pub struct CacheDataInner {
    pub new_parsers: HashMap<CacheKey, Rc<RefCell<CacheEntry>>>,
    pub entries: Vec<Rc<RefCell<CacheEntry>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    pub combinator: Rc<Combinator>,
    pub right_data: RightData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheEntry {
    pub parser: Option<Box<Parser>>,
    pub maybe_parse_results: Option<ParseResults>,
    pub num: usize,
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
    pub num: usize,
}

impl Hash for CachedParser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entry.borrow().num.hash(state);
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
        self.cache_data_inner.borrow_mut().new_parsers.clear();
        self.cache_data_inner.borrow_mut().entries.retain(|entry| !entry.borrow().maybe_parse_results.as_ref().unwrap().done);
    }
}

impl CombinatorTrait for CacheContext {
    fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
        assert!(right_data.cache_data.inner.is_none(), "CacheContextParser already initialized");
        let cache_data_inner = Rc::new(RefCell::new(CacheDataInner::default()));
        right_data.cache_data.inner = Some(cache_data_inner.clone());
        let (parser, results) = self.inner.parser(right_data);
        // Reverse the order of entries
        cache_data_inner.borrow_mut().entries.reverse();
        let mut cache_context_parser = CacheContextParser { inner: Box::new(parser), cache_data_inner };
        cache_context_parser.cleanup();
        (Parser::CacheContextParser(cache_context_parser), results)
    }
}

impl ParserTrait for CacheContextParser {
    fn step(&mut self, c: u8) -> ParseResults {
        for entry in self.cache_data_inner.borrow_mut().entries.iter() {
            entry.borrow_mut().maybe_parse_results.take();
        }
        let num_entries_initial = self.cache_data_inner.borrow().entries.len().clone();
        for i in (0..num_entries_initial).rev() {
            let mut entry = {
                let binding = self.cache_data_inner.borrow();
                binding.entries[i].clone()
            };
            let mut parse_results = catch_unwind(AssertUnwindSafe(|| entry.borrow_mut().parser.as_mut().unwrap().step(c))).expect(format!("CacheContextParser.step: parse_results is None for entry number {} at index {}", entry.borrow().num, i).as_str());
            parse_results.squash();
            entry.borrow_mut().maybe_parse_results.replace(parse_results.clone());
        }
        let parse_result = self.inner.step(c);
        // Reverse the order of new entries
        let mut new_entries = self.cache_data_inner.borrow_mut().entries.split_off(num_entries_initial);
        new_entries.reverse();
        self.cache_data_inner.borrow_mut().entries.append(&mut new_entries);
        self.cleanup();
        parse_result
    }
}

impl CombinatorTrait for Cached {
    fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
        for (key, entry) in right_data.cache_data.inner.as_ref().unwrap().borrow().new_parsers.iter() {
            if Rc::ptr_eq(&key.combinator, &self.inner) && key.right_data == right_data {
                let parse_results = entry.borrow().maybe_parse_results.clone().expect("CachedParser.parser: parse_results is None");
                return (
                    Parser::CachedParser(CachedParser {
                        entry: entry.clone(),
                        num: entry.borrow().num,
                    }),
                    parse_results
                );
            }
        }

        let entry = Rc::new(RefCell::new(CacheEntry {
            parser: None,
            maybe_parse_results: None,
            num: 0
        }));
        let key = CacheKey {
            combinator: self.inner.clone(),
            right_data: right_data.clone()
        };

        let (parser, mut parse_results) = self.inner.parser(right_data.clone());
        parse_results.squash();
        {
            let mut cache_data_inner = right_data.cache_data.inner.as_ref().unwrap().borrow_mut();
            cache_data_inner.new_parsers.insert(key, entry.clone());
            entry.borrow_mut().num = cache_data_inner.entries.len();
            cache_data_inner.entries.push(entry.clone());
        }
        entry.borrow_mut().parser = Some(Box::new(parser));
        entry.borrow_mut().maybe_parse_results = Some(parse_results.clone());
        let num = entry.borrow().num;
        (Parser::CachedParser(CachedParser { entry, num }), parse_results)
    }
}

impl ParserTrait for CachedParser {
    fn step(&mut self, c: u8) -> ParseResults {
        self.entry.borrow().maybe_parse_results.clone().expect(format!("CachedParser.step: parse_results is None for entry number {} (self.num = {})", self.entry.borrow().num, self.num).as_str())
    }
}

pub fn cache_context(a: impl Into<Combinator>) -> CacheContext {
    CacheContext { inner: Box::new(a.into()) }
}

pub fn cached(a: impl Into<Combinator>) -> Cached {
    Cached { inner: Rc::new(a.into()) }
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
