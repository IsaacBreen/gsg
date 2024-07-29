use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::ops::AddAssign;
use std::rc::Rc;
use crate::{CacheContext, CacheContextParser, Cached, CachedParser, Choice, ChoiceParser, EatString, EatStringParser, EatU8, EatU8Parser, Eps, EpsParser, Fail, FailParser, ForbidFollows, ForbidFollowsCheckNot, ForbidFollowsClear, ForwardRef, FrameStackOp, FrameStackOpParser, IndentCombinator, IndentCombinatorParser, MutateRightData, MutateRightDataParser, ParseResults, Repeat1, Repeat1Parser, RightData, Seq, SeqParser, Symbol, SymbolParser, Tagged, TaggedParser, U8Set, WithNewFrame, WithNewFrameParser, EatByteStringChoice, EatByteStringChoiceParser, TrieNode};

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
    ForbidFollowsCheckNot,
    EatByteStringChoice
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
    WithNewFrameParser,
    EatByteStringChoiceParser
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
            ForbidFollowsCheckNot,
            EatByteStringChoice
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
            WithNewFrameParser,
            EatByteStringChoiceParser
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

impl Combinator {
    pub fn type_name(&self) -> String {
        match_combinator!(self, inner => std::any::type_name_of_val(&inner)).to_string()
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
            Parser::SeqParser(SeqParser { children, .. }) => {
                children.iter().for_each(|(_, parsers)| {
                    parsers.iter().for_each(|p| p.collect_stats(stats));
                });
            }
            Parser::ChoiceParser(ChoiceParser { parsers }) => {
                parsers.iter().for_each(|p| p.collect_stats(stats));
            }
            Parser::EatU8Parser(EatU8Parser { u8set, .. }) => {
                stats.active_u8_matchers.entry(u8set.clone()).or_default().add_assign(1);
            }
            Parser::EatStringParser(EatStringParser { string, .. }) => {
                stats.active_string_matchers.entry(String::from_utf8_lossy(string).to_string()).or_default().add_assign(1);
            }
            Parser::CacheContextParser(CacheContextParser { inner, cache_data_inner, .. }) => {
                inner.collect_stats(stats);
                for entry in cache_data_inner.borrow().entries.iter() {
                    entry.borrow().parser.as_ref().map(|p| p.collect_stats(stats));
                }
            }
            Parser::FrameStackOpParser(FrameStackOpParser { a: inner, .. }) |
            Parser::SymbolParser(SymbolParser { inner, .. }) => inner.collect_stats(stats),
            Parser::TaggedParser(TaggedParser { inner, tag }) => {
                inner.collect_stats(stats);
                stats.active_tags.entry(tag.clone()).or_default().add_assign(1);
            }
            Parser::Repeat1Parser(Repeat1Parser { a_parsers, .. }) => {
                a_parsers.iter().for_each(|p| p.collect_stats(stats));
            }
            Parser::WithNewFrameParser(WithNewFrameParser { a, .. }) => {
                a.as_ref().map(|a| a.collect_stats(stats));
            }
            Parser::IndentCombinatorParser(IndentCombinatorParser::DentParser(parser)) => parser.collect_stats(stats),
            _ => {}
        }
        stats.active_parser_type_counts.entry(self.type_name()).or_default().add_assign(1);
    }

    fn type_name(&self) -> String {
        match_parser!(self, inner => std::any::type_name_of_val(&inner)).to_string()
    }
}

