use crate::RightData;
// src/combinators/cache.rs
// src/combinators/cache.rs
use crate::{BaseCombinatorTrait, DynCombinatorTrait, UnambiguousParseResults};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::intrinsics::transmute;
use std::num::NonZeroUsize;
use std::rc::Rc;

use crate::{count_hit, profile, profile_internal, CombinatorTrait, IntoCombinator, ParseResultTrait, ParseResults, ParserTrait, Squash, U8Set};
use derivative::Derivative;
use lru::LruCache;


// macro_rules! profile {
//     ($tag:expr, $body:expr) => {{
//         $body
//     }};
// }

// macro_rules! count_hit { ($tag:expr) => {} }

thread_local! {
    pub static GLOBAL_CACHE: RefCell<GlobalCache> = RefCell::new(GlobalCache::new());
}

#[derive(Debug)]
struct GlobalCache {
    new_parsers: HashMap<usize, LruCache<CacheKey, Rc<RefCell<CacheEntry>>>>,
    pub(crate) entries: HashMap<usize, Vec<Rc<RefCell<CacheEntry>>>>,
    pub one_shot_results: HashMap<usize, LruCache<CacheKey, UnambiguousParseResults>>,
    pub(crate) parse_id_counter: usize,
    pub(crate) parse_id: Option<usize>,
}

impl GlobalCache {
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
struct CacheEntry {
    pub(crate) parser: Option<Box<dyn ParserTrait>>,
    maybe_parse_results: Option<ParseResults>,
    maybe_u8set: Option<U8Set>,
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
pub struct CachedParser {
    pub entry: Rc<RefCell<CacheEntry>>,
}

impl Hash for CachedParser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.entry.as_ref() as *const RefCell<CacheEntry>, state);
    }
}

impl PartialEq for CachedParser {
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
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<T: CombinatorTrait> CombinatorTrait for CacheContext<T> {
    type Parser<'a> = CacheContextParser<'a> where Self: 'a;
    type Output = ();

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        profile!("CacheContext.one_shot_parse: start", {
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
            })
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        profile!("CacheContext.old_parse: start", {
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
            let (parser, results) = profile!("CacheContext.parse: inner.parse", self.inner.parse(right_data, bytes));
            let mut global_cache = cache.borrow_mut();
            global_cache.entries.get_mut(&parse_id).unwrap().reverse();
            global_cache.cleanup();
            let cache_context_parser = CacheContextParser { inner: Box::new(parser), parse_id };
            (cache_context_parser, results)
        })
        })
    }
}

impl<T: CombinatorTrait> BaseCombinatorTrait for CacheContext<T> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
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
        profile!("CacheContextParser.parse: start", {
        GLOBAL_CACHE.with(|cache| {
            {
                let mut global_cache = cache.borrow_mut();
                global_cache.parse_id = Some(self.parse_id);
                global_cache.entries.get_mut(&self.parse_id).unwrap().iter_mut().for_each(|entry| {
                    entry.borrow_mut().maybe_parse_results.take();
                    entry.borrow_mut().maybe_u8set.take();
                });
            }
            let num_entries_initial = cache.borrow_mut().entries[&self.parse_id].len();
            for i in (0..num_entries_initial).rev() {
                let entry_refcell = cache.borrow_mut().entries[&self.parse_id][i].clone();
                let mut entry = entry_refcell.borrow_mut();
                let parse_results = profile!("CacheContextParser.parse: entry.parser.parse", entry.parser.as_mut().unwrap().parse(bytes));
                entry.maybe_parse_results = Some(parse_results);
            }

            let parse_result = profile!("CacheContextParser.parse: inner.parse", self.inner.as_mut().parse(bytes));

            let mut global_cache = cache.borrow_mut();
            let mut new_entries = global_cache.entries.get_mut(&self.parse_id).unwrap().split_off(num_entries_initial);
            new_entries.reverse();
            global_cache.entries.get_mut(&self.parse_id).unwrap().append(&mut new_entries);
            global_cache.cleanup();
            parse_result
        })
        })
    }
}

impl<T: CombinatorTrait> DynCombinatorTrait for Cached<T> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<T: CombinatorTrait> CombinatorTrait for Cached<T> {
    type Parser<'a> = CachedParser where Self: 'a;
    type Output = ();

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
                maybe_u8set: None,
            }));
            // let inner: &'static T = unsafe { transmute(&self.inner) };
            // let (parser, mut parse_results): (_, ParseResults) = profile!("Cached.parse: inner.parse", inner.parse(right_data, bytes));
            // let (parser, mut parse_results) = inner.parse_dyn(right_data, bytes);
            let (parser, mut parse_results) = self.inner.parse_dyn(right_data, bytes);
            let parser: Box<dyn ParserTrait + 'static> = unsafe { transmute(parser) };
            // profile!("Cached.parse: parse_results.squash", parse_results.squash());

            let mut global_cache = cache.borrow_mut();
            let parse_id = global_cache.parse_id.unwrap();
            global_cache.new_parsers.get_mut(&parse_id).unwrap().put(key, entry.clone());
            if !parse_results.done() {
                global_cache.entries.get_mut(&parse_id).unwrap().push(entry.clone());
            }
            *entry.borrow_mut() = CacheEntry { parser: Some(parser), maybe_parse_results: Some(parse_results.clone()), maybe_u8set: None };
            (CachedParser { entry }, parse_results)
        })
    }
}

impl<T: CombinatorTrait> BaseCombinatorTrait for Cached<T> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        f(&self.inner);
    }
}

impl ParserTrait for CachedParser {
    fn get_u8set(&self) -> U8Set {
        if self.entry.borrow().maybe_u8set.is_some() {
            self.entry.borrow().maybe_u8set.clone().unwrap()
        } else {
            let u8set = self.entry.borrow().parser.as_ref().unwrap().get_u8set();
            self.entry.borrow_mut().maybe_u8set = Some(u8set.clone());
            u8set
        }
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.entry.borrow().maybe_parse_results.clone().expect("CachedParser.steps: parse_results is None")
    }
}

pub fn cache_context<'a, T: IntoCombinator>(a: T)-> impl CombinatorTrait where T::Output: 'static {
    profile_internal("cache_context", CacheContext { inner: a.into_combinator() })
    // a.into_combinator()
}

// todo: do we really need to make this 'static?
pub fn cached<T: IntoCombinator>(a: T)-> impl CombinatorTrait where T::Output: 'static {
    profile_internal("cached", Cached { inner: a.into_combinator() })
    // a.into_combinator()
}
