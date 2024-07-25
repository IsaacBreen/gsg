use std::any::Any;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{CombinatorTrait, IntoCombinator, ParseResults, ParserTrait, RightData, U8Set};

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct LookaheadData {
    pub lookaheads: Vec<Rc<RefCell<LookaheadFilter>>>,
}

impl Hash for LookaheadData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lookaheads.len().hash(state);
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
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<P> ParserTrait for LookaheadParser<P> {
    fn step(&mut self, c: u8) -> ParseResults {
        let parse_results = self.inner.step(c);
        let mut u8set = U8Set::none();
        for up_data in &parse_results.up_data_vec {
            u8set |= up_data.u8set;
        }
        self.filter.borrow_mut().u8set |= u8set;
        ParseResults::no_match()
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

pub fn lookahead<T>(t: T) -> Lookahead<T::Output>
where
    T: IntoCombinator,
{
    Lookahead {
        inner: t.into_combinator(),
    }
}