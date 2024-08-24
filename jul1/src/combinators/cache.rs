// src/combinators/cache.rs
// src/combinators/cache.rs
use crate::{BaseCombinatorTrait, DynCombinatorTrait, UnambiguousParseResults};
use crate::RightData;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::intrinsics::transmute;
use std::num::NonZeroUsize;
use std::rc::Rc;

use derivative::Derivative;
use lru::LruCache;
use crate::{CombinatorTrait, ParseResults, ParserTrait, profile, profile_internal, ParseResultTrait, Squash, U8Set, IntoCombinator, IntoDyn, count_hit};


macro_rules! profile {
    ($tag:expr, $body:expr) => {{
        $body
    }};
}

// macro_rules! count_hit { ($tag:expr) => {} }

thread_local! {
    pub static GLOBAL_CACHE: RefCell<GlobalCache<'static>> = RefCell::new(GlobalCache::new());
}

#[derive(Debug)]
struct GlobalCache<'a> {
    new_parsers: HashMap<usize, LruCache<CacheKey, Rc<RefCell<CacheEntry<'a>>>>>,
    pub(crate) entries: HashMap<usize, Vec<Rc<RefCell<CacheEntry<'a>>>>>,
    pub one_shot_results: HashMap<usize, LruCache<CacheKey, UnambiguousParseResults>>,
    pub(crate) parse_id_counter: usize,
    pub(crate) parse_id: Option<usize>,
}

impl GlobalCache<'_> {
    fn new() -> Self {
        Self {
            new_parsers: HashMap::new(),
            entries: HashMap::new(),
            one_shot_results: HashMap::new(),
            parse_id_counter: 0,
            parse_id: None,
        }
    }

    fn cleanup(&mut self) {
        let parse_id = self.parse_id.take().unwrap();
        self.new_parsers.get_mut(&parse_id).unwrap().clear();
        self.entries.get_mut(&parse_id).unwrap().retain(|entry| !entry.borrow().maybe_parse_results.as_ref().unwrap().done());
    }

    fn one_shot_cleanup(&mut self) {
        let parse_id = self.parse_id.take().unwrap();
        self.one_shot_results.get_mut(&parse_id).unwrap().clear();
    }
}

#[derive(Debug)]
struct CacheKey {
    combinator: *const dyn DynCombinatorTrait,
    right_data: RightData,
}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.combinator.hash(state);
        self.right_data.hash(state);
    }
}

impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.combinator == other.combinator && self.right_data == other.right_data
    }
}

impl Eq for CacheKey {}

#[derive(Derivative)]
#[derivative(Debug)]
struct CacheEntry<'a> {
    pub(crate) parser: Option<Box<dyn ParserTrait + 'a>>,
    maybe_parse_results: Option<ParseResults>,
}

#[derive(Debug)]
pub struct CacheContext<T: CombinatorTrait> {
    pub inner: T,
}

#[derive(Debug)]
pub struct Cached<T: CombinatorTrait> {
    pub inner: T,
}

#[derive(Debug)]
pub struct CachedParser<'a> {
    pub entry: Rc<RefCell<CacheEntry<'a>>>,
}

impl Hash for CachedParser<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.entry.as_ref() as *const RefCell<CacheEntry<'_>>, state);
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
    pub inner: Box<dyn ParserTrait + 'a>,
    pub(crate) parse_id: usize,
}

impl<T: CombinatorTrait> DynCombinatorTrait for CacheContext<T> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait>, ParseResults) {
        todo!()
    }
}

impl<T: CombinatorTrait> CombinatorTrait for CacheContext<T> {
    type Parser<'a> = CacheContextParser<'a> where Self: 'a;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        GLOBAL_CACHE.with(|cache| {
            let parse_id = {
                let mut global_cache = cache.borrow_mut();
                let parse_id = global_cache.parse_id_counter;
                global_cache.new_parsers.insert(parse_id, LruCache::new(NonZeroUsize::new(64).unwrap()));
                global_cache.entries.insert(parse_id, Vec::new());
                global_cache.one_shot_results.insert(parse_id, LruCache::new(NonZeroUsize::new(64).unwrap()));
                global_cache.parse_id = Some(global_cache.parse_id_counter);
                global_cache.parse_id_counter += 1;
                parse_id
            };
            let parse_result = self.inner.one_shot_parse(right_data, bytes);
            let mut global_cache = cache.borrow_mut();
            global_cache.one_shot_cleanup();
            parse_result
        })
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        GLOBAL_CACHE.with(|cache| {
            let parse_id = {
                let mut global_cache = cache.borrow_mut();
                let parse_id = global_cache.parse_id_counter;
                global_cache.new_parsers.insert(parse_id, LruCache::new(NonZeroUsize::new(64).unwrap()));
                global_cache.entries.insert(parse_id, Vec::new());
                global_cache.one_shot_results.insert(parse_id, LruCache::new(NonZeroUsize::new(64).unwrap()));
                global_cache.parse_id = Some(global_cache.parse_id_counter);
                global_cache.parse_id_counter += 1;
                parse_id
            };
            let (parser, results) = self.inner.parse(right_data, bytes);
            let mut global_cache = cache.borrow_mut();
            global_cache.entries.get_mut(&parse_id).unwrap().reverse();
            global_cache.cleanup();
            let cache_context_parser = CacheContextParser { inner: Box::new(parser), parse_id };
            (cache_context_parser, results)
        })
    }
}

