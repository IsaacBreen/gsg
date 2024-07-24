use std::any::Any;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
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
        // Hash the pointer of the inner data
        // let inner = self.inner.as_ref().unwrap();
        // let ptr = Rc::as_ptr(inner) as usize;
        // ptr.hash(state);
    }
}

pub struct ComparableRc<T: ?Sized>(pub Rc<T>);

impl From<Rc<DynCombinator>> for ComparableRc<DynCombinator> {
    fn from(rc: Rc<DynCombinator>) -> Self {
        ComparableRc(rc)
    }
}

// impl<T: ?Sized> PartialEq for ComparableRc<T> {
//     fn eq(&self, other: &Self) -> bool {
//         Rc::ptr_eq(&self.0, &other.0)
//     }
// }
//
// impl<T: ?Sized> Eq for ComparableRc<T> {}
//
// impl<T: ?Sized> PartialOrd for ComparableRc<T> {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
// impl<T: ?Sized> Ord for ComparableRc<T> {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         // Compare the pointers of the inner data
//         let ptr = std::ptr::addr_of!(self.0) as *const ();
//         let other_ptr = std::ptr::addr_of!(other.0) as *const ();
//         ptr.cmp(&other_ptr)
//     }
// }

// impl<T: ?Sized> Hash for ComparableRc<T> {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         // Rc::as_ptr(&self.0).hash(state);
//     }
// }

#[derive(Default)]
pub struct CacheDataInner {
    // pub new_parsers_i: HashMap<ComparableRc<DynCombinator>, usize>,
    // pub new_parsers_i: Vec<(ComparableRc<DynCombinator>, usize)>,
    // pub new_parsers: Vec<(Box<dyn ParserTrait>, Rc<RefCell<Option<ParseResults>>>)>,

    pub new_parsers: Vec<(ComparableRc<DynCombinator>, (Box<dyn ParserTrait>, Rc<RefCell<Option<ParseResults>>>))>,

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

macro_rules! time {
    ($desc:expr, $code:block) => {
        let start = std::time::Instant::now();
        let r = $code;
        let end = std::time::Instant::now();
        println!("{}: {:?}", $desc, end - start);
        r
    };
}

impl<P> ParserTrait for CacheContextParser<P>
where
    P: ParserTrait + 'static,
{
    fn step(&mut self, c: u8) -> ParseResults {
        time! ("CacheContextParser.step part 1", {
            // Move new parsers to existing parsers
            let mut cache_data_inner = self.cache_data_inner.borrow_mut();
            // cache_data_inner.new_parsers_i.clear();
            let mut new_parsers = std::mem::take(&mut cache_data_inner.new_parsers);
            new_parsers.reverse();
            // cache_data_inner.existing_parsers.extend(new_parsers);
            for (_, x) in new_parsers.into_iter() {
                cache_data_inner.existing_parsers.push(x);
            }
            // // Remove any terminated parsers
            // cache_data_inner.existing_parsers.retain(|(parser, parse_results)| {
            //     let binding = parse_results.borrow();
            //     let parse_results = binding.as_ref().expect("CacheContextParser.step: parse_results is None");
            //     let terminated = parse_results.up_data_vec.is_empty() && parse_results.right_data_vec.is_empty();
            //     !terminated
            // });
        });
        let mut existing_parsers = std::mem::take(&mut self.cache_data_inner.borrow_mut().existing_parsers);
        time! ("CacheContextParser.step part 2", {
            // Step existing parsers
            existing_parsers.reverse();
            // First, clear existing results
            for (_, results) in existing_parsers.iter_mut() {
                results.borrow_mut().take();
            }
        });
        println!("Number of existing parsers: {}", existing_parsers.len());
        time! ("CacheContextParser.step part 3", {
            // Second, compute new results
            for (mut parser, results) in existing_parsers.into_iter() {
                let mut new_results = parser.step(c);
                // new_results.squash();
                *results.borrow_mut() = Some(new_results);
                self.cache_data_inner.borrow_mut().existing_parsers.push((parser, results));
            }
        });
        time! ("CacheContextParser.step part 4", {
            self.cache_data_inner.borrow_mut().existing_parsers.reverse();
        });
        let r;
        time! ("CacheContextParser.step part 5", {
            r = self.inner.step(c)
        });
        println!("\n\n\n");
        r
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
        // Try to use an already-initialized new parser
        let mut maybe_i = {
            let cache_data_inner = right_data.cache_data.inner.as_ref().unwrap().borrow();
            // cache_data_inner.new_parsers_i.get(&self.inner.clone().into()).cloned()
            // cache_data_inner.new_parsers_i.iter().position(|(parser, _)| Rc::ptr_eq(&parser.0, &self.inner))
            Some(0)
        };
        // maybe_i = None;
        if let Some(i) = maybe_i {
            let parse_results_rc_refcell = right_data.cache_data.inner.as_ref().unwrap().borrow_mut().new_parsers[i].1.1.clone();
            let parse_results = parse_results_rc_refcell.borrow().clone().expect("CachedParser.parser: parse_results is None");

            let (_, parse_results_gt) = self.inner.parser(right_data.clone());
            assert_eq!(parse_results, parse_results_gt);

            (CachedParser { parse_results: parse_results_rc_refcell }, parse_results)
        } else {
            // Create a new parser
            let (parser, mut parse_results) = self.inner.parser(right_data.clone());
            // parse_results.squash();
            let parse_results_rc_refcell = Rc::new(RefCell::new(Some(parse_results.clone())));
            let mut cache_data_inner = right_data.cache_data.inner.as_ref().unwrap().borrow_mut();
            // let i = cache_data_inner.new_parsers.len();
            // cache_data_inner.new_parsers_i.insert(self.inner.clone().into(), i);
            // cache_data_inner.new_parsers_i.push((self.inner.clone().into(), i));
            // cache_data_inner.new_parsers.push((Box::new(parser), parse_results_rc_refcell.clone()));
            // assert!(cache_data_inner.new_parsers_i.len() == cache_data_inner.new_parsers.len());
            cache_data_inner.new_parsers.push((self.inner.clone().into(), (Box::new(parser), parse_results_rc_refcell.clone())));
            (CachedParser { parse_results: parse_results_rc_refcell }, parse_results)
        }
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