use std::any::Any;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{CombinatorTrait, eps, IntoCombinator, ParseResults, ParserTrait, RightData, U8Set};

#[derive(Debug, Clone, Default)]
pub struct LookaheadData {
    pub lookaheads: Vec<Rc<RefCell<LookaheadFilter>>>,
}

impl Hash for LookaheadData {
    fn hash<H: Hasher>(&self, state: &mut H) {
    }
}

impl PartialEq for LookaheadData {
    fn eq(&self, other: &Self) -> bool {
        true
        // self.lookaheads.len() == other.lookaheads.len()
    }
}

impl Eq for LookaheadData {}

impl PartialOrd for LookaheadData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LookaheadData {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
        // self.lookaheads.len().cmp(&other.lookaheads.len())
    }
}

#[derive(Debug, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct LookaheadFilter {
    pub u8set: U8Set,
}

pub struct Lookahead<A> {
    pub inner: A,
}

pub struct LookaheadParser<P> {
    pub inner: P,
    pub filter: Rc<RefCell<LookaheadFilter>>,
}

pub struct FilteredTerminal<A> {
    pub inner: A,
}

pub struct FilteredTerminalParser<P> {
    pub inner: P,
    pub prev_lookahead_filter: LookaheadFilter,
    pub lookahead_data: LookaheadData,
    done: bool,
}

impl<T: CombinatorTrait> CombinatorTrait for Lookahead<T> {
    type Parser = LookaheadParser<T::Parser>;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        let (inner, parse_result) = self.inner.parser(right_data.clone());
        let mut u8set = U8Set::none();
        for up_data in &parse_result.up_data_vec {
            u8set |= up_data.u8set;
        }
        let filter = LookaheadFilter { u8set };
        let filter_rc_refcell = Rc::new(RefCell::new(filter));
        right_data.lookahead_data.lookaheads.push(filter_rc_refcell.clone());
        (LookaheadParser { inner, filter: filter_rc_refcell }, ParseResults {
            up_data_vec: vec![],
            right_data_vec: vec![right_data],
            cut: parse_result.cut,
            done: true,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<P: ParserTrait + 'static> ParserTrait for LookaheadParser<P> {
    fn step(&mut self, c: u8) -> ParseResults {
        let parse_results = self.inner.step(c);
        let mut u8set = U8Set::none();
        for up_data in &parse_results.up_data_vec {
            u8set |= up_data.u8set;
        }
        self.filter.borrow_mut().u8set |= u8set;
        ParseResults::empty_unfinished()
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

impl<T: CombinatorTrait> CombinatorTrait for FilteredTerminal<T> {
    type Parser = FilteredTerminalParser<T::Parser>;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        let lookahead_data = right_data.lookahead_data.clone();
        let (inner, mut parse_results) = self.inner.parser(right_data.clone());
        let filter = lookahead_data.get_u8set();
        let mut merged = filter.clone();
        dbg!(&filter);
        for up_data in &mut parse_results.up_data_vec {
            dbg!(&up_data.u8set);
            up_data.u8set = up_data.u8set.intersection(&filter);
            merged |= up_data.u8set;
        }
        if merged.is_empty() {
            parse_results.done = true;
        }
        (FilteredTerminalParser { inner, prev_lookahead_filter: LookaheadFilter { u8set: filter }, lookahead_data, done: parse_results.done }, parse_results)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<P: ParserTrait + 'static> ParserTrait for FilteredTerminalParser<P> {
    fn step(&mut self, c: u8) -> ParseResults {
        if !self.prev_lookahead_filter.u8set.contains(c) {
            self.done = true;
        }
        if self.done {
            return ParseResults::empty_finished();
        }
        let mut parse_results = self.inner.step(c);
        let filter = self.lookahead_data.get_u8set();
        let mut merged = filter.clone();
        for up_data in &mut parse_results.up_data_vec {
            up_data.u8set = up_data.u8set.intersection(&filter);
            merged |= up_data.u8set;
        }
        if merged.is_empty() {
            parse_results.done = true;
            self.done = true;
        }
        parse_results
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

impl LookaheadData {
    pub fn get_u8set(&self) -> U8Set {
        if self.lookaheads.is_empty() {
            U8Set::all()
        } else {
            let mut u8set = U8Set::none();
            for lookahead in &self.lookaheads {
                u8set |= lookahead.borrow().u8set;
            }
            u8set
        }
    }
}

pub fn lookahead<T>(t: T) -> Lookahead<T::Output>
where
    T: IntoCombinator,
{
    // Lookahead {
    //     inner: t.into_combinator(),
    // }
    // todo: lookaheads are not working
    eps()
}

pub fn filtered_terminal<T>(t: T) -> FilteredTerminal<T::Output>
where
    T: IntoCombinator,
{
    // FilteredTerminal {
    //     inner: t.into_combinator(),
    // }
    // todo: lookaheads are not working
    eps()
}