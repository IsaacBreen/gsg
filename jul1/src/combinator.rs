use std::rc::Rc;

use crate::parse_state::{RightData, UpData};

use std::collections::HashMap;
// use crate::{Seq2, Choice2, EatU8, EatString, Repeat1, FrameStackOp, Eps, BruteForce, IndentCombinator, Symbol, ParserTrait, CombinatorTrait};
// use crate::{Seq2Parser, Choice2Parser, EatU8Parser, EatStringParser, Repeat1Parser, FrameStackOpParser, EpsParser, BruteForceParser, IndentCombinatorParser};
// use std::rc::Rc;

#[derive(Default, Debug)]
pub struct Stats
{
    pub active_parser_type_counts: HashMap<String, usize>,
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
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>);
    fn stats(&self) -> Stats {
        let mut stats = Stats::default();
        self.collect_stats(&mut stats);
        stats
    }
    fn collect_stats(&self, stats: &mut Stats);
}

impl ParserTrait for Box<dyn ParserTrait> {
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>) {
        (**self).step(c)
    }
    fn collect_stats(&self, stats: &mut Stats) {
        (**self).collect_stats(stats);
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