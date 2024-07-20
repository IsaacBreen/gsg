use std::collections::BTreeMap;
use std::fmt::Display;
use std::hash::Hasher;
use std::rc::Rc;
use dyn_eq::DynEq;
use crate::parse_state::{RightData, UpData};
use crate::{ParseResults, U8Set};

#[derive(Default, Debug)]
pub struct Stats
{
    pub active_parser_type_counts: BTreeMap<String, usize>,
    pub active_symbols: BTreeMap<String, usize>,
    pub active_tags: BTreeMap<String, usize>,
    pub active_string_matchers: BTreeMap<String, usize>,
    pub active_u8_matchers: BTreeMap<U8Set, usize>,
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "Active Parser Types:")?;
        // for (name, count) in &self.active_parser_type_counts {
        //     writeln!(f, "    {}: {}", name, count)?;
        // }
        // writeln!(f, "Active Symbols:")?;
        // for (name, count) in &self.active_symbols {
        //     writeln!(f, "    {}: {}", name, count)?;
        // }
        writeln!(f, "Active Tags:")?;
        for (name, count) in &self.active_tags {
            writeln!(f, "    {}: {}", name, count)?;
        }
        // writeln!(f, "Active String Matchers:")?;
        // for (name, count) in &self.active_string_matchers {
        //     writeln!(f, "    {}: {}", name, count)?;
        // }
        // writeln!(f, "Active U8 Matchers:")?;
        // for (name, count) in &self.active_u8_matchers {
        //     writeln!(f, "    {}: {}", name, count)?;
        // }
        Ok(())
    }
}

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
}

pub trait ParserTrait: DynEq {
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
}

impl PartialEq for Box<dyn ParserTrait> {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other)
    }
}
impl Eq for Box<dyn ParserTrait> {}

impl ParserTrait for Box<dyn ParserTrait> {
    fn step(&mut self, c: u8) -> ParseResults {
        (**self).step(c)
    }

    fn stats(&self) -> Stats {
        (**self).stats()
    }

    fn collect_stats(&self, stats: &mut Stats) {
        (**self).collect_stats(stats)
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a dyn ParserTrait> + 'a> {
        (**self).iter_children()
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut dyn ParserTrait> + 'a> {
        (**self).iter_children_mut()
    }

    fn gc(&mut self) {
        (**self).gc()
    }
}

impl<C> CombinatorTrait for Rc<C>
where
    C: CombinatorTrait + ?Sized,
{
    type Parser = C::Parser;

    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        (**self).parser(right_data)
    }
}

struct DynWrapper<T>(T);

impl<T, P> CombinatorTrait for DynWrapper<T>
where
    T: CombinatorTrait<Parser=P>,
    P: ParserTrait + 'static,
{
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        let (parser, parse_results) = self.0.parser(right_data);
        (Box::new(parser), parse_results)
    }
}

impl CombinatorTrait for Box<DynCombinator> {
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        (**self).parser(right_data)
    }
}

pub type DynCombinator = dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>;

pub trait IntoCombinator {
    type Output: CombinatorTrait;
    fn into_combinator(self) -> Self::Output;
}

impl<T> IntoCombinator for T
where
    T: CombinatorTrait,
{
    type Output = T;
    fn into_combinator(self) -> Self::Output {
        self
    }
}

impl<T> IntoCombinator for &Rc<T>
where
    T: CombinatorTrait,
{
    type Output = Rc<T>;
    fn into_combinator(self) -> Self::Output {
        self.clone()
    }
}

// TODO: why do we need this? Should be able to delete it, but throws an error (try it!)
impl IntoCombinator for &Rc<DynCombinator> {
    type Output = Rc<DynCombinator>;
    fn into_combinator(self) -> Self::Output {
        self.clone()
    }
}