use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::any::Any;
use std::hash::Hasher;
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
        fn write_sorted<S: Clone + Display>(f: &mut Formatter, title: &str, items: &[(S, usize)]) -> std::fmt::Result {
            writeln!(f, "{}", title)?;
            let mut sorted_items = items.to_vec();
            sorted_items.sort_by(|a, b| a.1.cmp(&b.1));
            for (name, count) in sorted_items {
                writeln!(f, "    {}: {}", name, count)?;
            }
            writeln!(f, "")
        }

        write_sorted(f, "Active Parser Types:", self.active_parser_type_counts.clone().into_iter().collect::<Vec<_>>().as_slice())?;
        write_sorted(f, "Active Symbols:", self.active_symbols.clone().into_iter().collect::<Vec<_>>().as_slice())?;
        write_sorted(f, "Active Tags:", self.active_tags.clone().into_iter().collect::<Vec<_>>().as_slice())?;
        write_sorted(f, "Active String Matchers:", self.active_string_matchers.clone().into_iter().collect::<Vec<_>>().as_slice())?;
        write_sorted(f, "Active U8 Matchers:", self.active_u8_matchers.clone().into_iter().collect::<Vec<_>>().as_slice())?;

        Ok(())
    }
}

impl Stats {
    pub fn total_active_parsers(&self) -> usize {
        self.active_parser_type_counts.values().sum()
    }

    pub fn total_active_symbols(&self) -> usize {
        self.active_symbols.values().sum()
    }

    pub fn total_active_tags(&self) -> usize {
        self.active_tags.values().sum()
    }

    pub fn total_active_string_matchers(&self) -> usize {
        self.active_string_matchers.values().sum()
    }

    pub fn total_active_u8_matchers(&self) -> usize {
        self.active_u8_matchers.values().sum()
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
    fn eq(&self, other: &dyn ParserTrait) -> bool where Self: PartialEq + Sized + 'static {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }
    fn dyn_hash(&self, state: &mut dyn Hasher) {}
    fn as_any(&self) -> &dyn Any;
}

impl<T: ParserTrait + 'static + PartialEq> ParserTrait for T {
    fn step(&mut self, c: u8) -> ParseResults {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

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

    fn as_any(&self) -> &dyn Any {
        self
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