impl<T: CombinatorTrait> BaseCombinatorTrait for CacheContext<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        f(&self.inner);
    }
}

impl ParserTrait for CacheContextParser<'_> {
    fn get_u8set(&self) -> U8Set {
        self.inner.as_ref().get_u8set()
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

            let parse_result = self.inner.as_mut().parse(bytes);

            let mut global_cache = cache.borrow_mut();
            let mut new_entries = global_cache.entries.get_mut(&self.parse_id).unwrap().split_off(num_entries_initial);
            new_entries.reverse();
            global_cache.entries.get_mut(&self.parse_id).unwrap().append(&mut new_entries);
            global_cache.cleanup();
            parse_result
        })
    }
}

impl<T: CombinatorTrait> DynCombinatorTrait for Cached<T> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait>, ParseResults) {
        todo!()
    }
}

impl<T: CombinatorTrait> CombinatorTrait for Cached<T> {
    type Parser<'a> = CachedParser<'a> where Self: 'a;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        GLOBAL_CACHE.with(move |cache| {
            let key = CacheKey { combinator: std::ptr::addr_of!(self.inner) as *const dyn DynCombinatorTrait, right_data: right_data.clone() };

            let mut global_cache = cache.borrow_mut();
            let parse_id = global_cache.parse_id.unwrap();
            if let Some(entry) = profile!("Cached.parse: check cache: get entry", {
                global_cache.one_shot_results.get_mut(&parse_id).unwrap().get(&key).cloned()
            }) {
                count_hit!("Cached.parse: cache hit");
                return entry;
            }
            drop(global_cache);

            count_hit!("Cached.parse: cache miss");
            let inner = &self.inner;
            let parse_result: UnambiguousParseResults = profile!("Cached.parse: inner.one_shot_parse", inner.one_shot_parse(right_data, bytes));
            let mut global_cache = cache.borrow_mut();
            let parse_id = global_cache.parse_id.unwrap();
            global_cache.one_shot_results.get_mut(&parse_id).unwrap().put(key, parse_result.clone());
            parse_result
        })
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        GLOBAL_CACHE.with(move |cache| {
            let key = CacheKey { combinator: std::ptr::addr_of!(self.inner) as *const dyn DynCombinatorTrait, right_data: right_data.clone() };

            let mut global_cache = cache.borrow_mut();
            let parse_id = global_cache.parse_id.unwrap();
            if let Some(entry) = profile!("Cached.parse: check cache: get entry", {
                global_cache.new_parsers.get_mut(&parse_id).unwrap().get(&key).cloned()
            }) {
                count_hit!("Cached.parse: cache hit");
                let parse_results = entry.borrow().maybe_parse_results.clone().expect("CachedParser.parser: parse_results is None");
                return (CachedParser { entry }, parse_results);
            }
            drop(global_cache);

            count_hit!("Cached.parse: cache miss");
            let entry = Rc::new(RefCell::new(CacheEntry {
                parser: None,
                maybe_parse_results: None,
            }));
            let inner: &'static T = unsafe { transmute(&self.inner) };
            let (parser, mut parse_results): (_, ParseResults) = profile!("Cached.parse: inner.parse", inner.parse(right_data, bytes));
            let parser: Box<dyn ParserTrait> = Box::new(parser);
            profile!("Cached.parse: parse_results.squash", parse_results.squash());

            let mut global_cache = cache.borrow_mut();
            let parse_id = global_cache.parse_id.unwrap();
            global_cache.new_parsers.get_mut(&parse_id).unwrap().put(key, entry.clone());
            if !parse_results.done() {
                global_cache.entries.get_mut(&parse_id).unwrap().push(entry.clone());
            }
            *entry.borrow_mut() = CacheEntry { parser: Some(parser), maybe_parse_results: Some(parse_results.clone()) };
            (CachedParser { entry }, parse_results)
        })
    }
}

impl<T: CombinatorTrait> BaseCombinatorTrait for Cached<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        f(&self.inner);
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

pub fn cache_context<'a, T: IntoCombinator>(a: T)-> impl CombinatorTrait {
    profile_internal("cache_context", CacheContext { inner: a.into_combinator() })
    // a.into_combinator()
}

// todo: do we really need to make this 'static?
pub fn cached<T: IntoCombinator>(a: T)-> impl CombinatorTrait where T::Output: 'static {
    profile_internal("cached", Cached { inner: a.into_combinator() })
    // a.into_combinator()
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
//