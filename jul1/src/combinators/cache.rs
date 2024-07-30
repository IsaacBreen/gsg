use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
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
    pub firsts: HashMap<CacheKey, ParseResults>,
    pub new_parsers: HashMap<CacheKey, Rc<RefCell<CacheEntry>>>,
    // pub keys_hit_this_step: HashSet<CacheKey>,
    pub entries: Vec<Rc<RefCell<CacheEntry>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheKey {
    pub combinator: Rc<Combinator>,
    pub right_data: RightData,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CacheStatus {
    Uninitialized,
    Active,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheEntry {
    pub key: CacheKey,
    pub parser: Option<Box<Parser>>,
    pub maybe_parse_results: Option<ParseResults>,
    pub status: CacheStatus,
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
        self.entry.borrow().hash(state);
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
        // TODO: HERE'S THE ISSUE. entry contains a parser. we clone that. but that parser contains cached parsers, and those are tied to a previous position.
        // cache_data_inner.new_parsers.values_mut().for_each(|entry| {
        //     let cloned = entry.borrow().clone();
        //     *entry = Rc::new(RefCell::new(cloned));
        // });
        cache_data_inner.new_parsers.clear();
        // cache_data_inner.keys_hit_this_step.clear();
        cache_data_inner.entries.retain(|entry| !entry.borrow().maybe_parse_results.as_ref().unwrap().done);
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
        let num_entries_initial = self.cache_data_inner.borrow().entries.len().clone();
        for i in (0..num_entries_initial).rev() {
            let mut entry = {
                let binding = self.cache_data_inner.borrow();
                binding.entries[i].clone()
            };
            if entry.borrow().status == CacheStatus::Uninitialized {
                assert!(entry.borrow().parser.is_none());
                let (parser, parse_results) = entry.borrow().key.combinator.parser(entry.borrow().key.right_data.clone());
                assert!(entry.borrow().maybe_parse_results == Some(parse_results.clone()), "CacheContextParser.step: parse_results mismatch: maybe_parse_results: {:?}, parse_results: {:?}", entry.borrow().maybe_parse_results, parse_results);
                entry.borrow_mut().maybe_parse_results.replace(parse_results.clone());
                entry.borrow_mut().parser = Some(Box::new(parser));
                entry.borrow_mut().status = CacheStatus::Active;
            }
        }
        for entry in self.cache_data_inner.borrow_mut().entries.iter() {
            entry.borrow_mut().maybe_parse_results.take();
        }
        let num_entries_initial = self.cache_data_inner.borrow().entries.len().clone();
        for i in (0..num_entries_initial).rev() {
            let mut entry = {
                let binding = self.cache_data_inner.borrow();
                binding.entries[i].clone()
            };
            let mut parse_results = catch_unwind(AssertUnwindSafe(|| entry.borrow_mut().parser.as_mut().unwrap().step(c))).expect(format!("CacheContextParser.step: parse_results is None").as_str());
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
        let key = CacheKey {
            combinator: self.inner.clone(),
            right_data: right_data.clone()
        };

        {
            let mut cache_data_inner = right_data.cache_data.inner.as_ref().unwrap().borrow_mut();
            let maybe_entry = cache_data_inner.new_parsers.get(&key).cloned();
            if let Some(entry) = maybe_entry {
                // if !cache_data_inner.keys_hit_this_step.contains(&key) {
                //     cache_data_inner.keys_hit_this_step.insert(key.clone());
                //     cache_data_inner.entries.push(entry.clone());
                // }
                let parse_results = entry.borrow().maybe_parse_results.clone().expect("CachedParser.parser: parse_results is None");
                return (
                    Parser::CachedParser(CachedParser {
                        entry: entry.clone(),
                    }),
                    parse_results
                );
            }

            if let Some(parse_results) = cache_data_inner.firsts.get(&key).cloned() {
                let entry = Rc::new(RefCell::new(CacheEntry {
                    key: key.clone(),
                    parser: None,
                    maybe_parse_results: Some(parse_results.clone()),
                    status: CacheStatus::Uninitialized,
                }));
                cache_data_inner.new_parsers.insert(key.clone(), entry.clone());
                cache_data_inner.entries.push(entry.clone());
                return (Parser::CachedParser(CachedParser { entry }), parse_results.clone());
            }
        }

        let entry = Rc::new(RefCell::new(CacheEntry {
            key: key.clone(),
            parser: None,
            maybe_parse_results: None,
            status: CacheStatus::Active,
        }));

        let (parser, mut parse_results) = self.inner.parser(right_data.clone());
        parse_results.squash();
        {
            let mut cache_data_inner = right_data.cache_data.inner.as_ref().unwrap().borrow_mut();
            // cache_data_inner.keys_hit_this_step.insert(key.clone());
            cache_data_inner.firsts.insert(key.clone(), parse_results.clone());
            cache_data_inner.new_parsers.insert(key.clone(), entry.clone());
            cache_data_inner.entries.push(entry.clone());
        }
        entry.borrow_mut().parser = Some(Box::new(parser));
        entry.borrow_mut().maybe_parse_results = Some(parse_results.clone());
        (Parser::CachedParser(CachedParser { entry }), parse_results)
    }
}

impl ParserTrait for CachedParser {
    fn step(&mut self, c: u8) -> ParseResults {
        self.entry.borrow().maybe_parse_results.clone().expect(format!("CachedParser.step: parse_results is None").as_str())
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
