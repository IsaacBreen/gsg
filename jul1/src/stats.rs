use std::collections::HashMap;
use crate::{Seq2, Choice2, EatU8, EatString, Repeat1, FrameStackOp, Eps, BruteForce, IndentCombinator, Symbol, ParserTrait, CombinatorTrait};
use crate::{Seq2Parser, Choice2Parser, EatU8Parser, EatStringParser, Repeat1Parser, FrameStackOpParser, EpsParser, BruteForceParser, IndentCombinatorParser};
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct Stats
{
    pub active_parser_type_counts: HashMap<String, usize>,
}

pub trait ParserStats
where
    Self: 'static,
{
    fn stats(&self) -> Stats {
        let mut stats = Stats::default();
        self.collect_stats(&mut stats);
        stats
    }
    fn collect_stats(&self, stats: &mut Stats);
}

impl<B, ParserA> ParserStats for Seq2Parser<B, ParserA>
where
    B: CombinatorTrait,
    B::Parser: ParserStats,
    ParserA: ParserTrait + ParserStats,
{
    fn collect_stats(&self, stats: &mut Stats) {
        self.a.as_ref().map(|a| a.collect_stats(stats));
        for b in &self.bs {
            b.collect_stats(stats);
        }
        stats.active_parser_type_counts.entry("Seq2Parser".to_string()).and_modify(|c| *c += 1).or_insert(1);
    }
}

impl<A, B> ParserStats for Choice2Parser<A, B>
where
    A: ParserTrait + ParserStats,
    B: ParserTrait + ParserStats,
{
    fn collect_stats(&self, stats: &mut Stats) {
        self.a.as_ref().map(|a| a.collect_stats(stats));
        self.b.as_ref().map(|b| b.collect_stats(stats));
        stats.active_parser_type_counts.entry("Choice2Parser".to_string()).and_modify(|c| *c += 1).or_insert(1);
    }
}

impl ParserStats for EatU8Parser {
    fn collect_stats(&self, stats: &mut Stats) {
        stats.active_parser_type_counts.entry("EatU8Parser".to_string()).and_modify(|c| *c += 1).or_insert(1);
    }
}

impl ParserStats for EatStringParser {
    fn collect_stats(&self, stats: &mut Stats) {
        stats.active_parser_type_counts.entry("EatStringParser".to_string()).and_modify(|c| *c += 1).or_insert(1);
    }
}

impl<T> ParserStats for Repeat1Parser<T>
where
    T: CombinatorTrait,
    T::Parser: ParserStats,
{
    fn collect_stats(&self, stats: &mut Stats) {
        self.a_parsers.iter().for_each(|a| a.collect_stats(stats));
        stats.active_parser_type_counts.entry("Repeat1Parser".to_string()).and_modify(|c| *c += 1).or_insert(1);
    }
}

impl<T> ParserStats for FrameStackOpParser<T>
where
    T: ParserTrait + ParserStats,
{
    fn collect_stats(&self, stats: &mut Stats) {
        self.a.collect_stats(stats);
        stats.active_parser_type_counts.entry("FrameStackOpParser".to_string()).and_modify(|c| *c += 1).or_insert(1);
    }
}

impl ParserStats for EpsParser {
    fn collect_stats(&self, stats: &mut Stats) {
        stats.active_parser_type_counts.entry("EpsParser".to_string()).and_modify(|c| *c += 1).or_insert(1);
    }
}

impl ParserStats for BruteForceParser{
    fn collect_stats(&self, stats: &mut Stats) {
        stats.active_parser_type_counts.entry("BruteForceParser".to_string()).and_modify(|c| *c += 1).or_insert(1);
    }
}

impl ParserStats for IndentCombinatorParser {
    fn collect_stats(&self, stats: &mut Stats) {
        stats.active_parser_type_counts.entry("IndentCombinatorParser".to_string()).and_modify(|c| *c += 1).or_insert(1);
    }
}

impl<T> ParserStats for Box<T>
where
    T: ParserTrait + ParserStats,
{
    fn collect_stats(&self, stats: &mut Stats) {
        self.as_ref().collect_stats(stats);
    }
}

impl<T> ParserStats for Rc<T>
where
    T: ParserTrait + ParserStats,
{
    fn collect_stats(&self, stats: &mut Stats) {
        self.as_ref().collect_stats(stats);
    }
}

impl ParserStats for dyn ParserTrait {
    fn collect_stats(&self, stats: &mut Stats) {
        // (*self).collect_stats(stats);
    }
}