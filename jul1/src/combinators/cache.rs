use std::any::Any;
use std::cell::RefCell;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{CombinatorTrait, DynCombinator, IntoCombinator, ParseResults, ParserTrait, RightData, Squash, Stats};

#[derive(Debug, Clone, PartialEq, Default, PartialOrd, Ord, Eq)]
pub struct CacheData {
    pub inner: Option<Rc<RefCell<CacheDataInner>>>,
}

impl Hash for CacheData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(ref inner) = self.inner {
            inner.borrow().hash(state);
        }
    }
}

pub struct CacheEntry {
    parser: Box<dyn ParserTrait>,
    maybe_parse_results: Option<ParseResults>,
}

#[derive(Default)]
pub struct CacheDataInner {
    pub new_parsers: Vec<(Rc<DynCombinator>, Rc<RefCell<CacheEntry>>)>,
    pub entries: Vec<Rc<RefCell<CacheEntry>>>,
}

impl Debug for CacheDataInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CacheDataInner")
    }
}

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

impl<T> CacheContextParser<T> {}

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
        for entry in self.cache_data_inner.borrow_mut().entries.iter() {
            entry.borrow_mut().maybe_parse_results.take();
        }
        self.inner.step(c)
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
            entry.borrow().parser.collect_stats(stats);
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
    pub parser_gt: Box<dyn ParserTrait>,
}

impl CombinatorTrait for Cached {
    type Parser = CachedParser;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        for (combinator, entry) in right_data.cache_data.inner.as_ref().unwrap().borrow().new_parsers.iter() {
            if Rc::ptr_eq(combinator, &self.inner) {
                let parse_results = entry.borrow().maybe_parse_results.clone().expect("CachedParser.parser: parse_results is None");
                let (parser_gt, mut parse_results_gt) = self.inner.parser(right_data.clone());
                parse_results_gt.squash();
                assert_eq!(parse_results, parse_results_gt);
                return (
                    CachedParser {
                        entry: entry.clone(),
                        parser_gt: Box::new(parser_gt)
                    },
                    parse_results
                );
            }
        }

        let (parser, mut parse_results) = self.inner.parser(right_data.clone());
        let (parser_gt, _) = self.inner.parser(right_data.clone());
        parse_results.squash();
        let parse_results_rc_refcell = Rc::new(RefCell::new(Some(parse_results.clone())));
        let entry = Rc::new(RefCell::new(CacheEntry {
            parser,
            maybe_parse_results: Some(parse_results.clone())
        }));
        let mut cache_data_inner = right_data.cache_data.inner.as_ref().unwrap().borrow_mut();
        cache_data_inner.new_parsers.push((self.inner.clone(), entry.clone()));
        cache_data_inner.entries.push(entry.clone());
        (CachedParser {
            entry,
            parser_gt: Box::new(parser_gt)
        },
        parse_results)
    }
}

impl ParserTrait for CachedParser {
    fn step(&mut self, c: u8) -> ParseResults {
        let mut parse_results_gt = self.parser_gt.step(c);
        parse_results_gt.squash();
        if let Some(parse_results) = { let binding = self.entry.borrow(); binding.maybe_parse_results.clone() } {
            assert_eq!(parse_results, parse_results_gt);
            parse_results
        } else {
            let mut entry = self.entry.borrow_mut();
            let mut parse_results = entry.parser.step(c);
            parse_results.squash();
            assert_eq!(parse_results, parse_results_gt);
            entry.maybe_parse_results.replace(parse_results.clone());
            parse_results
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