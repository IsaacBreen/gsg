use std::any::Any;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{CombinatorTrait, DynCombinator, IntoCombinator, ParseResults, ParserTrait, RightData, Stats};

#[derive(Debug, Clone, PartialEq, Default, PartialOrd, Ord, Eq)]
pub struct CacheData {
    pub inner: Option<Rc<RefCell<CacheDataInner>>>,
}

impl Hash for CacheData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

pub struct ComparableRc<T: ?Sized>(pub Rc<T>);

impl From<Rc<DynCombinator>> for ComparableRc<DynCombinator> {
    fn from(rc: Rc<DynCombinator>) -> Self {
        ComparableRc(rc)
    }
}

impl<T: ?Sized> PartialEq for ComparableRc<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: ?Sized> Eq for ComparableRc<T> {}

impl<T: ?Sized> PartialOrd for ComparableRc<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: ?Sized> Ord for ComparableRc<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Rc::as_ptr(&self.0).cmp(&Rc::as_ptr(&other.0))
    }
}

impl<T: ?Sized> Hash for ComparableRc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state);
    }
}

#[derive(Default)]
pub struct CacheDataInner {
    pub new_parsers_i: HashMap<ComparableRc<DynCombinator>, usize>,
    pub new_parsers: Vec<(Box<dyn ParserTrait>, Rc<RefCell<ParseResults>>)>,
    pub existing_parsers: Vec<(Box<dyn ParserTrait>, Rc<RefCell<ParseResults>>)>,
}

impl Debug for CacheDataInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// #[derive(Clone, PartialEq, Default, Hash, PartialOrd, Ord, Eq)]
impl PartialEq for CacheDataInner {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
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
        let self_ptr = std::ptr::addr_of!(self) as *const ();
        let other_ptr = std::ptr::addr_of!(other) as *const ();
        self_ptr.cmp(&other_ptr)
    }
}

pub struct CacheContext<T> {
    pub inner: T,
}

pub struct CacheContextParser<P> {
    pub inner: P,
    pub cache_data_inner: Rc<RefCell<CacheDataInner>>,
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
        (CacheContextParser { inner: parser, cache_data_inner }, results)
    }
}

impl<P> ParserTrait for CacheContextParser<P>
where
    P: ParserTrait + 'static,
{
    fn step(&mut self, c: u8) -> ParseResults {
        {
            // Move new parsers to existing parsers
            let mut cache_data_inner = self.cache_data_inner.borrow_mut();
            let new_parsers_i = std::mem::take(&mut cache_data_inner.new_parsers_i);
            let new_parsers = std::mem::take(&mut cache_data_inner.new_parsers);
            self.cache_data_inner.borrow_mut().existing_parsers.extend(new_parsers);
            // Remove any terminated parsers
            cache_data_inner.existing_parsers.retain(|(parser, parse_results)| {
                let parse_results = parse_results.borrow();
                let terminated = parse_results.up_data_vec.is_empty() && parse_results.right_data_vec.is_empty();
                !terminated
            });
        }
        // Step existing parsers
        for (parser, results) in self.cache_data_inner.borrow_mut().existing_parsers.iter_mut() {
            *results.borrow_mut() = parser.step(c);
        }
        self.inner.step(c)
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a dyn ParserTrait> + 'a> {
        // Iterate over the inner parser, new parsers in the cache, and the existing parsers in the cache
        todo!()
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut dyn ParserTrait> + 'a> {
        // Iterate over the inner parser, new parsers in the cache, and the existing parsers in the cache
        todo!()
    }

    fn collect_stats(&self, stats: &mut Stats) {
        self.inner.collect_stats(stats);
        for (parser, _) in self.cache_data_inner.borrow().new_parsers.iter() {
            parser.collect_stats(stats);
        }
        for (parser, _) in self.cache_data_inner.borrow().existing_parsers.iter() {
            parser.collect_stats(stats);
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
    pub parse_results: Rc<RefCell<ParseResults>>,
}

impl CombinatorTrait for Cached {
    type Parser = CachedParser;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        // Try to use an already-initialized new parser
        if let Some(i) = right_data.cache_data.inner.as_ref().unwrap().borrow_mut().new_parsers_i.get(&self.inner.clone().into()) {
            let parse_results_rc_refcell = right_data.cache_data.inner.as_ref().unwrap().borrow_mut().new_parsers[*i].1.clone();
            (CachedParser { parse_results: parse_results_rc_refcell.clone() }, parse_results_rc_refcell.clone().borrow().clone())
        } else {
            // Create a new parser
            let (parser, parse_results) = self.inner.parser(right_data.clone());
            let parse_results_rc_refcell = Rc::new(RefCell::new(parse_results.clone()));
            let i = right_data.cache_data.inner.as_ref().unwrap().borrow_mut().new_parsers.len();
            right_data.cache_data.inner.as_ref().unwrap().borrow_mut().new_parsers_i.insert(self.inner.clone().into(), i);
            right_data.cache_data.inner.as_ref().unwrap().borrow_mut().new_parsers.push((Box::new(parser), parse_results_rc_refcell.clone()));
            (CachedParser { parse_results: parse_results_rc_refcell }, parse_results)
        }
    }
}

impl ParserTrait for CachedParser {
    fn step(&mut self, c: u8) -> ParseResults {
        self.parse_results.borrow().clone()
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