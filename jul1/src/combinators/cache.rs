use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;

use derivative::Derivative;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, RightData, Squash, U8Set, UpData};

#[derive(Clone, PartialEq, Default, Eq)]
pub struct CacheData {    pub inner: Option<Rc<RefCell<CacheDataInner>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheDataInner {
    pub new_parsers: HashMap<CacheKey, Rc<RefCell<CacheEntry>>>,
    pub entries: Vec<Rc<RefCell<CacheEntry>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
        let cache_data_inner = Rc::new(RefCell::new(CacheDataInner {
            new_parsers: HashMap::new(),
            entries: Vec::new(),
        }));
        right_data.cache_data.inner = Some(cache_data_inner.clone());
        let (parser, results) = self.inner.parser(right_data);
        cache_data_inner.borrow_mut().entries.reverse();
        let mut cache_context_parser = CacheContextParser { inner: Box::new(parser), cache_data_inner };
        cache_context_parser.cleanup();
        (Parser::CacheContextParser(cache_context_parser), results)
    }

    fn parser_with_steps(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        assert!(right_data.cache_data.inner.is_none(), "CacheContextParser already initialized");
        let cache_data_inner = Rc::new(RefCell::new(CacheDataInner {
            new_parsers: HashMap::new(),
            entries: Vec::new(),
        }));
        right_data.cache_data.inner = Some(cache_data_inner.clone());
        let (parser, results) = self.inner.parser_with_steps(right_data, bytes);
        cache_data_inner.borrow_mut().entries.reverse();
        let mut cache_context_parser = CacheContextParser { inner: Box::new(parser), cache_data_inner };
        cache_context_parser.cleanup();
        (Parser::CacheContextParser(cache_context_parser), results)
    }
}

impl ParserTrait for CacheContextParser {
    fn step(&mut self, c: u8) -> ParseResults {
        let mut all_up_data = Vec::new();
        self.cache_data_inner.borrow_mut().entries.iter_mut().for_each(|entry| {
            entry.borrow_mut().maybe_parse_results.take();
        });
        let num_entries_initial = self.cache_data_inner.borrow().entries.len().clone();
        for i in (0..num_entries_initial).rev() {
            let entry = self.cache_data_inner.borrow().entries[i].clone();
            let parse_results = catch_unwind(AssertUnwindSafe(|| entry.borrow_mut().parser.as_mut().unwrap().step(c))).expect("CacheContextParser.step: panic");
            let mut parse_results = parse_results.clone();
            parse_results.squash();
            entry.borrow_mut().maybe_parse_results = Some(parse_results);
            if !parse_results.done {
                all_up_data.extend(parse_results.up_data_vec);
            }
        }
        let parse_result = self.inner.step(c);
        let mut new_entries = self.cache_data_inner.borrow_mut().entries.split_off(num_entries_initial);        new_entries.reverse();
        self.cache_data_inner.borrow_mut().entries.append(&mut new_entries);
        self.cleanup();
        parse_result
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        let mut all_up_data = Vec::new();
        self.cache_data_inner.borrow_mut().entries.iter_mut().for_each(|entry| {
            entry.borrow_mut().maybe_parse_results.take();
        });
        let num_entries_initial = self.cache_data_inner.borrow().entries.len().clone();
        for i in (0..num_entries_initial).rev() {
            let entry = self.cache_data_inner.borrow().entries[i].clone();
            let parse_results = catch_unwind(AssertUnwindSafe(|| entry.borrow_mut().parser.as_mut().unwrap().steps(bytes))).expect("CacheContextParser.steps: panic");
            let mut parse_results = parse_results.clone();
            parse_results.squash();
            entry.borrow_mut().maybe_parse_results = Some(parse_results);
            if !parse_results.done {
                all_up_data.extend(parse_results.up_data_vec);
            }
        }
        let parse_result = self.inner.steps(bytes);
        let mut new_entries = self.cache_data_inner.borrow_mut().entries.split_off(num_entries_initial);        new_entries.reverse();
        self.cache_data_inner.borrow_mut().entries.append(&mut new_entries);
        self.cleanup();
        parse_result
    }
}

impl CombinatorTrait for Cached {
    fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
        let key = CacheKey { combinator: self.inner.clone(), right_data: right_data.clone() };
        if let Some(entry) = right_data.cache_data.inner.as_ref().unwrap().borrow().new_parsers.get(&key).cloned() {
            let parse_results = entry.borrow().maybe_parse_results.clone().expect("CachedParser.parser: parse_results is None").squashed();
            return (Parser::CachedParser(CachedParser { entry }), parse_results);
        }
        let entry = Rc::new(RefCell::new(CacheEntry {            parser: None,
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

    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let key = CacheKey { combinator: self.inner.clone(), right_data: right_data.clone() };
        if let Some(entry) = right_data.cache_data.inner.as_ref().unwrap().borrow().new_parsers.get(&key).cloned() {
            let parse_results = entry.borrow().maybe_parse_results.clone().expect("CachedParser.parser: parse_results is None").squashed();
            return (Parser::CachedParser(CachedParser { entry }), parse_results);
        }
        let entry = Rc::new(RefCell::new(CacheEntry {
            parser: None,
            maybe_parse_results: None,
        }));
        let (parser, mut parse_results) = self.inner.parser_with_steps(right_data.clone(), bytes);
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
    fn step(&mut self, c: u8) -> ParseResults {
        self.entry.borrow().maybe_parse_results.clone().expect("CachedParser.step: parse_results is None").squashed()
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        self.entry.borrow().maybe_parse_results.clone().expect("CachedParser.steps: parse_results is None").squashed()
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
