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

    }
}

#[derive(Default)]
pub struct CacheDataInner {
    pub new_parsers: Vec<(Rc<DynCombinator>, (Rc<RefCell<dyn ParserTrait>>, Rc<RefCell<Option<ParseResults>>>))>,
    pub parse_results: Vec<Rc<RefCell<Option<ParseResults>>>>,
}

impl Debug for CacheDataInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
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
        Some(self.cmp(other))
    }
}

impl Ord for CacheDataInner {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.new_parsers.len().cmp(&other.new_parsers.len())
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
        let mut parser = CacheContextParser { inner: parser, cache_data_inner };
        (parser, results)
    }
}

impl<P> ParserTrait for CacheContextParser<P>
where
    P: ParserTrait + 'static,
{
    fn step(&mut self, c: u8) -> ParseResults {
        self.cache_data_inner.borrow_mut().new_parsers.clear();
        for parse_result in self.cache_data_inner.borrow().parse_results.iter() {
            parse_result.borrow_mut().take();
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
    pub parser: Rc<RefCell<dyn ParserTrait>>,
    pub parser_gt: Box<dyn ParserTrait>,
}

impl CombinatorTrait for Cached {
    type Parser = CachedParser;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        for (combinator, (parser, parse_results_rc_refcell)) in right_data.cache_data.inner.as_ref().unwrap().borrow().new_parsers.iter() {
            if Rc::ptr_eq(combinator, &self.inner) {
                let parse_results = parse_results_rc_refcell.borrow().clone().expect("CachedParser.parser: parse_results is None");
                let (parser_gt, mut parse_results_gt) = self.inner.parser(right_data.clone());
                parse_results_gt.squash();
                assert_eq!(parse_results, parse_results_gt);
                return (CachedParser { parse_results: parse_results_rc_refcell.clone(), parser: parser.clone(), parser_gt: Box::new(parser_gt) }, parse_results);
            }
        }

        let (parser, mut parse_results) = self.inner.parser(right_data.clone());
        let (parser_gt, _) = self.inner.parser(right_data.clone());
        parse_results.squash();
        let parse_results_rc_refcell = Rc::new(RefCell::new(Some(parse_results.clone())));
        let parser_rc_refcell = Rc::new(RefCell::new(parser));
        let mut cache_data_inner = right_data.cache_data.inner.as_ref().unwrap().borrow_mut();
        cache_data_inner.new_parsers.push((self.inner.clone(), (parser_rc_refcell.clone(), parse_results_rc_refcell.clone())));
        (CachedParser { parse_results: parse_results_rc_refcell, parser: parser_rc_refcell, parser_gt: Box::new(parser_gt) }, parse_results)
    }
}

impl ParserTrait for CachedParser {
    fn step(&mut self, c: u8) -> ParseResults {
        let mut parse_results_gt = self.parser_gt.step(c);
        parse_results_gt.squash();
        if let Some(parse_results) = self.parse_results.borrow().clone() {
            assert_eq!(parse_results, parse_results_gt);
            parse_results
        } else {
            let mut parser = self.parser.borrow_mut();
            let mut parse_results = parser.step(c);
            parse_results.squash();
            assert_eq!(parse_results, parse_results_gt);
            self.parse_results.borrow_mut().replace(parse_results.clone());
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