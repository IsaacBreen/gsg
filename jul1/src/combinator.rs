use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

use crate::{CacheContext, CacheContextParser, Cached, CachedParser, Choice, ChoiceParser, EatString, EatStringParser, EatU8, EatU8Parser, Eps, EpsParser, Fail, FailParser, ForbidFollows, ForbidFollowsCheckNot, ForbidFollowsClear, ForwardRef, FrameStackOp, FrameStackOpParser, IndentCombinator, IndentCombinatorParser, MutateRightData, MutateRightDataParser, ParseResults, Repeat1, Repeat1Parser, RightData, Seq, SeqParser, Symbol, SymbolParser, Tagged, TaggedParser, U8Set, WithNewFrame, WithNewFrameParser};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Stats {
    pub active_parser_type_counts: BTreeMap<String, usize>,
    pub active_symbols: BTreeMap<String, usize>,
    pub active_tags: BTreeMap<String, usize>,
    pub active_string_matchers: BTreeMap<String, usize>,
    pub active_u8_matchers: BTreeMap<U8Set, usize>,
}

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn write_sorted<S: Clone + Display>(f: &mut Formatter, title: &str, items: &[(S, usize)]) -> std::fmt::Result {
            writeln!(f, "{}", title)?;
            let mut sorted_items = items.to_vec();
            sorted_items.sort_by(|a, b| a.1.cmp(&b.1));
            for (name, count) in sorted_items {
                let mut name = name.to_string();
                if name.len() > 80 {
                    name.truncate(80);
                    name.push_str("...");
                }
                writeln!(f, "    {}: {}", name, count)?;
            }
            writeln!(f, "")
        }

        write_sorted(f, "Active Parser Types:", self.active_parser_type_counts.clone().into_iter().collect::<Vec<_>>().as_slice())?;
        write_sorted(f, "Active Tags:", self.active_tags.clone().into_iter().collect::<Vec<_>>().as_slice())?;
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Combinator {
    Seq(Box<Seq>),
    Choice(Box<Choice>),
    EatU8(EatU8),
    Eps(Eps),
    EatString(EatString),
    Fail(Fail),
    CacheContext(CacheContext),
    Cached(Cached),
    Indent(IndentCombinator),
    MutateRightData(MutateRightData),
    Repeat1(Repeat1),
    Symbol(Symbol),
    Tagged(Tagged),
    ForwardRef(ForwardRef),
    FrameStackOp(FrameStackOp),
    WithNewFrame(WithNewFrame),
    ForbidFollows(ForbidFollows),
    ForbidFollowsClear(ForbidFollowsClear),
    ForbidFollowsCheckNot(ForbidFollowsCheckNot),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Parser {
    Seq(SeqParser),
    Choice(ChoiceParser),
    EatU8(EatU8Parser),
    EatString(EatStringParser),
    Eps(EpsParser),
    FailParser(FailParser),
    CacheContext(CacheContextParser),
    Cached(CachedParser),
    FrameStackOp(FrameStackOpParser),
    MutateRightData(MutateRightDataParser),
    Indent(IndentCombinatorParser),
    Repeat1(Repeat1Parser),
    Symbol(SymbolParser),
    Tagged(TaggedParser),
    WithNewFrame(WithNewFrameParser),
}

pub trait CombinatorTrait {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults);
}

pub trait ParserTrait {
    fn step(&mut self, c: u8) -> ParseResults;
    fn stats(&self) -> Stats {
        let mut stats = Stats::default();
        self.collect_stats(&mut stats);
        stats
    }
    fn collect_stats(&self, stats: &mut Stats);
    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Parser> + 'a>;
    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut Parser> + 'a>;

}

impl CombinatorTrait for Combinator {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        match self {
            Combinator::Seq(seq) => seq.parser(right_data),
            Combinator::Choice(choice) => choice.parser(right_data),
            Combinator::EatU8(eat_u8) => eat_u8.parser(right_data),
            Combinator::EatString(eat_string) => eat_string.parser(right_data),
            Combinator::Eps(eps) => eps.parser(right_data),
            Combinator::Fail(fail) => fail.parser(right_data),
            Combinator::CacheContext(cache_context) => cache_context.parser(right_data),
            Combinator::Cached(cached) => cached.parser(right_data),
            Combinator::Indent(indent) => indent.parser(right_data),
            Combinator::MutateRightData(mutate_right_data) => mutate_right_data.parser(right_data),
            Combinator::Repeat1(repeat1) => repeat1.parser(right_data),
            Combinator::Symbol(symbol) => symbol.parser(right_data),
            Combinator::Tagged(tagged) => tagged.parser(right_data),
            Combinator::ForwardRef(forward_ref) => forward_ref.parser(right_data),
            Combinator::FrameStackOp(frame_stack_op) => frame_stack_op.parser(right_data),
            Combinator::WithNewFrame(with_new_frame) => with_new_frame.parser(right_data),
            Combinator::ForbidFollows(forbid_follows) => forbid_follows.parser(right_data),
            Combinator::ForbidFollowsClear(forbid_follows_clear) => forbid_follows_clear.parser(right_data),
            Combinator::ForbidFollowsCheckNot(forbid_follows_check_not) => forbid_follows_check_not.parser(right_data),
        }
    }
}

impl ParserTrait for Parser {
    fn step(&mut self, c: u8) -> ParseResults {
        match self {
            Parser::Seq(seq) => seq.step(c),
            Parser::Choice(choice) => choice.step(c),
            Parser::EatU8(eat_u8) => eat_u8.step(c),
            Parser::EatString(eat_string) => eat_string.step(c),
            Parser::Eps(eps) => eps.step(c),
            Parser::FailParser(fail) => fail.step(c),
            Parser::CacheContext(cache_context) => cache_context.step(c),
            Parser::Cached(cached) => cached.step(c),
            Parser::FrameStackOp(frame_stack_op) => frame_stack_op.step(c),
            Parser::MutateRightData(mutate_right_data) => mutate_right_data.step(c),
            Parser::Indent(indent) => indent.step(c),
            Parser::Repeat1(repeat1) => repeat1.step(c),
            Parser::Symbol(symbol) => symbol.step(c),
            Parser::Tagged(tagged) => tagged.step(c),
            Parser::WithNewFrame(with_new_frame) => with_new_frame.step(c),
        }
    }

    fn collect_stats(&self, stats: &mut Stats) {
        todo!()
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Parser> + 'a> {
        todo!()
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut Parser> + 'a> {
        todo!()
    }
}
