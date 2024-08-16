use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::num::NonZeroUsize;
use std::rc::Rc;

use derivative::Derivative;
use lru::LruCache;
use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, profile, profile_internal, RightData, Squash, U8Set, IntoCombinator, IntoDyn};


macro_rules! profile {
    ($tag:expr, $body:expr) => {{
        $body
    }};
}

thread_local! {
    pub static GLOBAL_CACHE: RefCell<GlobalCache<'static>> = RefCell::new(GlobalCache::new());
}

#[derive(Debug)]
struct GlobalCache<'a> {
    new_parsers: HashMap<usize, HashMap<CacheKey, Rc<RefCell<CacheEntry<'a>>>>>,
    pub(crate) entries: HashMap<usize, Vec<Rc<RefCell<CacheEntry<'a>>>>>,
    pub(crate) parse_id_counter: usize,
    pub(crate) parse_id: Option<usize>,
}

impl GlobalCache<'_> {
    fn new() -> Self {
        Self {
            new_parsers: HashMap::new(),
            entries: HashMap::new(),
            parse_id_counter: 0,
            parse_id: None,
        }
    }

    fn cleanup(&mut self) {
        let parse_id = self.parse_id.take().unwrap();
        self.new_parsers.get_mut(&parse_id).unwrap().clear();
        self.entries.get_mut(&parse_id).unwrap().retain(|entry| !entry.borrow().maybe_parse_results.as_ref().unwrap().done());
    }
}

#[derive(Debug)]
struct CacheKey {
    combinator: Rc<dyn CombinatorTrait>,
    right_data: RightData,
}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // use type_id::TypeId;
        self.right_data.hash(state);
    }
}

impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.combinator, &other.combinator) && self.right_data == other.right_data
    }
}

impl Eq for CacheKey {}

#[derive(Derivative)]
#[derivative(Debug)]
struct CacheEntry<'a> {
    pub(crate) parser: Option<Box<Parser<'a>>>,
    maybe_parse_results: Option<ParseResults>,
}

#[derive(Debug)]
pub struct CacheContext<T: CombinatorTrait> {
    pub inner: T,
}

#[derive(Debug)]
pub struct Cached<T: CombinatorTrait> {
    pub inner: Rc<T>,
}

#[derive(Debug)]
pub struct CachedParser<'a> {
    pub entry: Rc<RefCell<CacheEntry<'a>>>,
}

impl Hash for CachedParser<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.entry.as_ref() as *const RefCell<CacheEntry>, state);
    }
}

impl PartialEq for CachedParser<'_> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.entry, &other.entry)
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct CacheContextParser<'a> {
    pub inner: Box<Parser<'a>>,
    pub(crate) parse_id: usize,
}

impl<T: CombinatorTrait> CombinatorTrait for CacheContext<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }
    fn parse<'a, 'b>(&'b self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'a>, ParseResults) where Self: 'a, 'a: 'b {
        GLOBAL_CACHE.with(|cache| {
            let parse_id = {
                let mut global_cache = cache.borrow_mut();
                let parse_id = global_cache.parse_id_counter;
                global_cache.new_parsers.insert(parse_id, HashMap::new());
                global_cache.entries.insert(parse_id, Vec::new());
                global_cache.parse_id = Some(global_cache.parse_id_counter);
                global_cache.parse_id_counter += 1;
                parse_id
            };
            let (parser, results) = self.inner.parse(right_data, bytes);
            let mut global_cache = cache.borrow_mut();
            global_cache.entries.get_mut(&parse_id).unwrap().reverse();
            global_cache.cleanup();
            let cache_context_parser = CacheContextParser { inner: Box::new(parser), parse_id };
            (Parser::CacheContextParser(cache_context_parser), results)
        })
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(&self.inner);
    }
}