pub enum CombinatorMatcher<'a> {
    Seq(Box<[CombinatorMatcher<'a>]>),
    Choice(Box<[CombinatorMatcher<'a>]>),
    EatU8(Box<[u8]>),
    EatString(&'a [u8]),
    Eps,
    Fail,
    ForbidFollows(Box<[CombinatorMatcher<'a>]>),
    ForbidFollowsClear(Box<[CombinatorMatcher<'a>]>),
    ForbidFollowsCheckNot(Box<[CombinatorMatcher<'a>]>),
    WithNewFrame(Box<[CombinatorMatcher<'a>]>),
    CacheContext(Box<[(CombinatorMatcher<'a>, CombinatorMatcher<'a>)]>),
    Cached(Box<[(CombinatorMatcher<'a>, CombinatorMatcher<'a>)]>),
    Repeat1(Box<[CombinatorMatcher<'a>]>),
    Symbol(Box<[(String, CombinatorMatcher<'a>)]>),
    Tagged(Box<[(String, CombinatorMatcher<'a>)]>),
    MutateRightData(Box<[(String, CombinatorMatcher<'a>)]>),
    IndentCombinator(Box<[(CombinatorMatcher<'a>, CombinatorMatcher<'a>)]>),
    EatByteStringChoice(Box<[CombinatorMatcher<'a>]>),
    FrameStackOp(Box<[(FrameStackOp, CombinatorMatcher<'a>)]>),
    ForwardRef(Box<[CombinatorMatcher<'a>]>),
}

impl<'a> CombinatorMatcher<'a> {
    pub fn to_combinator(&self) -> Combinator {
        match self {
            CombinatorMatcher::Seq(children) => Combinator::Seq(Seq { children: children.iter().map(|m| (None, m.to_combinator())).collect() }),
            CombinatorMatcher::Choice(children) => Combinator::Choice(Choice { children: children.iter().map(|m| m.to_combinator()).collect() }),
            CombinatorMatcher::EatU8(chars) => Combinator::EatU8(EatU8 { u8set: U8Set::from_slice(chars) }),
            CombinatorMatcher::EatString(string) => Combinator::EatString(EatString { string: string.to_vec() }),
            CombinatorMatcher::Eps => Combinator::Eps(Eps),
            CombinatorMatcher::Fail => Combinator::Fail(Fail),
            CombinatorMatcher::ForbidFollows(children) => Combinator::ForbidFollows(ForbidFollows { parsers: children.iter().map(|m| m.to_combinator()).collect() }),
            CombinatorMatcher::ForbidFollowsClear(children) => Combinator::ForbidFollowsClear(ForbidFollowsClear { parsers: children.iter().map(|m| m.to_combinator()).collect() }),
            CombinatorMatcher::ForbidFollowsCheckNot(children) => Combinator::ForbidFollowsCheckNot(ForbidFollowsCheckNot { parsers: children.iter().map(|m| m.to_combinator()).collect() }),
            CombinatorMatcher::WithNewFrame(children) => Combinator::WithNewFrame(WithNewFrame { parsers: children.iter().map(|m| m.to_combinator()).collect() }),
            CombinatorMatcher::CacheContext(pairs) => Combinator::CacheContext(CacheContext { pairs: pairs.iter().map(|(k, v)| (k.to_combinator(), v.to_combinator())).collect() }),
            CombinatorMatcher::Cached(pairs) => Combinator::Cached(Cached { pairs: pairs.iter().map(|(k, v)| (k.to_combinator(), v.to_combinator())).collect() }),
            CombinatorMatcher::Repeat1(children) => Combinator::Repeat1(Repeat1 { parsers: children.iter().map(|m| m.to_combinator()).collect() }),
            CombinatorMatcher::Symbol(pairs) => Combinator::Symbol(Symbol { pairs: pairs.iter().map(|(s, m)| (s.clone(), m.to_combinator())).collect() }),
            CombinatorMatcher::Tagged(pairs) => Combinator::Tagged(Tagged { pairs: pairs.iter().map(|(s, m)| (s.clone(), m.to_combinator())).collect() }),
            CombinatorMatcher::MutateRightData(pairs) => Combinator::MutateRightData(MutateRightData { pairs: pairs.iter().map(|(s, m)| (s.clone(), m.to_combinator())).collect() }),
            CombinatorMatcher::IndentCombinator(pairs) => Combinator::IndentCombinator(IndentCombinator { pairs: pairs.iter().map(|(k, v)| (k.to_combinator(), v.to_combinator())).collect() }),
            CombinatorMatcher::EatByteStringChoice(children) => Combinator::EatByteStringChoice(EatByteStringChoice { parsers: children.iter().map(|m| m.to_combinator()).collect() }),
            CombinatorMatcher::FrameStackOp(pairs) => Combinator::FrameStackOp(FrameStackOp { pairs: pairs.iter().map(|(op, m)| (op.clone(), m.to_combinator())).collect() }),
            CombinatorMatcher::ForwardRef(children) => Combinator::ForwardRef(ForwardRef { parsers: children.iter().map(|m| m.to_combinator()).collect() }),
        }
    }
}