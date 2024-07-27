use std::any::Any;
use std::hash::Hasher;
use std::rc::Rc;
use crate::parse_state::{RightData, ParseResults};

pub trait CombinatorTrait
where
    Self: 'static,
{
    type Parser: ParserTrait;
    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults);
    fn into_box_dyn(self) -> Box<DynCombinator>
    where
        Self: Sized,
    {
        Box::new(DynWrapper(self))
    }
    fn into_rc_dyn(self) -> Rc<DynCombinator>
    where
        Self: Sized,
    {
        Rc::new(DynWrapper(self))
    }
    fn dyn_eq(&self, other: &DynCombinator) -> bool { std::ptr::eq(self.as_any(), other.as_any()) }
    fn dyn_hash(&self, state: &mut dyn Hasher) {}
    fn as_any(&self) -> &dyn Any;
}

pub trait ParserTrait {
    fn step(&mut self, c: u8) -> ParseResults;
    fn stats(&self) -> Stats {
        let mut stats = Stats::default();
        self.collect_stats(&mut stats);
        stats
    }
    fn collect_stats(&self, stats: &mut Stats) {
        stats.active_parser_type_counts.entry(std::any::type_name::<Self>().to_string()).and_modify(|c| *c += 1).or_insert(1);
        for child in self.iter_children() {
            child.collect_stats(stats);
        }
    }
    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn ParserTrait> + 'a> {
        Box::new(std::iter::empty())
    }
    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut dyn ParserTrait> + 'a> {
        Box::new(std::iter::empty())
    }
    fn gc(&mut self) {
        for child in self.iter_children_mut() {
            child.gc();
        }
    }
    fn dyn_eq(&self, other: &dyn ParserTrait) -> bool;
    fn dyn_hash(&self, state: &mut dyn Hasher) {}
    fn as_any(&self) -> &dyn Any;
}