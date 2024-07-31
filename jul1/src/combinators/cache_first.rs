use std::cell::RefCell;
use std::collections::btree_map::Keys;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::rc::Rc;

use derivative::Derivative;

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, RightData, Squash};

#[derive(Clone, PartialEq, Default, Eq)]
pub struct CacheFirstData {
    pub inner: Option<Rc<RefCell<CacheFirstDataInner>>>,
}

#[derive(Debug, Clone, PartialEq, Default, Eq)]
pub struct CacheFirstDataInner {
    pub entries: HashMap<CacheFirstKey, ParseResults>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheFirstKey {
    pub combinator: Rc<Combinator>,
    pub right_data: RightData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheFirstContext {
    pub inner: Box<Combinator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheFirst {
    pub inner: Rc<Combinator>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheFirstParser {
    Uninitialized { key: CacheFirstKey },
    Initialized { parser: Box<Parser> },
}

impl Hash for CacheFirstParser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // self.entry.borrow().hash(state);
    }
}

impl Hash for CacheFirstKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self.combinator.as_ref()).hash(state);
        self.right_data.hash(state);
    }
}

#[derive(Derivative)]
#[derivative(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheFirstContextParser {
    pub inner: Box<Parser>,
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    pub cache_first_data_inner: Rc<RefCell<CacheFirstDataInner>>,
}

impl CombinatorTrait for CacheFirstContext {
    fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
        assert!(right_data.cache_first_data.inner.is_none(), "CacheFirstContextParser already initialized");
        right_data.cache_first_data.inner = Some(Rc::new(RefCell::new(CacheFirstDataInner::default())));
        let (parser, results) = self.inner.parser(right_data.clone());
        (Parser::CacheFirstContextParser(CacheFirstContextParser {
            inner: Box::new(parser),
            cache_first_data_inner: right_data.cache_first_data.inner.clone().unwrap(),
        }), results)
    }
}

impl ParserTrait for CacheFirstContextParser {
    fn step(&mut self, c: u8) -> ParseResults {
        self.inner.step(c)
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        self.inner.steps(bytes)
    }
}

impl CombinatorTrait for CacheFirst {
    fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
        // Try to get the entry from the cache
        let key = CacheFirstKey { combinator: self.inner.clone(), right_data: right_data.clone() };
        if let Some(entry) = right_data.cache_first_data.inner.clone().unwrap().borrow().entries.get(&key).cloned() {
            return (Parser::CacheFirstParser(CacheFirstParser::Uninitialized { key }), entry);
        }
        // Initialize the parser and create a new entry
        let (parser, mut parse_results) = self.inner.parser(right_data.clone());
        parse_results.squash();
        let binding = right_data.cache_first_data.inner.unwrap();
        let mut cache_first_data_inner = binding.borrow_mut();
        cache_first_data_inner.entries.insert(key.clone(), parse_results.clone());
        (parser, parse_results)
    }
}

impl ParserTrait for CacheFirstParser {
    fn step(&mut self, c: u8) -> ParseResults {
        match self {
            CacheFirstParser::Uninitialized { key } => {
                // Initialize the parser and step it.
                let (parser, parse_results) = key.combinator.parser(key.right_data.clone());
                *self = CacheFirstParser::Initialized { parser: Box::new(parser) };
                self.step(c)
            }
            CacheFirstParser::Initialized { parser } => {
                // Step the parser.
                parser.step(c)
            }
        }
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        match self {
            CacheFirstParser::Uninitialized { key } => {
                // Initialize the parser and step it.
                let (mut parser, parse_results) = key.combinator.parser(key.right_data.clone());
                *self = CacheFirstParser::Initialized { parser: Box::new(parser) };
                self.steps(bytes)
            }
            CacheFirstParser::Initialized { parser } => {
                // Step the parser.
                parser.steps(bytes)
            }
        }
    }
}

// pub fn cache_first_context(a: impl Into<Combinator>) -> CacheFirstContext {
//     CacheFirstContext { inner: Box::new(a.into()) }
// }
//
// pub fn cache_first(a: impl Into<Combinator>) -> CacheFirst {
//     CacheFirst { inner: Rc::new(a.into()) }
// }

pub fn cache_first_context<A: Into<Combinator>>(a: A) -> Combinator {
    a.into()
}

pub fn cache_first<A: Into<Combinator>>(a: A) -> Combinator {
    a.into()
}

impl From<CacheFirstContext> for Combinator {
    fn from(value: CacheFirstContext) -> Self {
        Combinator::CacheFirstContext(value)
    }
}

impl From<CacheFirst> for Combinator {
    fn from(value: CacheFirst) -> Self {
        Combinator::CacheFirst(value)
    }
}