impl ParserTrait for CacheContextParser<'_> {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        GLOBAL_CACHE.with(|cache| {
            {
                let mut global_cache = cache.borrow_mut();
                global_cache.parse_id = Some(self.parse_id);
                global_cache.entries.get_mut(&self.parse_id).unwrap().iter_mut().for_each(|entry| {
                    entry.borrow_mut().maybe_parse_results.take();
                });
            }
            let num_entries_initial = cache.borrow_mut().entries[&self.parse_id].len();
            for i in (0..num_entries_initial).rev() {
                let entry_refcell = cache.borrow_mut().entries[&self.parse_id][i].clone();
                let mut entry = entry_refcell.borrow_mut();
                let parse_results = entry.parser.as_mut().unwrap().parse(bytes);
                entry.maybe_parse_results = Some(parse_results);
            }

            let parse_result = self.inner.parse(bytes);

            let mut global_cache = cache.borrow_mut();
            let mut new_entries = global_cache.entries.get_mut(&self.parse_id).unwrap().split_off(num_entries_initial);
            new_entries.reverse();
            global_cache.entries.get_mut(&self.parse_id).unwrap().append(&mut new_entries);
            global_cache.cleanup();
            parse_result
        })
    }
}

impl<T: CombinatorTrait> CombinatorTrait for Cached<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }
    fn parse<'a, 'b>(&'b self, right_data: RightData<>, bytes: &[u8]) -> (Parser<'a>, ParseResults) where Self: 'a, 'a: 'b {
        GLOBAL_CACHE.with(move |cache| {
            // let key = CacheKey { combinator: self.inner.clone(), right_data: right_data.clone() };
            //
            // let mut global_cache = cache.borrow_mut();
            // let parse_id = global_cache.parse_id.unwrap();
            // if let Some(entry) = profile!("Cached.parse: check cache: get entry", {
            //     global_cache.new_parsers.get_mut(&parse_id).unwrap().get(&key).cloned()
            // }) {
            //     profile!("Cached.parse: cache hit", {});
            //     let parse_results = entry.borrow().maybe_parse_results.clone().expect("CachedParser.parser: parse_results is None");
            //     return (Parser::CachedParser(CachedParser { entry }), parse_results);
            // }
            // drop(global_cache);
            //
            // profile!("Cached.parse: cache miss", {});
            // let entry = Rc::new(RefCell::new(CacheEntry {
            //     parser: None,
            //     maybe_parse_results: None,
            // }));
            // let inner = self.inner.clone();
            // let (parser, mut parse_results) = profile!("Cached.parse: inner.parse", inner.parse(right_data, bytes));
            // profile!("Cached.parse: parse_results.squash", parse_results.squash());
            //
            // let mut global_cache = cache.borrow_mut();
            // let parse_id = global_cache.parse_id.unwrap();
            // global_cache.new_parsers.get_mut(&parse_id).unwrap().insert(key, entry.clone());
            // if !parse_results.done() {
            //     global_cache.entries.get_mut(&parse_id).unwrap().push(entry.clone());
            // }
            // *entry.borrow_mut() = CacheEntry { parser: Some(Box::new(parser)), maybe_parse_results: Some(parse_results.clone()) };
            // (Parser::CachedParser(CachedParser { entry }), parse_results)
            todo!("fix lifetimes")
        })
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(self.inner.as_ref());
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

pub fn cache_context<'a, T: IntoCombinator>(a: T)-> impl CombinatorTrait where T::Output: IntoDyn<'a> {
    // profile_internal("cache_context", CacheContext { inner: a.into_combinator().into_dyn() })
    a.into_combinator()
}

pub fn cached<'a, T: IntoCombinator>(a: T)-> impl CombinatorTrait where T::Output: IntoDyn<'a> {
    // profile_internal("cached", Cached { inner: Rc::new(a.into_combinator().into_dyn()) })
    a.into_combinator()
}

// impl From<CacheContext> for Combinator {
//     fn from(value: CacheContext) -> Self {
//         Combinator::CacheContext(value)
//     }
// }
//
// impl From<Cached> for Combinator {
//     fn from(value: Cached) -> Self {
//         Combinator::Cached(value)
//     }
// }
