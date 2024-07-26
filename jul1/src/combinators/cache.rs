use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;
use crate::{CombinatorTrait, DynCombinator, IntoCombinator, ParseResults, ParserTrait, RightData, Squash, Stats};

#[derive(Debug, Clone, PartialEq, Default, PartialOrd, Ord, Eq)]
pub struct CacheData {
    pub inner: Option<Rc<RefCell<CacheDataInner>>>,
}

impl Hash for CacheData {
    fn hash<H: Hasher>(&self, state: &mut H) {}
}

pub struct CacheEntry {
    parser: Option<Box<dyn ParserTrait>>,
    maybe_parse_results: Option<ParseResults>,
    num: usize,
}

pub struct CacheKey {
    combinator: Rc<DynCombinator>,
    right_data: RightData,
}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.combinator.dyn_hash(state);
        self.right_data.hash(state)
    }
}

impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        // self.combinator.dyn_eq(&other.combinator) && self.right_data == other.right_data
        Rc::ptr_eq(&self.combinator, &other.combinator) && self.right_data == other.right_data
    }
}

impl Eq for CacheKey {}

#[derive(Default)]
pub struct CacheDataInner {
    pub new_parsers: HashMap<CacheKey, Rc<RefCell<CacheEntry>>>,
    pub entries: Vec<Rc<RefCell<CacheEntry>>>,
}

impl Debug for CacheDataInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CacheDataInner")
    }
}

impl PartialEq for CacheDataInner {
    fn eq(&self, other: &Self) -> bool {
        false
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
        self.new_parsers.len().cmp(&other.new_parsers.len())
    }
}

impl Hash for CacheDataInner {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.new_parsers.len().hash(state);
        self.entries.len().hash(state);
    }
}

pub struct CacheContext<T> {
    pub inner: T,
}

pub struct CacheContextParser<P> {
    pub inner: P,
    pub cache_data_inner: Rc<RefCell<CacheDataInner>>,
}

impl<T> CacheContextParser<T> {
    fn cleanup(&mut self) {
        self.cache_data_inner.borrow_mut().new_parsers.clear();
        self.cache_data_inner.borrow_mut().entries.retain(|entry| !entry.borrow().maybe_parse_results.as_ref().unwrap().done);
    }
}

impl<T> CombinatorTrait for CacheContext<T>
where
    T: CombinatorTrait,
{
    type Parser = CacheContextParser<T::Parser>;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        assert!(right_data.cache_data.inner.is_none(), "CacheContextParser already initialized");
        let cache_data_inner = Rc::new(RefCell::new(CacheDataInner::default()));
        right_data.cache_data.inner = Some(cache_data_inner.clone());
        let (parser, results) = self.inner.parser(right_data);
        // Reverse the order of entries
        cache_data_inner.borrow_mut().entries.reverse();
        let mut cache_context_parser = CacheContextParser { inner: parser, cache_data_inner };
        cache_context_parser.cleanup();
        (cache_context_parser, results)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<P> ParserTrait for CacheContextParser<P>
where
    P: ParserTrait + 'static,
{
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

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a dyn ParserTrait> + 'a> {
        todo!()
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut dyn ParserTrait> + 'a> {
        todo!()
    }

    fn collect_stats(&self, stats: &mut Stats) {
        self.inner.collect_stats(stats);
        for entry in self.cache_data_inner.borrow().entries.iter() {
            entry.borrow().parser.as_ref().unwrap().collect_stats(stats);
        }
    }

    fn dyn_eq(&self, other: &dyn ParserTrait) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self.inner.dyn_eq(&other.inner)
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct Cached {
    pub(crate) inner: Rc<DynCombinator>,
}

pub struct CachedParser {
    pub entry: Rc<RefCell<CacheEntry>>,
    pub num: usize,
}

impl CombinatorTrait for Cached {
    type Parser = CachedParser;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        for (key, entry) in right_data.cache_data.inner.as_ref().unwrap().borrow().new_parsers.iter() {
            if Rc::ptr_eq(&key.combinator, &self.inner) && key.right_data == right_data {
                let parse_results = entry.borrow().maybe_parse_results.clone().expect("CachedParser.parser: parse_results is None");
                return (
                    CachedParser {
                        entry: entry.clone(),
                        num: entry.borrow().num,
                    },
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
        entry.borrow_mut().parser = Some(parser);
        entry.borrow_mut().maybe_parse_results = Some(parse_results.clone());
        let num = entry.borrow().num;
        (CachedParser { entry, num }, parse_results)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ParserTrait for CachedParser {
    fn step(&mut self, c: u8) -> ParseResults {
        self.entry.borrow().maybe_parse_results.clone().expect(format!("CachedParser.step: parse_results is None for entry number {} (self.num = {})", self.entry.borrow().num, self.num).as_str())
    }

    fn dyn_eq(&self, other: &dyn ParserTrait) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            Rc::ptr_eq(&self.entry, &other.entry)
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl IntoCombinator for &Cached {
    type Output = Cached;
    fn into_combinator(self) -> Self::Output {
        self.clone()
    }
}

pub fn cached<T>(t: T) -> Cached
where
    T: IntoCombinator,
{
    Cached { inner: t.into_combinator().into_rc_dyn() }
}

pub fn cache_context<T>(t: T) -> CacheContext<T::Output>
where
    T: IntoCombinator,
{
    CacheContext { inner: t.into_combinator() }
}