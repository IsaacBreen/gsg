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


macro_rules! define_enum {
    ($name:ident, $($variants:ident),*) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                $variants($variants),
            )*
        }
    };
}

macro_rules! match_enum {
    ($expr:expr, $enum:ident, $inner:ident => $arm:expr, $($variant:ident),*) => {
        match $expr {
            $(
                $enum::$variant($inner) => $arm,
            )*
        }
    };
}

define_enum!(
    Combinator,
    Seq,
    Choice,
    EatU8,
    EatString,
    Eps,
    Fail,
    CacheContext,
    Cached,
    IndentCombinator,
    FrameStackOp,
    MutateRightData,
    Repeat1,
    Symbol,
    Tagged,
    ForwardRef,
    WithNewFrame,
    ForbidFollows,
    ForbidFollowsClear,
    ForbidFollowsCheckNot
);

define_enum!(
    Parser,
    SeqParser,
    ChoiceParser,
    EatU8Parser,
    EatStringParser,
    EpsParser,
    FailParser,
    CacheContextParser,
    CachedParser,
    IndentCombinatorParser,
    FrameStackOpParser,
    MutateRightDataParser,
    Repeat1Parser,
    SymbolParser,
    TaggedParser,
    WithNewFrameParser
);

macro_rules! match_combinator {
    ($expr:expr, $inner:ident => $arm:expr) => {
        match_enum!($expr, Combinator, $inner => $arm,
            Seq,
            Choice,
            EatU8,
            EatString,
            Eps,
            Fail,
            CacheContext,
            Cached,
            IndentCombinator,
            FrameStackOp,
            MutateRightData,
            Repeat1,
            Symbol,
            Tagged,
            ForwardRef,
            WithNewFrame,
            ForbidFollows,
            ForbidFollowsClear,
            ForbidFollowsCheckNot
        )
    };
}

macro_rules! match_parser {
    ($expr:expr, $inner:ident => $arm:expr) => {
        match_enum!($expr, Parser, $inner => $arm,
            SeqParser,
            ChoiceParser,
            EatU8Parser,
            EatStringParser,
            EpsParser,
            FailParser,
            CacheContextParser,
            CachedParser,
            IndentCombinatorParser,
            FrameStackOpParser,
            MutateRightDataParser,
            Repeat1Parser,
            SymbolParser,
            TaggedParser,
            WithNewFrameParser
        )
    };
}

pub trait CombinatorTrait {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults);
}

pub trait ParserTrait {
    fn step(&mut self, c: u8) -> ParseResults;
}

impl CombinatorTrait for Combinator {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        match_combinator!(self, inner => inner.parser(right_data))
    }
}

impl ParserTrait for Parser {
    fn step(&mut self, c: u8) -> ParseResults {
        match_parser!(self, inner => inner.step(c))
    }
}

impl Parser {
    pub fn stats(&self) -> Stats {
        let mut stats = Stats::default();
        self.collect_stats(&mut stats);
        stats
    }

    pub fn collect_stats(&self, stats: &mut Stats) {
        match self {
            Parser::SeqParser(SeqParser { a, bs, .. }) => {
                if let Some(a) = a {
                    a.collect_stats(stats);
                }
                for b in bs {
                    b.collect_stats(stats);
                }
                *stats.active_parser_type_counts.entry("SeqParser".to_string()).or_insert(0) += 1;
            }
            Parser::ChoiceParser(ChoiceParser { a, b }) => {
                if let Some(a) = a {
                    a.collect_stats(stats);
                }
                if let Some(b) = b {
                    b.collect_stats(stats);
                }
                *stats.active_parser_type_counts.entry("ChoiceParser".to_string()).or_insert(0) += 1;
            }
            Parser::EatU8Parser(EatU8Parser { u8set, .. }) => {
                *stats.active_u8_matchers.entry(u8set.clone()).or_insert(0) += 1;
                *stats.active_parser_type_counts.entry("EatU8Parser".to_string()).or_insert(0) += 1;
            }
            Parser::EatStringParser(EatStringParser { string, .. }) => {
                *stats.active_string_matchers.entry(String::from_utf8_lossy(string).to_string()).or_insert(0) += 1;
                *stats.active_parser_type_counts.entry("EatStringParser".to_string()).or_insert(0) += 1;
            }
            Parser::EpsParser(_) => {
                *stats.active_parser_type_counts.entry("EpsParser".to_string()).or_insert(0) += 1;
            }
            Parser::FailParser(_) => {
                *stats.active_parser_type_counts.entry("FailParser".to_string()).or_insert(0) += 1;
            }
            Parser::CacheContextParser(CacheContextParser { inner, .. }) => {
                inner.collect_stats(stats);
                *stats.active_parser_type_counts.entry("CacheContextParser".to_string()).or_insert(0) += 1;
            }
            Parser::CachedParser(CachedParser { .. }) => {
                *stats.active_parser_type_counts.entry("CachedParser".to_string()).or_insert(0) += 1;
            }
            Parser::IndentCombinatorParser(IndentCombinatorParser::DentParser(parser)) => {
                parser.collect_stats(stats);
                *stats.active_parser_type_counts.entry("IndentCombinatorParser::DentParser".to_string()).or_insert(0) += 1;
            }
            Parser::IndentCombinatorParser(IndentCombinatorParser::IndentParser(_)) => {
                *stats.active_parser_type_counts.entry("IndentCombinatorParser::IndentParser".to_string()).or_insert(0) += 1;
            }
            Parser::IndentCombinatorParser(IndentCombinatorParser::Done) => {
                *stats.active_parser_type_counts.entry("IndentCombinatorParser::Done".to_string()).or_insert(0) += 1;
            }
            Parser::FrameStackOpParser(FrameStackOpParser { a, .. }) => {
                a.collect_stats(stats);
                *stats.active_parser_type_counts.entry("FrameStackOpParser".to_string()).or_insert(0) += 1;
            }
            Parser::MutateRightDataParser(_) => {
                *stats.active_parser_type_counts.entry("MutateRightDataParser".to_string()).or_insert(0) += 1;
            }
            Parser::Repeat1Parser(Repeat1Parser { a, .. }) => {
                a.collect_stats(stats);
                *stats.active_parser_type_counts.entry("Repeat1Parser".to_string()).or_insert(0) += 1;
            }
            Parser::SymbolParser(SymbolParser { inner, .. }) => {
                inner.collect_stats(stats);
                *stats.active_parser_type_counts.entry("SymbolParser".to_string()).or_insert(0) += 1;
            }
            Parser::TaggedParser(TaggedParser { inner, tag, .. }) => {
                inner.collect_stats(stats);
                *stats.active_tags.entry(tag.clone()).or_insert(0) += 1;
                *stats.active_parser_type_counts.entry("TaggedParser".to_string()).or_insert(0) += 1;
            }
            Parser::WithNewFrameParser(WithNewFrameParser { a, .. }) => {
                if let Some(a) = a {
                    a.collect_stats(stats);
                }
                *stats.active_parser_type_counts.entry("WithNewFrameParser".to_string()).or_insert(0) += 1;
            }
        }
    }
}
