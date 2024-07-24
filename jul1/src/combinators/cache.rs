use std::any::Any;
use std::cell::RefCell;
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

    }
}

#[derive(Default)]
pub struct CacheDataInner {
    pub new_parsers: Vec<(Rc<DynCombinator>, (Box<dyn ParserTrait>, Rc<RefCell<Option<ParseResults>>>))>,
    pub existing_parsers: Vec<(Box<dyn ParserTrait>, Rc<RefCell<Option<ParseResults>>>)>,
}

impl Debug for CacheDataInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl PartialEq for CacheDataInner {
    fn eq(&self, other: &Self) -> bool {
        todo!();
        std::ptr::eq(self, other)
    }
}

impl Eq for CacheDataInner {}

impl PartialOrd for CacheDataInner {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!();
        Some(self.cmp(other))
    }
}

impl Ord for CacheDataInner {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!();
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
            let mut cache_data_inner = self.cache_data_inner.borrow_mut();
            let mut new_parsers = std::mem::take(&mut cache_data_inner.new_parsers);
            new_parsers.reverse();
            for (_, x) in new_parsers {
                cache_data_inner.existing_parsers.push(x);
            }
        }

        // Ensure no results appear more than once
        for i in 0..self.cache_data_inner.borrow().existing_parsers.len() {
            for j in 0..i {
                let (parser_i, results_i) = self.cache_data_inner.borrow().existing_parsers[i].clone();
                let (parser_j, results_j) = self.cache_data_inner.borrow().existing_parsers[j].clone();
                assert!(!Rc::ptr_eq(&parser_i, &parser_j));
            }
        }

        let mut existing_parsers = std::mem::take(&mut self.cache_data_inner.borrow_mut().existing_parsers);

        existing_parsers.reverse();
        for (_, results) in existing_parsers.iter() {
            results.borrow_mut().take();
        }

        for (mut parser, results) in existing_parsers.into_iter() {
            let mut new_results = parser.step(c);
            *results.borrow_mut() = Some(new_results);
            self.cache_data_inner.borrow_mut().existing_parsers.push((parser, results));
        }

        self.cache_data_inner.borrow_mut().existing_parsers.reverse();

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
        for (_, (parser, _)) in self.cache_data_inner.borrow().new_parsers.iter() {
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
    pub parse_results: Rc<RefCell<Option<ParseResults>>>,
}

impl CombinatorTrait for Cached {
    type Parser = CachedParser;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        for (combinator, (parser, parse_results_rc_refcell)) in right_data.cache_data.inner.as_ref().unwrap().borrow().new_parsers.iter() {
            if Rc::ptr_eq(combinator, &self.inner) {
                let parse_results = parse_results_rc_refcell.borrow().clone().expect("CachedParser.parser: parse_results is None");
                return (CachedParser { parse_results: parse_results_rc_refcell.clone() }, parse_results);
            }
        }

        let (parser, mut parse_results) = self.inner.parser(right_data.clone());
        let parse_results_rc_refcell = Rc::new(RefCell::new(Some(parse_results.clone())));
        let mut cache_data_inner = right_data.cache_data.inner.as_ref().unwrap().borrow_mut();
        cache_data_inner.new_parsers.push((self.inner.clone(), (parser, parse_results_rc_refcell.clone())));
        (CachedParser { parse_results: parse_results_rc_refcell }, parse_results)
    }
}

impl ParserTrait for CachedParser {
    fn step(&mut self, c: u8) -> ParseResults {
        self.parse_results.borrow().clone().expect("CachedParser.step: parse_results is None")
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