use std::collections::BTreeMap;
use std::rc::Rc;

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

pub trait CombinatorTrait
where
    Self: 'static,
{
    type Parser: ParserTrait;
    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>);
    fn into_boxed(self) -> Box<DynCombinator>
    where
        Self: Sized,
    {
        Box::new(DynWrapper(self))
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
}


impl ParserTrait for Box<dyn ParserTrait> {
    fn step(&mut self, c: u8) -> ParseResults {
        (**self).step(c)
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a dyn ParserTrait> + 'a> {
        (**self).iter_children()
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut dyn ParserTrait> + 'a> {
        (**self).iter_children_mut()
    }
}

impl<C> CombinatorTrait for Rc<C>
where
    C: CombinatorTrait + ?Sized,
{
    type Parser = C::Parser;

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
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

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        let (parser, right_data, up_data) = self.0.parser(right_data);
        (Box::new(parser), right_data, up_data)
    }
}

impl CombinatorTrait for Box<DynCombinator> {
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
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